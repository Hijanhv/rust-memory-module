//! `memscope uaf | overflow | race`: the three Lesson 2 bugs, each followed by
//! the safe Rust version that stops it.
//!
//! The unsafe parts here are UNDEFINED BEHAVIOUR on purpose. They are kept
//! tiny and bounded so the demo stays readable, but their output is never
//! guaranteed: it can change between machines, runs and build modes.

use crate::tracker::{describe, trace};
use std::ptr::addr_of_mut;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, Mutex};
use std::thread;

// ---------- use-after-free ----------

pub fn use_after_free() {
    println!("USE-AFTER-FREE  (temporal bug: right address, dead object)\n");

    let ((stale, fresh, same_spot), events) = trace(|| unsafe {
        let raw: *mut u64 = Box::into_raw(Box::new(5)); // Rust stops tracking this memory
        drop(Box::from_raw(raw)); // free it
        let y = Box::new(1234u64); // the allocator may hand out the same spot
        let stale = *raw; // UB: reading freed memory
        (stale, *y, std::ptr::eq(raw, &*y))
    });

    println!("Heap events while it ran:");
    for e in &events {
        println!("    {}", describe(e));
    }
    println!("\nThe old pointer was supposed to hold 5. Reading it gave: {stale}");
    if events.is_empty() {
        println!("Zero heap events: the optimiser deleted the allocations completely. It may");
        println!("assume your code has no undefined behaviour, so it rewrote it freely.");
        println!("Same source, different result in debug vs --release: that IS undefined behaviour.");
    } else if same_spot {
        println!("The new Box (value {fresh}) got the SAME address, so the old pointer reads its data.");
    } else {
        println!("This time the allocator picked a different spot, so you saw leftover bytes instead.");
    }
    println!("No crash, no warning. That's why use-after-free is dangerous.\n");

    println!("SAFE RUST: the same idea does not compile.");
    println!("    let x = Box::new(5);");
    println!("    drop(x);");
    println!("    println!(\"{{x}}\");   // error[E0382]: borrow of moved value: `x`");
}

// ---------- buffer overflow ----------

/// A fixed layout so the demo shows the same bytes every time. On a real
/// stack the compiler decides the layout, and the padding has no name.
#[repr(C)]
pub struct Frame {
    pub buffer: [u8; 5],
    pub padding: [u8; 3], // filler so `neighbour` starts on an 8-byte boundary
    pub neighbour: u64,
}

pub const START: u64 = 56789;

impl Frame {
    pub fn new() -> Self {
        Frame { buffer: [0; 5], padding: [0; 3], neighbour: START }
    }

    pub fn bytes(&self) -> [u8; 16] {
        // SAFETY: Frame is repr(C), exactly 16 bytes, with every byte initialised
        unsafe { std::mem::transmute_copy(self) }
    }
}

/// Write `writes` bytes starting at buffer[0], with no bounds check.
/// Capped at 16 so we never leave the struct and smash the real stack.
pub fn overflow(frame: &mut Frame, writes: usize) {
    let writes = writes.min(16);
    let base = frame as *mut Frame as *mut u8; // pointer to the whole Frame, so the writes stay inside it
    for i in 0..writes {
        // in the lesson this is buffer.as_mut_ptr().add(i): same address, no bounds check
        unsafe { *base.add(i) = i as u8 };
    }
}

fn dump(label: &str, b: &[u8; 16]) {
    let hex: Vec<String> = b.iter().map(|x| format!("{x:02X}")).collect();
    println!("  {label:<7} [{}] [{}] [{}]", hex[0..5].join(" "), hex[5..8].join(" "), hex[8..16].join(" "));
}

pub fn buffer_overflow(writes: usize) {
    println!("BUFFER OVERFLOW  (spatial bug: writing outside the object)\n");
    let mut f = Frame::new();
    println!("  bytes:  [buffer: 5 bytes] [padding: 3] [neighbour: u64, little-endian]");
    dump("before", &f.bytes());
    overflow(&mut f, writes);
    dump("after", &f.bytes());
    println!("\n  wrote {} bytes into a 5-byte buffer", writes.min(16));
    println!("  neighbour: {} -> {}", START, f.neighbour);
    match writes {
        0..=5 => println!("  Everything stayed inside the buffer. No bug."),
        6..=8 => println!("  Out of bounds, but only padding was hit. Looks fine, still undefined behaviour."),
        _ => println!("  Out of bounds and the neighbour is corrupted."),
    }
    if writes > 16 {
        println!("  (capped at 16 so the demo doesn't smash the real stack and crash)");
    }

    println!("\nSAFE RUST: indexing is bounds checked.");
    let mut buffer = [0u8; 5];
    for i in 0..writes.max(6) {
        match buffer.get_mut(i) {
            Some(slot) => *slot = i as u8,
            None => {
                println!("    buffer[{i}] = ...  ->  refused: index out of bounds (buffer[{i}] would panic)");
                break;
            }
        }
    }
}

// ---------- data race ----------

static mut COUNTER: usize = 0;

/// Run `threads` threads that each add 1 to an unsynchronised static `per_thread`
/// times. Returns the final count. UB: the result is unpredictable.
pub fn racy_count(threads: usize, per_thread: usize) -> usize {
    unsafe { *addr_of_mut!(COUNTER) = 0 };
    let handles: Vec<_> = (0..threads)
        .map(|_| {
            thread::spawn(move || {
                for _ in 0..per_thread {
                    // the three steps of `COUNTER += 1`, written out.
                    // volatile stops the optimiser merging the loop into one big add.
                    unsafe {
                        let p = addr_of_mut!(COUNTER);
                        let old = p.read_volatile(); // 1. read
                        let new = old + 1; //          2. add
                        p.write_volatile(new); //      3. write
                    }
                }
            })
        })
        .collect();
    for h in handles {
        h.join().unwrap();
    }
    unsafe { *addr_of_mut!(COUNTER) }
}

pub fn atomic_count(threads: usize, per_thread: usize) -> usize {
    let n = Arc::new(AtomicUsize::new(0));
    let handles: Vec<_> = (0..threads)
        .map(|_| {
            let n = Arc::clone(&n);
            thread::spawn(move || {
                for _ in 0..per_thread {
                    n.fetch_add(1, Ordering::Relaxed); // read + add + write as one step
                }
            })
        })
        .collect();
    for h in handles {
        h.join().unwrap();
    }
    n.load(Ordering::Relaxed)
}

pub fn mutex_count(threads: usize, per_thread: usize) -> usize {
    let n = Arc::new(Mutex::new(0));
    let handles: Vec<_> = (0..threads)
        .map(|_| {
            let n = Arc::clone(&n);
            thread::spawn(move || {
                for _ in 0..per_thread {
                    *n.lock().unwrap() += 1; // only one thread holds the lock at a time
                }
            })
        })
        .collect();
    for h in handles {
        h.join().unwrap();
    }
    let total = *n.lock().unwrap();
    total
}

pub fn data_race() {
    let (threads, per) = (8, 10_000);
    let expected = threads * per;
    println!("DATA RACE  (thread bug: unsynchronised writes to shared memory)\n");
    println!("{threads} threads each add 1, {per} times. Correct answer: {expected}\n");
    for round in 1..=5 {
        let got = racy_count(threads, per);
        println!("  static mut, round {round}: {got:>6}   ({} updates lost)", expected - got);
    }
    println!("\nSAFE RUST:");
    println!("  AtomicUsize:      {:>6}", atomic_count(threads, per));
    println!("  Arc<Mutex<_>>:    {:>6}", mutex_count(threads, per));
    println!("\nAnd sharing a plain `let mut count` with threads doesn't compile at all:");
    println!("    error[E0373]: closure may outlive the current function, but it borrows `count`");
}

#[cfg(test)]
mod tests {
    use super::*;

    fn after(writes: usize) -> u64 {
        let mut f = Frame::new();
        overflow(&mut f, writes);
        f.neighbour
    }

    #[test]
    fn frame_layout_is_buffer_padding_neighbour() {
        assert_eq!(std::mem::size_of::<Frame>(), 16);
        assert_eq!(std::mem::offset_of!(Frame, neighbour), 8);
    }

    #[test]
    fn overflow_hits_padding_first_then_the_neighbour() {
        assert_eq!(after(5), START, "in bounds");
        assert_eq!(after(8), START, "only padding touched");
        assert_eq!(after(9), 0xDD08, "lowest byte replaced (little-endian)");
        assert_eq!(after(12), 0x0B0A_0908);
        assert_eq!(after(100), u64::from_le_bytes([8, 9, 10, 11, 12, 13, 14, 15]), "capped at 16");
    }

    #[test]
    fn safe_counters_are_always_exact() {
        assert_eq!(atomic_count(8, 5_000), 40_000);
        assert_eq!(mutex_count(8, 5_000), 40_000);
    }

    #[test]
    fn racy_counter_never_gains_updates() {
        // it can lose updates, but each write is an old value + 1, so it can't overshoot
        assert!(racy_count(4, 1_000) <= 4_000);
    }
}

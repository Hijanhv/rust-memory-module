// Lesson 2: the same counter, written in SAFE Rust. Always 8000.
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, Mutex};
use std::thread;

static ATOMIC: AtomicUsize = AtomicUsize::new(0); // fix 1: an atomic static

fn main() {
    let locked = Arc::new(Mutex::new(0usize)); // fix 2: shared ownership + a lock

    let handles: Vec<_> = (0..8)
        .map(|_| {
            let locked = Arc::clone(&locked);
            thread::spawn(move || {
                for _ in 0..1000 {
                    ATOMIC.fetch_add(1, Ordering::Relaxed); // one indivisible step
                    *locked.lock().unwrap() += 1; // only one thread at a time
                }
            })
        })
        .collect();
    for h in handles { h.join().unwrap(); }

    println!("atomic: {}", ATOMIC.load(Ordering::Relaxed));
    println!("mutex : {}", *locked.lock().unwrap());
}

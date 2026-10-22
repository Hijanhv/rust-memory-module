//! A global allocator that wraps the system one and records heap events.
//!
//! Every Box, Vec and String asks the global allocator for memory. By putting
//! our own allocator in the middle we can watch each request happen.
//!
//! We can't print inside `alloc` (printing allocates, which would call `alloc`
//! again forever), so events go into a fixed-size buffer and get printed later.
//! Recording is per thread, so other threads' allocations don't show up.

use std::alloc::{GlobalAlloc, Layout, System};
use std::cell::{Cell, RefCell};

pub struct Tracker;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Event {
    Alloc { addr: usize, size: usize },
    Realloc { old: usize, new: usize, old_size: usize, new_size: usize },
    Free { addr: usize, size: usize },
}

const MAX_EVENTS: usize = 256;

thread_local! {
    // const-initialised and free of Drop, so touching these never allocates
    static RECORDING: Cell<bool> = const { Cell::new(false) };
    static EVENTS: RefCell<[Option<Event>; MAX_EVENTS]> = const { RefCell::new([None; MAX_EVENTS]) };
    static COUNT: Cell<usize> = const { Cell::new(0) };
}

fn record(e: Event) {
    // try_with: during thread shutdown the thread-locals may already be gone
    let _ = RECORDING.try_with(|on| {
        if !on.get() {
            return;
        }
        let n = COUNT.get();
        if n < MAX_EVENTS {
            EVENTS.with(|ev| ev.borrow_mut()[n] = Some(e));
            COUNT.set(n + 1);
        }
    });
}

unsafe impl GlobalAlloc for Tracker {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let p = System.alloc(layout);
        record(Event::Alloc { addr: p as usize, size: layout.size() });
        p
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        record(Event::Free { addr: ptr as usize, size: layout.size() });
        System.dealloc(ptr, layout);
    }

    unsafe fn realloc(&self, ptr: *mut u8, layout: Layout, new_size: usize) -> *mut u8 {
        let p = System.realloc(ptr, layout, new_size);
        record(Event::Realloc { old: ptr as usize, new: p as usize, old_size: layout.size(), new_size });
        p
    }
}

/// Run `f` while recording this thread's heap events, and return them.
pub fn trace<R>(f: impl FnOnce() -> R) -> (R, Vec<Event>) {
    COUNT.set(0);
    RECORDING.set(true);
    let out = f();
    RECORDING.set(false);
    // collecting into a Vec allocates, which is fine now that recording is off
    let events = EVENTS.with(|ev| ev.borrow()[..COUNT.get()].iter().flatten().copied().collect());
    (out, events)
}

pub fn describe(e: &Event) -> String {
    match *e {
        Event::Alloc { addr, size } => format!("alloc   {size:>4} bytes  at {addr:#x}"),
        Event::Free { addr, size } => format!("free    {size:>4} bytes  at {addr:#x}"),
        Event::Realloc { old, new, old_size, new_size } => {
            let moved = if old == new { "grew in place" } else { "MOVED, old block freed" };
            format!("realloc {old_size:>4} -> {new_size} bytes  {old:#x} -> {new:#x}  ({moved})")
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn stack_values_never_touch_the_heap() {
        let ((), ev) = trace(|| {
            let x = std::hint::black_box([0u64; 8]);
            std::hint::black_box(&x);
        });
        assert!(ev.is_empty(), "{ev:?}");
    }

    #[test]
    fn a_box_is_allocated_then_freed_at_the_same_address() {
        let ((), ev) = trace(|| {
            std::hint::black_box(Box::new(5u64));
        });
        match ev.as_slice() {
            [Event::Alloc { addr: a, size: 8 }, Event::Free { addr: f, size: 8 }] => assert_eq!(a, f),
            other => panic!("unexpected events {other:?}"),
        }
    }

    #[test]
    fn every_byte_allocated_is_freed_when_owners_go_out_of_scope() {
        let ((), ev) = trace(|| {
            let mut v = Vec::new();
            for i in 0..100u32 {
                v.push(i);
            }
            let s = String::from("hello") + " world";
            std::hint::black_box((&v, &s));
        });
        let mut live: isize = 0;
        for e in &ev {
            match *e {
                Event::Alloc { size, .. } => live += size as isize,
                Event::Free { size, .. } => live -= size as isize,
                Event::Realloc { old_size, new_size, .. } => live += new_size as isize - old_size as isize,
            }
        }
        assert_eq!(live, 0, "leak or double free: {ev:?}");
    }
}

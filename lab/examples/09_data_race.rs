// Lesson 2: data race (UNDEFINED BEHAVIOUR, on purpose).
// Loops until two threads trample each other's increment.
// Run it a few times: the number of lost updates changes every time.
#![allow(static_mut_refs)]

use std::thread;

static mut COUNTER: usize = 0;

fn main() {
    for round in 1.. {
        unsafe { COUNTER = 0; }
        let handles: Vec<_> = (0..8)
            .map(|_| thread::spawn(|| {
                for _ in 0..1000 {
                    unsafe { COUNTER += 1; } // READ, ADD, WRITE: not atomic
                }
            }))
            .collect();
        for h in handles { h.join().unwrap(); }

        let got = unsafe { COUNTER };
        if got != 8000 {
            println!("round {round}: expected 8000, got {got}. {} updates lost.", 8000 - got);
            break;
        }
    }
}

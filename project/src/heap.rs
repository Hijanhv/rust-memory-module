//! `memscope heap`: watch the allocator while we use Box, Vec and String.

use crate::tracker::{describe, trace};

/// (len, capacity, did the buffer move?) after each push into an empty Vec.
pub fn vec_growth(pushes: usize) -> Vec<(usize, usize, bool)> {
    let mut v: Vec<u32> = Vec::new();
    let mut last = v.as_ptr();
    let mut out = Vec::with_capacity(pushes);
    for i in 0..pushes {
        v.push(i as u32);
        out.push((v.len(), v.capacity(), v.as_ptr() != last));
        last = v.as_ptr();
    }
    out
}

fn show(title: &str, f: impl FnOnce()) {
    let ((), events) = trace(f);
    println!("{title}");
    if events.is_empty() {
        println!("    (no heap activity)");
    }
    for e in &events {
        println!("    {}", describe(e));
    }
    println!();
}

pub fn run() {
    println!("Every line below is a real request to the heap allocator.\n");

    show("let x = 5;  (stack only)", || {
        std::hint::black_box(5);
    });
    show("Box::new(5u32), then it goes out of scope", || {
        let b = Box::new(5u32);
        std::hint::black_box(&b);
    });
    show("Vec::new() and 9 pushes, then it goes out of scope", || {
        let mut v: Vec<u32> = Vec::new();
        for i in 0..9 {
            v.push(i);
        }
        std::hint::black_box(&v);
    });
    show("Vec::with_capacity(9) and 9 pushes (reserve once)", || {
        let mut v: Vec<u32> = Vec::with_capacity(9);
        for i in 0..9 {
            v.push(i);
        }
        std::hint::black_box(&v);
    });
    show("let s = \"hi\";  (a literal: static, no heap)", || {
        let s: &str = std::hint::black_box("hi");
        std::hint::black_box(s);
    });
    show("String::from(\"hi\"), then push_str(\" there\")", || {
        let mut s = String::from("hi");
        s.push_str(" there");
        std::hint::black_box(&s);
    });

    println!("Vec<u32> growth: len / capacity after each push");
    let mut prev = 0;
    for (len, cap, moved) in vec_growth(17) {
        let note = if moved { "   <- grew, buffer moved" } else if len > 1 && cap != prev { "   <- grew in place" } else { "" };
        println!("    len {len:>2}  cap {cap:>2}{note}");
        prev = cap;
    }
}

#[cfg(test)]
mod tests {
    use super::vec_growth;

    #[test]
    fn capacity_doubles_and_buffer_never_moves_without_growing() {
        let g = vec_growth(17);
        let caps: Vec<usize> = g.iter().map(|x| x.1).collect();
        assert_eq!(caps, [4, 4, 4, 4, 8, 8, 8, 8, 16, 16, 16, 16, 16, 16, 16, 16, 32]);
        let mut prev_cap = 0;
        for (len, cap, moved) in g {
            assert!(len <= cap);
            // growing MAY move the buffer (the allocator can sometimes extend it
            // in place), but a push that fits must never move it
            if cap == prev_cap {
                assert!(!moved, "moved without growing at len {len}");
            }
            prev_cap = cap;
        }
    }
}

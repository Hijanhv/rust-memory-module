// Lesson 1: the heap. A Vec is 3 words on the stack (ptr, len, capacity)
// pointing at a buffer on the heap. When len would pass capacity, Vec
// allocates a bigger buffer, copies everything over, and frees the old one.

fn main() {
    let b = Box::new(5);
    println!("Box: stack slot {:p} -> heap value {:p} = {}", &b, &*b, b);

    let mut v: Vec<i32> = Vec::new();
    let mut last_ptr = v.as_ptr();
    println!("\nempty Vec: len={} cap={} ptr={:p} (no heap allocation yet)", v.len(), v.capacity(), last_ptr);

    for i in 0..20 {
        v.push(i);
        let moved = if v.as_ptr() != last_ptr { "  <- new heap buffer, data copied" } else { "" };
        println!("push {i:2}: len={:2} cap={:2} ptr={:p}{moved}", v.len(), v.capacity(), v.as_ptr());
        last_ptr = v.as_ptr();
    }

    let mut w = Vec::with_capacity(20); // reserve once, no reallocation later
    let start = w.as_ptr();
    for i in 0..20 { w.push(i); }
    println!("\nwith_capacity(20): pointer unchanged after 20 pushes? {}", start == w.as_ptr());
} // v, w and b are dropped here: their heap memory is freed automatically

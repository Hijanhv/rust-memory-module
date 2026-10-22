// Lesson 1: each function call pushes a new stack frame.
// Watch the address of a local variable go DOWN as calls go deeper.

fn depth(level: u32) {
    let local = level; // lives in this call's frame
    println!("level {level}: &local = {:p}", &local);
    if level < 4 {
        depth(level + 1);
    }
    // when we return here, this frame is popped
}

fn main() {
    let a: i32 = 1;
    let b: i32 = 2;
    // Two i32 locals in the same frame are usually 4 bytes apart
    // (the compiler is allowed to reorder them).
    println!("&a = {:p}, &b = {:p}", &a, &b);
    depth(0);
}

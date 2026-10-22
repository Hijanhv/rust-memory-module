// Lesson 2: use-after-free (UNDEFINED BEHAVIOUR, on purpose).
// Run it several times and compare the output.

fn main() {
    let x = Box::new(5);
    let raw_x = Box::into_raw(x); // Box -> raw pointer, Rust stops tracking it

    unsafe {
        drop(Box::from_raw(raw_x)); // take ownership back and free the heap memory

        // Reuse the freed spot so the bug becomes visible.
        let y = Box::new(1234);
        println!("new Box lives at {:p}, old pointer is {:p}", &*y, raw_x);

        // UB: reading memory we already freed
        println!("value after free: {}", *raw_x);
    }
}

// Lesson 2: buffer overflow (UNDEFINED BEHAVIOUR, on purpose).
// Try changing WRITES to 7, 10, 64... and run in debug and --release.
// Safe Rust would never let this compile or run silently.

const WRITES: usize = 6;

fn main() {
    let mut buffer = [0u8; 5];
    let not_in_buffer: u64 = 56789;

    println!("&buffer        = {:p}", &buffer);
    println!("&not_in_buffer = {:p}", &not_in_buffer);

    unsafe {
        let ptr = buffer.as_mut_ptr();
        for i in 0..WRITES {
            *ptr.add(i) = i as u8; // UB once i >= 5
        }
    }

    println!("buffer: {:?}", buffer);
    println!("not_in_buffer: {} (was 56789)", not_in_buffer);

    // Safe Rust: the same mistake is caught at runtime with a clean panic.
    // let i = WRITES - 1; buffer[i] = 0;   // panics: index out of bounds
}

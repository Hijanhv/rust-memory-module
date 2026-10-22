// Lesson 1: the stack is small. Spawned threads get 2 MiB by default.
// A 4 MiB array cannot fit, so this crashes with "has overflowed its stack".
// The same data in a Vec (heap) is fine.

fn main() {
    let ok = std::thread::spawn(|| {
        let big = vec![0u8; 4 * 1024 * 1024]; // heap: fine
        big.len()
    });
    println!("heap version worked: {} bytes", ok.join().unwrap());

    let bad = std::thread::spawn(|| {
        let big = [1u8; 4 * 1024 * 1024]; // stack: too big for a 2 MiB stack
        std::hint::black_box(&big); // stop the compiler optimising it away
        big[0]
    });
    println!("stack version: {:?}", bad.join());
}

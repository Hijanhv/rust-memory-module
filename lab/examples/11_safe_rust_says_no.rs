// Preview of Lesson 3: safe Rust refuses to compile each bug from Lesson 2.
// Uncomment one block at a time and read the compiler error.

fn main() {
    // 1) use-after-free -> error[E0382]: borrow of moved value
    // let x = Box::new(5);
    // drop(x);
    // println!("{}", x);

    // 2) dangling reference -> error[E0106]/[E0515]: returns a reference to a local
    // fn dangle() -> &i32 { let n = 5; &n }

    // 3) data race -> error[E0499]/[E0373]: closure may outlive the borrowed value
    // let mut count = 0;
    // let t = std::thread::spawn(|| count += 1);
    // t.join().unwrap();

    // 4) buffer overflow -> compiles, but PANICS instead of corrupting memory
    // let buf = [0u8; 5];
    // let i = std::hint::black_box(5);
    // println!("{}", buf[i]);

    println!("Uncomment a block to see Rust stop the bug.");
}

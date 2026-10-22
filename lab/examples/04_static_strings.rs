// Lesson 1: static memory and the two kinds of strings.

static GREETING_OUTER: &str = "Hello, universe!";

fn hello() -> &'static str {
    static GREETING_INNER: &str = "Hello, world!"; // name only visible in here
    GREETING_INNER // but the data lives for the whole program, so returning it is fine
}

fn main() {
    let a: &str = "same text";
    let b: &str = "same text";
    // Identical literals are usually stored once, so both point to the same bytes.
    println!("a -> {:p}, b -> {:p}", a.as_ptr(), b.as_ptr());

    // String::from copies the static bytes into a NEW heap buffer.
    let owned: String = String::from(a);
    println!("String copy -> {:p} (heap, different address)", owned.as_ptr());

    println!("{GREETING_OUTER} / {}", hello());
}

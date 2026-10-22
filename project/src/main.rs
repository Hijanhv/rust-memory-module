//! memscope: see how a Rust process uses memory, and what memory bugs do to it.
//! Module 1 project covering Lesson 1 (memory) and Lesson 2 (memory bugs).

mod bugs;
mod heap;
mod map;
mod page;
mod sizes;
mod stack;
mod tracker;

#[global_allocator]
static ALLOC: tracker::Tracker = tracker::Tracker;

const HELP: &str = "memscope: see how a Rust process uses memory

USAGE: cargo run -- <command>

Lesson 1: memory
  map              real address of every region, drawn as a memory map
  stack [depth]    nested calls get lower addresses (default depth 5)
  heap             every alloc / realloc / free made by Box, Vec and String
  sizes            bytes each type takes on the stack (thin vs fat pointers)

Lesson 2: memory bugs (undefined behaviour on purpose)
  uaf              use-after-free: a freed Box's pointer reads new data
  overflow [n]     write n bytes into a 5-byte buffer (default 9, max 16)
  race             unsynchronised counter loses updates, safe versions don't

  all              run everything in order";

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let num = |default: usize| args.get(1).and_then(|s| s.parse().ok()).unwrap_or(default);
    match args.first().map(String::as_str) {
        Some("map") => map::run(),
        Some("stack") => stack::run(num(5)),
        Some("heap") => heap::run(),
        Some("sizes") => sizes::run(),
        Some("uaf") => bugs::use_after_free(),
        Some("overflow") => bugs::buffer_overflow(num(9)),
        Some("race") => bugs::data_race(),
        Some("all") => {
            let line = "=".repeat(64);
            let parts: [(&str, fn()); 7] = [
                ("map", map::run),
                ("stack", || stack::run(5)),
                ("heap", heap::run),
                ("sizes", sizes::run),
                ("uaf", bugs::use_after_free),
                ("overflow", || bugs::buffer_overflow(9)),
                ("race", bugs::data_race),
            ];
            for (name, f) in parts {
                println!("{line}\n memscope {name}\n{line}\n");
                f();
                println!();
            }
        }
        _ => println!("{HELP}"),
    }
}

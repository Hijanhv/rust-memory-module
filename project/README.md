# memscope

The Module 1 project. A small command-line tool, with no dependencies, that shows Lesson 1 (memory) and Lesson 2 (memory bugs) happening inside a real Rust process.

```sh
cd project
cargo run -- map        # or: stack, heap, sizes, uaf, overflow 9, race, all
cargo test
```

| Command | Lesson | What you see |
| --- | --- | --- |
| `map` | 1.1, 1.2 | One real address per region (text, rodata, data, bss, heap, stack), drawn as a map with page numbers |
| `stack [depth]` | 1.3 | Each nested call's frame sits lower in memory |
| `heap` | 1.4, 1.5 | Every alloc, realloc and free made by Box, Vec and String, caught by a custom global allocator |
| `sizes` | 1.6 | Thin vs fat pointers |
| `uaf` | 2.2 | A freed Box's pointer reading the next Box's value |
| `overflow [n]` | 2.3 | A byte dump of a 5-byte buffer overflowing into padding, then its neighbour |
| `race` | 2.4 | An unsynchronised counter losing updates, next to exact Atomic and Mutex versions |

## How it works

- `src/tracker.rs` is a `#[global_allocator]` that wraps the system allocator. It can't print from inside `alloc` (printing allocates), so it saves events into a fixed per-thread buffer and prints them afterwards.
- `src/page.rs` calls the C function `getpagesize` through `extern "C"`, which is one of unsafe Rust's five superpowers.
- `src/bugs.rs` has the undefined behaviour. The overflow writes into a `#[repr(C)]` struct, so you see the same bytes on every run, and it's capped at 16 bytes so it never smashes the real stack.

## Things it found that the lessons don't mention

- **UB changes with build mode.** `cargo run -- uaf` reads `1234` (the reused block). `cargo run --release -- uaf` reads `1` and records zero heap events, because the optimiser deleted the allocations. It's allowed to assume there is no UB.
- **Only the main thread's stack is at the top.** Tests run on spawned threads, and their stacks can be mapped below the heap.
- **Growing a Vec doesn't always move it.** The allocator can sometimes extend a block in place. A push that fits never moves it.

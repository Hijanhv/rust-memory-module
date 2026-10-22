# Rust Memory, Step by Step

A beginner-friendly learning site for Module 1 (Memory) of my Rust course: Lesson 1 (virtual memory, process layout, stack, heap, static) and Lesson 2 (memory bugs in unsafe Rust: use-after-free, buffer overflow, data race).

Live site: https://rust-memory-module.vercel.app

## What's inside

- `index.html` and `lessons/`: the site. Plain HTML, CSS and JS, no build step.
  - **0.1 to 0.5 Rust Book essentials**: the Rust Book chapters these lessons depend on (ch. 3, 4, 8, 15, 16, 20.1), explained simply.
  - **1.1 to 1.6 Lesson 1: Memory**, with click-through stack/heap/static diagrams.
  - **2.1 to 2.4 Lesson 2: Memory bugs**, including an interactive byte-level buffer overflow.
  - **Mastery test**: a "where does it live?" game and a scored quiz.
- `lab/`: runnable Rust examples, one per lesson.
- `project/`: **memscope**, the Module 1 project. A dependency-free CLI that shows every Lesson 1 and 2 idea inside a real process. See [project/README.md](project/README.md).

## Run the lab

```sh
cd lab
cargo run --example 01_addresses
```

| Example | Shows |
| --- | --- |
| `01_addresses` | One real address from code, static, heap and stack |
| `02_stack_frames` | Each nested call's frame sits lower: the stack grows down |
| `03_heap_box_vec` | Vec length vs capacity, and the pointer moving on reallocation |
| `04_static_strings` | String literals share one static address; `String` copies to the heap |
| `05_sizes` | Thin vs fat pointers (`&str`, `&[T]`, `Box<dyn Trait>`) |
| `06_stack_overflow` | A 4 MiB array overflows a 2 MiB thread stack; a Vec doesn't |
| `07_use_after_free` | UB: a freed Box's pointer reads a newer Box's value |
| `08_buffer_overflow` | UB: writing past a 5-byte buffer changes a neighbour |
| `09_data_race` | UB: unsynchronised `static mut` counter loses updates |
| `10_data_race_fixed` | The same counter with `AtomicUsize` and `Arc<Mutex>` |
| `11_safe_rust_says_no` | Uncomment each block to see safe Rust reject the bug |

Examples 07 to 09 contain undefined behaviour on purpose. Their output can differ between machines, runs and build modes.

## Run the site locally

```sh
python3 -m http.server
```

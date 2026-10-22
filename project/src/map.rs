//! `memscope map`: one real address from every region, drawn as a memory map.

use crate::page;
use std::sync::atomic::{AtomicU32, AtomicU64};

static GREETING: &str = "hello"; // the letters are read-only static data
static CONFIG: [u32; 4] = [1, 2, 3, 4]; // never changes, so it can be read-only
static HITS: AtomicU32 = AtomicU32::new(7); // writable static with a value: data segment
static ZEROED: AtomicU64 = AtomicU64::new(0); // all zeros, so it can go in BSS

pub struct Region {
    pub name: &'static str,
    pub what: &'static str,
    pub addr: usize,
}

/// Collect one address per region. Kept separate from printing so tests can check the order.
pub fn regions() -> (Vec<Region>, Box<u64>) {
    let on_stack: u64 = 7;
    let on_heap = Box::new(99u64);
    let code = regions as fn() -> (Vec<Region>, Box<u64>);
    let list = vec![
        Region { name: "stack", what: "a local variable", addr: &on_stack as *const u64 as usize },
        Region { name: "heap", what: "the value inside a Box", addr: &*on_heap as *const u64 as usize },
        Region { name: "bss", what: "static that starts as zero", addr: &ZEROED as *const AtomicU64 as usize },
        Region { name: "data", what: "writable static with a value", addr: &HITS as *const AtomicU32 as usize },
        Region { name: "rodata", what: "immutable static array", addr: CONFIG.as_ptr() as usize },
        Region { name: "rodata", what: "a string literal's bytes", addr: GREETING.as_ptr() as usize },
        Region { name: "text", what: "the code of a function", addr: code as usize },
    ];
    // the Box is returned so the heap value stays alive while we print
    (list, on_heap)
}

fn row(text: &str) {
    println!("  | {text:<58} |");
}

pub fn run() {
    let (mut list, _keep) = regions();
    list.sort_by_key(|r| std::cmp::Reverse(r.addr)); // highest address first, like the diagrams
    let ps = page::size();
    let edge = format!("  +{}+", "-".repeat(60));
    println!("Memory map of this process (page size {ps} bytes = {} KB)\n", ps / 1024);
    println!("  high addresses");
    for (i, r) in list.iter().enumerate() {
        let (pg, off) = page::split(r.addr);
        println!("{edge}");
        row(&format!("{:<7}{:<31}{:>20}", r.name, r.what, format!("{:#x}", r.addr)));
        row(&format!("{:<7}page {pg}, byte {off} of that page", ""));
        if let Some(next) = list.get(i + 1) {
            let gap = r.addr - next.addr;
            if gap > 64 * 1024 * 1024 {
                println!("{edge}");
                row(&format!("{:^58}", format!("... {} MB gap, mostly unmapped ...", gap / 1024 / 1024)));
            }
        }
    }
    println!("{edge}");
    println!("  low addresses\n");
    println!("Run it again: every address changes (ASLR), but the order stays the same.");
}

#[cfg(test)]
mod tests {
    use super::*;

    fn addr(list: &[Region], name: &str) -> usize {
        list.iter().find(|r| r.name == name).unwrap().addr
    }

    #[test]
    fn regions_follow_the_memory_map() {
        let (list, _keep) = regions();
        let text = addr(&list, "text");
        let ro = addr(&list, "rodata");
        let data = addr(&list, "data");
        let heap = addr(&list, "heap");
        assert!(text < ro, "code sits below read-only data");
        assert!(ro < data, "read-only data sits below writable data");
        assert!(data < heap, "statics sit below the heap");
        // No stack check here: tests run on spawned threads, and a spawned thread's
        // stack is mapped wherever the OS finds room, often BELOW the heap.
        // Only the main thread's stack sits at the top (see `memscope map`).
        let _ = addr(&list, "stack");
    }
}

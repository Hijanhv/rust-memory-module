//! `memscope sizes`: how many bytes each type takes on the stack.

use std::mem::size_of;

#[allow(dead_code)]
trait Speak {
    fn hi(&self);
}

pub fn table() -> Vec<(&'static str, usize, &'static str)> {
    vec![
        ("u8", size_of::<u8>(), "1 byte"),
        ("i32", size_of::<i32>(), "the default integer"),
        ("char", size_of::<char>(), "any Unicode character"),
        ("[u8; 5]", size_of::<[u8; 5]>(), "whole array on the stack"),
        ("&i32", size_of::<&i32>(), "thin pointer: address"),
        ("Box<i32>", size_of::<Box<i32>>(), "thin pointer to the heap"),
        ("&str", size_of::<&str>(), "fat pointer: address + length"),
        ("&[i32]", size_of::<&[i32]>(), "fat pointer: address + length"),
        ("Box<dyn Speak>", size_of::<Box<dyn Speak>>(), "fat pointer: address + vtable"),
        ("String", size_of::<String>(), "pointer + length + capacity"),
        ("Vec<i32>", size_of::<Vec<i32>>(), "pointer + length + capacity"),
        ("Option<Box<i32>>", size_of::<Option<Box<i32>>>(), "None uses the null address"),
    ]
}

pub fn run() {
    println!("Size on the stack (heap data not counted):\n");
    for (name, bytes, why) in table() {
        println!("  {name:<17} {bytes:>2} bytes   {why}");
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn thin_and_fat_pointer_sizes() {
        let word = std::mem::size_of::<usize>();
        let t: std::collections::HashMap<_, _> = super::table().into_iter().map(|(n, b, _)| (n, b)).collect();
        assert_eq!(t["i32"], 4);
        assert_eq!(t["[u8; 5]"], 5);
        assert_eq!(t["Box<i32>"], word);
        assert_eq!(t["&str"], 2 * word);
        assert_eq!(t["&[i32]"], 2 * word);
        assert_eq!(t["Box<dyn Speak>"], 2 * word);
        assert_eq!(t["String"], 3 * word);
        assert_eq!(t["Option<Box<i32>>"], word);
    }
}

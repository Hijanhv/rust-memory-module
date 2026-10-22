// Lesson 1: how big is the thing on the STACK? (not the heap data)
// A "fat pointer" = address + metadata (a length, or a vtable pointer).
use std::mem::size_of;

#[allow(dead_code)]
trait Speak { fn hi(&self); }

fn main() {
    println!("i32              {:2} bytes", size_of::<i32>());
    println!("[i32; 5]         {:2} bytes (whole array is on the stack)", size_of::<[i32; 5]>());
    println!("&i32             {:2} bytes (thin pointer: address only)", size_of::<&i32>());
    println!("Box<i32>         {:2} bytes (thin pointer to heap)", size_of::<Box<i32>>());
    println!("&str             {:2} bytes (fat: address + length)", size_of::<&str>());
    println!("&[i32]           {:2} bytes (fat: address + length)", size_of::<&[i32]>());
    println!("String           {:2} bytes (ptr + len + capacity)", size_of::<String>());
    println!("Vec<i32>         {:2} bytes (ptr + len + capacity)", size_of::<Vec<i32>>());
    println!("Box<dyn Speak>   {:2} bytes (fat: address + vtable)", size_of::<Box<dyn Speak>>());
    println!("Option<Box<i32>> {:2} bytes (null is never a valid Box, so None is free)", size_of::<Option<Box<i32>>>());
}

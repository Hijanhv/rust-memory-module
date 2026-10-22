// Lesson 1: where does each kind of data live?
// Prints one address from each memory region so you can see the layout.
// Run it twice: the numbers change (ASLR), but the ORDER stays the same.

static GLOBAL: i32 = 42; // static memory (read-only data)
static mut COUNTER: i32 = 0; // static memory (writable data)

fn main() {
    let on_stack: i32 = 7; // stack
    let on_heap = Box::new(99); // pointer on stack, the 99 lives on the heap
    let literal: &str = "hello"; // pointer on stack, the bytes live in static memory

    println!("text  (code)    main()   : {:p}", main as fn() as *const ());
    println!("static (ro)     GLOBAL   : {:p}", &GLOBAL);
    println!("static (ro)     \"hello\"  : {:p}", literal.as_ptr());
    println!("static (rw)     COUNTER  : {:p}", std::ptr::addr_of!(COUNTER));
    println!("heap            *on_heap : {:p}", &*on_heap);
    println!("stack           on_stack : {:p}", &on_stack);
    println!("stack           on_heap  : {:p}  (the Box itself, i.e. the pointer)", &on_heap);
}

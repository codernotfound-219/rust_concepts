mod example1;
mod example2;

/*

NOTE:
1. Allocated to the stack.
2. Immutable
3. Defined to the scope alone.

*/

fn stack_alloc() {
    let s = "HELLO FROM STACK";
    println!("{}", s);
}

/*

NOTE:
1. Allocated to the heap.
2. Mutable.
3. Defined to the scope alone.

*/

fn heap_alloc() {
    let mut s = String::from("HELLO");
    s.push_str(" FROM HEAP!");
    println!("{}", s);
    // NOTE: the memory is freed once s, exits the scope.
    // No need to manually free it :)
} 

// MAIN
fn main() {
    println!("-- Understanding Heap vs Stack");
    stack_alloc();
    heap_alloc();

    println!("-- Understanding Move and Ownership in fn");
    example1::example1();

    println!("-- Ownership in return fn");
    example2::example2();    
}

/*
OUTPUT:


-- Understanding Heap vs Stack
HELLO FROM STACK
HELLO FROM HEAP!
-- Understanding Move and Ownership in fn
HEAP ALLOCATED
5
-- Ownership in return fn

*/

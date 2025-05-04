fn move_heap_alloc() {
    let s1 = String::from("HELLO from HEAP");
    let s2 = s1;                        // NOTE: s1 is moved to s2. {s1 is now an invalidated reference
    // println!("{}", s1);              // BUG: throws an error: invalidated reference
    println!("{}", s2);
}

/*

NOTE: WORKING:
1. s1 is created. {data object: .pointer, .length, .capacity}
2. data object {s1} is sent to stack.
3. s2 is created. {data object: .pointer, .length, .capacity}   -- 2 pointers, pointing to the same address on the heap 
4. s1 is no longer valid and cannot be used.                    -- To prevent double free error.
=> s1 was moved into s2.
=> this is also known as: SHALLOW COPY                          -- 2 pointers, to the same address.
=> opposite:              DEEP COPY                             -- 2 pointers, 2 different addresses. { expensive }

*/
/*

NOTE:
double free error:
=> 2 pointers {a, b} point to the same address.
=> a is freed.
=> if b is also freed when exiting the scope : double free error.
=> may cause memory corruption.

*/

fn deep_copy() {
    let s1 = String::from("HELLO from HEAP");
    let s2 = s1.clone();                        // clone: creates a copy in the heap
    println!("s1= {}, s2= {}", s1, s2);
}

fn stack_clone() {
    let x = 5;
    let y = x;                                  // NOTE: no shallow copying happens here.
    println!("x= {}, y = {}", x, y);            // copy created on the stack. {faster}
}

fn main() {
    move_heap_alloc();
    deep_copy();
    stack_clone();
    println!("TESTING MOVE");
}


/*
OUTPUT:

HELLO from HEAP
s1= HELLO from HEAP, s2= HELLO from HEAP
x= 5, y = 5
TESTING MOVE

*/

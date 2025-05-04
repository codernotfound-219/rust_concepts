pub fn example1() {
    let s = String::from("HEAP ALLOCATED");
    take_ownership(s);                          // : ownership of s changes. {s is moved to some_string}
    
    // s is invalidated. IT IS NO LONGER ACCESSIBLE NOW.

    let stack_allocated = 5;
    make_copy(stack_allocated);                 // : a copy of "5" is created
    
    // stack_allocated is usable now too.
}


fn take_ownership(some_string: String) {
    println!("{}", some_string);
} // memory freed

fn make_copy(x: i32) {
    println!("{x}");
} // x is freed

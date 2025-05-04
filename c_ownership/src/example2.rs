pub fn example2() {
    let _s1 = gives_ownership();
    let _s2 = String::from("HELLO from HEAP 2");
    let _s3 = take_and_give_ownership(_s2);

    // _s2 is invalidated. CANNOT BE USED NOW
}
// s1 is dropped
// s2 is moved
// s3 is dropped

fn gives_ownership() -> String {
    let s = String::from("Hello from HEAP 1");
    s
} // ownership moves to _s1 { s is moved to _s1 }

fn take_and_give_ownership(some_string: String) -> String {
    some_string
} // _s2 doesnt exist anymore. Ownership moves to _s3

fn main() {
    let s1 = String::from("Hello, ");
    let s2 = String::from("World");
    let _s3 = s1 + &s2;                              // does not use new memory
    
    //NOTE:
    // The concatenation operator takes in String object + reference to a String object
    // As a result, s3 takes ownership of s1 (s1 can no longer be used)

    // FORMAT macro
    let s_1 = String::from("tic");
    let s_2 = String::from("tac");
    let s_3 = String::from("toe");

    let s = format!("{s_1}-{s_2}-{s_3}");           // does not take ownership of any param
    println!("{s}");

}

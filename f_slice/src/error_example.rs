fn main() {
    let mut s = String::from("Hello World");
    let word = first_word(&s);
    s.clear();

    println!("This is the message {word}");
}

fn first_word(s: &String) -> &str {
    let string_bytes = s.as_bytes();

    for(i, &item) in string_bytes.iter().enumerate() {   // .iter().enumerate() runs through bytes
        // println!("{} and {}", i, item);               // of string and returns a tuple: (index, ascii value[reference])

        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

/*
error[E0502]: cannot borrow `s` as mutable because it is also borrowed as immutable
 --> ./src/error_example.rs:4:5
  |
3 |     let word = first_word(&s);
  |                           -- immutable borrow occurs here
4 |     s.clear();
  |     ^^^^^^^^^ mutable borrow occurs here
5 |
6 |     println!("This is the message {word}");
  |                                   ------ immutable borrow later used here

error: aborting due to 1 previous error
*/

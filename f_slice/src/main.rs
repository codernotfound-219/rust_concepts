mod error_example;

fn main() {
    let s = String::from("Hello World");
    let _s1 = &s[0..5];      // starting index - inclusive
    let _s2 = &s[6..11];     // ending index   - exclusive 

    let _s3 = &s[..5];       // s3 = s1
    let _s4 = &s[6..];       // s4 = s2
    let _s5 = &s[..];        // s5 = s
    let s6 = first_word(&s);
    println!("{}", s6);
    


    // --- string literals --- //
    let s_literal = "Hello World";          // s_literal is nothing but &str {its a slice pointing
                                            // to the specific point of the binary (executable) } 
    // To pass the above as a parameter:
    let _s7 = first_word_optimized(s_literal);    // s_literal is a slice(reference) already so, no
                                                  // errors :)

    let _s8 = first_word_optimized(&s_literal[0..6]);   // we can also slice string literals
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

fn first_word_optimized(s: &str) -> &str {
    let string_bytes = s.as_bytes();

    for(i, &item) in string_bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }

    &s[..]
}

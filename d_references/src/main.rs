/*
fn main() {
    let s1 = String::from("Hello from HEAP");
    let (s2, len) = calculate_length(s1);

    println!("The length of '{s2}' is '{len}'.");
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len();
    (s, length)
}
*/

/*
* NOTE:
* In order to calc length we transfer ownership to the calling function (calculate_length)
* The ownership is then returned back to main() via 'return'
* CONCLUSION: ownership changes thrice.
*/

fn main() {
    let s1 = String::from("HELLO from HEAP");
    let length = calculate_length(&s1); 
    // ownership is not transferred, but calling function borrows s1
    
    println!("The length of '{s1}' is '{length}'.");
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

/*
* NOTE:
* s goes out of scope
* but String is not freed from memory
* because s does not own the string
*/

// BUG: You cannot modify the borrowed data
// references are immutable by default

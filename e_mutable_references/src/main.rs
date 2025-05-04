fn main() {
    let mut s = String::from("Hello from HEAP");
    change(&mut s);
    println!("{}", s);
}

fn change(s: &mut String) {
    s.push_str(" HEAPPPPPPP");
}

// NOTE: 
// the variable s has been borrowed as a mutable reference

/* BUG: 
*  You cannot borrow the same mutable variable twice
*  let mut s = String::from("HELO");
*  let r1 = &mut s;
*  let r2 = &mut s;
*           ^^^^^^ throws error
*/

/* BUG:
*  You cannot borrow the mutable version of a variable 
*  when the immutable one is already borrowed:
*  let mut s = String::from("HELLO");
*  let r1 = &s;
*  let r2 = &s; // This is fine
*  let r3 = &mut s;
*           ^^^^^^ Throws the error
*  println!("{}, {} and {}", r1, r2, r3);
*/

/* WARNING: Exception:
*  let mut s = String::from("Hello");
*  let r1 = &s;
*  let r2 = &s;
*  println!("{r1} and {r2}");
*  let r3 = &mut s;
*  println!("{r3}");
*/

// The above snippet does not throw an error
// because the scopes do not overlap
// (immutable references are used separately)
// (mutable reference is used separately)

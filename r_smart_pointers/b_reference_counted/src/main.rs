enum List {
    Cons(i32, Rc<List>),
    Nil
}

use crate::List::{Cons, Nil};
use std::rc::Rc;

// NOTE:
// Analogy -> many people are watching the TV
// TV is on when first person enters
// Many more enter to watch TV
// TV stays on till the last person leaves
// Rc allows a single value to have multiple owners
// the count ensures that the value remains valid
// as long as any of the owners exist

fn main() {
    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    println!(
        "count after creating a: {}",
        Rc::strong_count(&a)
    );
    let b = Cons(3, Rc::clone(&a));  // Rc::clone increments the reference count (not deep copy)
    println!(                       // it is the number of objects using that reference
        "count after creating b: {}",
        Rc::strong_count(&a)
    );
    {
        let c = Cons(4, Rc::clone(&a));
        println!(
            "count after creating c: {}",
            Rc::strong_count(&a)
        );
    }
    println!(
        "count after c goes out of scope: {}",
        Rc::strong_count(&a)
    );
}

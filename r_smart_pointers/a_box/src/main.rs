struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!(
            "Dropping CustomSmartPointer with data: {}",
            self.data
        );
    }
}

// NOTE: start Box

#[derive(Debug)]
enum List {
    Cons(i32, Box<List>),
    Nil,
}

use crate::List::{Cons, Nil};

fn main() {
    let b = Box::new(5);        // Box -> stores data in the heap
    println!("b = {b}");        // stack holds the pointer to the data in the heap

    let list = Cons(1, 
            Box::new(Cons(2,
            Box::new(Cons(3,
            Box::new(Nil))
        ))
    ));

    println!("{:?}", list);

    let x = 5;
    let y = Box::new(x);        // 5 is created in heap, y points to 5 from stack
    println!("y is {}", *y);    // the pointer is dereferenced this way

    let c = CustomSmartPointer {
        data: String::from("my stuff")
    };

    let d = CustomSmartPointer {
        data: String::from("Other stuff")
    };

    println!("Smart pointers created");;
}

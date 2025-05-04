#[derive(Debug)]
struct Rectangle {
    width: i32,
    height: i32,
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    let scale = 2;
    let rect2 = Rectangle {
        width: dbg!(30 * scale),      // outputs the answer with formatting on stderr
        height: 60,
    };

    println!("rect1 is {:?}", rect1); // prints rect1 without formatting
    println!("rect1 is {:#?}", rect1); // prints rect1 with formatting
    dbg!(rect2);                        // prints the same as before, but under stderr
}

/*
fn area(rect: &Rectangles) -> i32 {
    rect.width * rect.height
}
*/

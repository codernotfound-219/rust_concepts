fn main() {
    let x = 4;

    let x = x + 1;

    {
        let x = x * 2;
        println!("Value of x in inner scope: {x}"); // 10
    }

    println!("Value of x in outer scope: {x}"); // 5
}

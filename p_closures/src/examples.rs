pub fn example1() {
    let mut list = vec![1, 2, 3];
    println!("Before defining closures: {:?}", list);

    let mut borrows_mutables = || list.push(7);
    borrows_mutables();
    println!("After defining closures: {:?}", list);
}

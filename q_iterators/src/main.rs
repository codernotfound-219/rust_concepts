fn main() {
    let v1 = vec![1, 2, 3];
    let v1_iter = v1.iter();

    for val in v1_iter {
        println!("Got: {val}");
    }

    let v2 = v1.clone();
    let v2_iter = v2.iter();
    let total: i32 = v2_iter.sum();  // sum takes ownership
    println!("The sum is {total}");

    let v3 = v1.clone();
    let v4: Vec<_> = v3.iter().map(|x| x+1).collect();

    assert_eq!(v4, vec![2, 3, 4]); // panics if false
}

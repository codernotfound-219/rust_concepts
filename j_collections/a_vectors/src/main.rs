enum Cell {
    Int(i32),
    Float(f64),
    Text(String),
}

fn main() {
    let mut v: Vec<i32> = Vec::new();
    let v1 = vec![1, 2, 3, 4, 5];
    let row = vec![
        Cell::Int(28),
        Cell::Float(55.2),
        Cell::Text(String::from("Hi")),
    ];

    v.push(2);
    v.push(3);
    v.push(5);
    v.push(6);

    let third: &i32 = &v1[2];
    println!("third element: {third}");
    
    // better way
    let third: Option<&i32> = v1.get(2);    // get function returns Option<&T>
    if let Some(i) = third {
        println!("third element: {i}");
    } else {
        println!("there is no third element");
    }

    for i in &mut v {
        *i += 50;
        println!("{i}");
    }

    
}

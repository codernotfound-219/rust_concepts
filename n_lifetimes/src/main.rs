fn main() {
    let string_1 = String::from("Hello World");
    let string_2 = String::from("Leo");

    let smallest = longest(&string_1.as_str(), &string_2.as_str());
    println!("The smallest is : {}", smallest);
}

fn longest<'a>(str1: &'a str, str2: &'a str) -> &'a str {
    if str1.len() > str2.len() {
        return str2;
    }

    str1
}

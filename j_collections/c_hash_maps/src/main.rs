mod counting;

use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Black"), 20);
    scores.insert(String::from("White"), 53);

    let team_name = String::from("Blue");
    let score = scores.get(&team_name).copied().unwrap_or(0);

    //NOTE:
    // get method returns Option<&i32>
    // copied returns Option<i32>
    // unwrap_or returns the i32 if value exists
    // unwrap_or returns 0 if value does not exist (0 is fed to it)

    for (key, value) in &scores {
        println!("{key} => {value}");
    }

    println!("The team {team_name} scored {score} runs");

    // entry keyword
    scores.entry(String::from("Yellow")).or_insert(45);     // Checks if "Yellow" is a key.
                                                            // (NOT A KEY) so creates it
    scores.entry(String::from("Blue")).or_insert(34);       // Checks if "Blue" is a key.
                                                            // (IS A KEY) so does nothing

    let count = counting::count_occurences("Hello world wonderful world");
    println!("{:?}", count);
}

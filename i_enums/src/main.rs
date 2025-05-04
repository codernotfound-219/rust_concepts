/*
enum IPaddrKind {
    V4,
    V6,
}
*/ //Also valid

enum IP {
    V4(String),
    modV4(u8, u8, u8, u8),
    V6(String),
}

/*
enum Option<T> {
    None,
    Some(T),
}
*/ //NOTE: Option is already given by rust

fn main() {
    let home = IP::V4(String::from("121.0.0.9"));
    let mod_home = IP::modV4(127, 0, 0, 9);
    let loopback = IP::V6(String::from("::1"));

    let secure_home = check(home);
    let secure_mod = check(mod_home);
    let secure_loopback = check(loopback);

    println!("Is home secure?: {}", secure_home);
    println!("Is mod_home secure?: {}", secure_mod);
    println!("Is loopback secure?: {}", secure_loopback);
    
    // NOTE: IF LET statement
    let config_max = Some(3u8);
    if let Some(max) = config_max {             // max binds to the value in some
        println!("The maximum is configured to be {max}");
    }
    // If there is a value in Some, the if clause is executed
    // Else nothing happens
    // u can add an else statement as well!
}

// NOTE: need for if let:
//  let config_max = Some(3u8);
//  match config_max {
//      Some(max) => println!("The maximum is configured to be {max}"),
//      _ => (),
//  }

fn check(ip: IP) -> bool {
    match ip {
        IP::V4(_) => false,
        IP::modV4(..) => false,
        IP::V6(_) => true,
    }
}

fn testing_match(ip: IP) {
    match ip {
        IP::V4(_) => fancy(),
        _ => ()                     // represents "do nothing with other cases"
    }
}

fn fancy() {}


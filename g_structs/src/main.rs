// *--- STRUCT ---* //
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: i32
}

// *--- TUPLE STRUCT ---* //
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

// *--- UNIT STRUCT ---* //
struct AlwaysEqual;

fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username,                   // as the parameter has the same name
        email,                      // we can use it without specifying the field
        sign_in_count: 1
    }
}

fn main() {
    let user1 = build_user(String::from("Aswatt"), String::from("aswatt@gmail.com"));

    let user2 = User {
        email: String::from("aswatt_alternate@gmail.com"),
        ..user1
    };

    let white = Color(0, 0, 0);
    let _black = AlwaysEqual;

    /* NOTE:
    *  username of user1 is moved to user2
    *  therefore, user1 can no longer be used
    *  active, sign_in_count are copied on the stack
    *  if a separate username was created, user1 and user2 can be used
    */

    println!("{} + {}", user2.email, white.1);
}

//NOTE: checkout rectangles in book-problems

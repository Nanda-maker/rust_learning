// struct User {
//     active: bool,
//     username: String,
//     email: String,
//     sign_in_count: u64,
// }

fn main() {
    // let mut user1 = User {
    //     active: true,
    //     username: String::from("someusername123"),
    //     email: String::from("someone@example.com"),
    //     sign_in_count: 1,
    // };

    // user1.username = String::from("something_else");
    // user1.username.push_str("Hey");

    let user1 = build_user(
        username:String::from("nanda@beyul.com");
        email:String::from("nanda@beyul.com");
    );

    let user2= User{
        email:String::from("another@email.com"),
        ...user1
    }
    println!("The value is {}", user1.username);

//     struct Color(i32, i32, i32);
// struct Point(i32, i32, i32);

// fn main() {
//     let black = Color(0, 0, 0);
//     let origin = Point(0, 0, 0);
// }
}

fn build_user(username: String, email: String) -> user {
    User {
        active: true,
        username,
        email,
        sign_in_count: 0,
    }
}

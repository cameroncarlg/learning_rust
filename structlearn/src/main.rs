#[derive(Debug)]
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn build_user(username: String, email: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}

fn main() {
    let mut user1 = User {
        active: true,
        username: String::from("slydog"),
        email: String::from("asdf@asdf.com"),
        sign_in_count: 1,
    };

    let user2 = User {
        email: String::from("another@example.com"),
        username: String::from("useraa"),
        // the "overload" syntax must come last
        ..user1
    };

    let user3 = build_user(String::from("Asaki"), String::from("jpairlines@jp.com"));

    println!("{:#?}", user1);
    println!("{:#?}", user2);
    println!("{:#?}", user3);
}

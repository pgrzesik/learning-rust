fn main() {
    let mut user_instance = User {
        username: String::from("userrrr"),
        email: String::from("mail@mail.com"),
        sign_in_count: 0,
        active: true
    };

    user_instance.sign_in_count = 1;

    println!("User sign in count: {}", user_instance.sign_in_count);

    let built_user = build_user(
        String::from("mail@mail2.com"),
        String::from("mickey mouse")
    );

    let updated_user = User {
        username: String::from("updated_username"),
        ..user_instance
    };
}

fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        sign_in_count: 0,
        active: false
    }
}

struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool
}

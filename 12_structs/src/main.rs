fn main() {
    let mut user_instance = User {
        username: String::from("userrrr"),
        email: String::from("mail@mail.com"),
        sign_in_count: 0,
        active: true
    };

    user_instance.sign_in_count = 1;

    println!("User sign in count: {}", user_instance.sign_in_count);
}


struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool
}

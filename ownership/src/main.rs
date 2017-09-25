fn main() {
    let mut s = String::from("hello there!");

    // append to string

    s.push_str(" Nice to meet you!");

    println!("{}", s);

    let s2 = s;
    // s is no longer valid here!
    println!("{}", s);
}

fn main() {
    let s = String::from("Hello there!");

    // We're using reference here so the s is not invalidated
    let len = calculate_length(&s);
    println!("The length of {} is {}.", s, len);
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

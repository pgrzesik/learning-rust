fn main() {
    let s = String::from("Hello there!");

    // We're using reference here so the s is not invalidated
    let len = calculate_length(&s);
    println!("The length of {} is {}.", s, len);

    let mut s_mut = String::from("Hello from mutable!");
    change(&mut s_mut);
    println!("s_mut: {}", s_mut);
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

fn change(s: &mut String) {
    s.push_str(" Pushed from func!");
}

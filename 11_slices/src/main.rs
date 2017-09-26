fn main() {
    let s = String::from("Hello there");

    let first = first_word(&s[..]);
    println!("First word is: {}", first);
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }

    &s[..]
}

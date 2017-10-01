fn main() {
    let s1 = String::from("abc");
    let s2 = "defg";

    let result = longest(s1.as_str(), s2);
    println!("Longest result is: {}", result);
}

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

struct VeryImportant<'a> {
    part: &'a str,
}

impl<'a> VeryImportant<'a> {
    fn level(&self) -> i32 {
        3
    }
}

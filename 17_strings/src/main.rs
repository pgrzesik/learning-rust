fn main() {
    let mut s = String::new();

    let data = "random stuff";

    let s_to_string = data.to_string();

    let s_from = String::from("Initial content");

    s.push_str("pushed");

    s.push('l');

    let s1 = String::from("First");
    let s2 = String::from("Second");

    let s3 = s1 + &s2;  // s1 is no longer valid

    let macro_s1 = String::from("tic");
    let macro_s2 = String::from("tac");
    let macro_s3 = String::from("toe");

    let macro_s3 = format!("{}-{}-{}", macro_s1, macro_s2, macro_s3); // does not take ownership of any element

}

fn main() {
    let mut s = String::from("hello there!");

    // append to string

    s.push_str(" Nice to meet you!");

    println!("{}", s);

    let s1 = s.clone();

    // cloning doesn't invalidate original variable
    println!("s: {}, s1: {}", s, s1);

    let s2 = s;

    // s is no longer valid here!
    // println!("{}", s); would fail
    // To preserve we can use Copy trait

    let s_to_own = String::from("Some stuff!");
    takes_ownership(s_to_own);

    // s_to_own is no longed valid here since it was moved to func
    // println!("{}", s_to_own); fails here

    let x = 10;
    makes_copy(x);

    // x was copied to function so it's still avaialable in this scope

}


fn takes_ownership(owned_string: String) {
    println!("{}", owned_string);  // After the function ends, owned_string goes out of scope
}

fn makes_copy(copied_int: i32) {
    println!("{}", copied_int); // copied_int goes out of scope - nothing special happens
}
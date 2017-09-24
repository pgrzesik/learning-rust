fn main() {
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);

    const MAX_POINTS: u32 = 100_000;
    println!("The value of contant MAX_POINTS is: {}", MAX_POINTS);

    let tup = (500, 6.4, 1);

    let (a, b, c) = tup;
    println!("The value of a is: {}", a);

    let d = tup.0;
    println!("The value of d is: {}", d);
}

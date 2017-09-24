fn main() {
    let x = 9;

    if x < 4 {
        println!("Number less than 4!");
    } else if x > 10 {
        println!("Number greater than 10!");
    } else {
        println!("Number between 4 and 10!");
    }

    let condition = true;

    let assigned = if condition {
        5
    } else {
        6
    };
    println!("Value assigned from if expression is equal to: {}", assigned);
}

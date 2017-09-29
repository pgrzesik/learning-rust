fn main() {
    let v: Vec<i32> = Vec::new();

    // Macro usage for vector
    let v_from_macro = vec![1, 2, 3];

    let third: &i32 = &v_from_macro[2];
    let third_2: Option<&i32> = v_from_macro.get(2);

    let mut mutable_vector = Vec::new();

    mutable_vector.push(1);
    mutable_vector.push(2);

    let row = vec![
        SpreadsheetCell::Int(1),
        SpreadsheetCell::Float(1.23),
        SpreadsheetCell::Text(String::from("example"))
    ];
}


enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String)
}
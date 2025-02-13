
enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}

fn main() {
    // let v1: Vec<i32> = Vec::new();

    let mut fib = vec![1, 2, 3, 5, 8];
    fib.push(13);
    for i in &mut fib {
        *i *= 10;
    }

    for f in &fib {
        println!("{f}");
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Float(3.14),
        SpreadsheetCell::Text(String::from("Hello")),
    ];

    for r in &row {
        match r {
            SpreadsheetCell::Int(value) => println!("Int: {}", value),
            SpreadsheetCell::Float(value) => println!("Float: {}", value),
            SpreadsheetCell::Text(value) => println!("Text: {}", value),
        }
    }
}

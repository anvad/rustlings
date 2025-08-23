fn main() {
    #[derive(Debug)]
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        T(String),
    }
    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::T(String::from("blue")),
        SpreadsheetCell::Float(3.14),
    ];
    println!("{:?}", &row[1]);
    for cell in &row {
        match cell {
            SpreadsheetCell::Int(i) => {
                // looks like compiler took care to deref &i32 (i) when adding an int, to produce the correct type- i32 for j
                let j = i + 4;
                println!("i32: i={i} j={j}")
            }
            SpreadsheetCell::Float(f) => println!("f64: {f}"),
            SpreadsheetCell::T(s) => println!("String: {s}"),
        }
    }
}

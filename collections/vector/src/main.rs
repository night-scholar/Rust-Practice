fn main(){
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }
    
    let row = vec![
        SpreadsheetCell::Int(4),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];

    let _a = SpreadsheetCell::Int(3);
    match row.get(0){
        Some(_a) => println!("11"),
        _ => println!("22"),
    }
    
}   
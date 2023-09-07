enum SpreadsheetCell
{
    INT(i32), FLOAT(f32), TEXT(String)
}

fn main() 
{
    let mut row: Vec<SpreadsheetCell> = Vec::new();
    row.push(SpreadsheetCell::INT(3));
    row.push(SpreadsheetCell::FLOAT(10.0));
    row.push(SpreadsheetCell::TEXT(String::from("value")));
}

use std::collections::HashMap;
fn main() {
    // let v: Vec<i32> = Vec::new();
    // let v = vec![1, 2, 3];
    // let v_index = 2;
    // let third: &i32 = &v[2];
    // match v.get(v_index) {
    //     Some(_) => { println!("Reachable element at index: {}", v_index); },
    //     None => { println!("Unreachable element at index: {}", v_index); }
    // }
    // let mut v = vec![100, 32, 57];
    // for i in &mut v {
    //     *i += 50;
    // }
    // println!("Hello, world!, {:?}", v);
    // enum SpreadsheetCell {
    //     Int(i32),
    //     Float(f64),
    //     Text(String)
    // }
    // let row = vec![
    //     SpreadsheetCell::Int(3),
    //     SpreadsheetCell::Float(10.12),
    //     SpreadsheetCell::Text(String::from("blue"))
    // ];
    let teams = vec![String::from("Blue"), String::from("Red")];
    let initial_scores = vec![10, 50];
    let scores: HashMap<_, _> = teams.iter().zip(initial_scores.iter()).collect();
}

// #[derive(Debug)]

// enum UsState {
//     Alabama,
//     Alaska
// }

// enum Coin {
//     Penny,
//     Nickel,
//     Dime,
//     Quarter(UsState)
// }

// fn value_in_cents(coin: Coin) -> u32 {
//     match coin {
//         Coin::Penny => 1,
//         Coin::Nickel => 5,
//         Coin::Dime => 10,
//         Coin::Quarter(state) => {
//             println!("State quarter from {:?}!", state);
//             25
//         }
//     }
// }

// #![allow(unused_variables)]
// fn main() {
//     fn plus_one(x: Option<i32>) -> Option<i32> {
//         match x {
//             None => None,
//             Some(i) => Some(i + 1),
//         }
//     }

//     let five = Some(5);
//     let six = plus_one(five);
//     let none = plus_one(None);
// }

fn main() {
    let some_u8_value = Some(0u8);
    match some_u8_value {
        Some(3) => println!("three"),
        _ => ()
    }
}



// fn main() {
//     println!("Hello, world!");
// }

struct Point {
    x: i32,
    y: i32
}
// enum Color {
//    Rgb(i32, i32, i32),
//    Hsv(i32, i32, i32)
// }

// enum Message {
//     Quit,
//     Move { x: i32, y: i32 },
//     Write(String),
//     ChangeColor(Color),
// }

// fn main() {
//     let msg = Message::ChangeColor(Color::Hsv(0, 160, 255));

//     match msg {
//         Message::ChangeColor(Color::Rgb(r, g, b)) => {
//             println!(
//                 "Change the color to red {}, green {}, and blue {}",
//                 r,
//                 g,
//                 b
//             )     
//         },
//         Message::ChangeColor(Color::Hsv(h, s, v)) => {
//             println!(
//                 "Change the color to hue {}, saturation {}, and value {}",
//                 h,
//                 s,
//                 v
//             )
//         }
//         _ => ()
//     }
// }

// fn main () {
//     let points = vec![
//         Point { x: 0, y: 0 },
//         Point { x: 1, y: 5 },
//         Point { x: 10, y: -3 }
//     ];

//     let sum_of_squares: i32 = points.iter().map(|&Point  { x, y }| x * x + y * y).sum();
// }

fn main() {
    let x = Some(5);
    let y = 10;

    match x {
        Some(x) => println!("Got 50 {}", x),
        Some(n) if n == y => println!("Matched, n = {:?}", n),
        _ => println!("Default case, x = {:?}", x),
    }

    println!("at the end: x = {:?}, y = {:?}", x, y);
}

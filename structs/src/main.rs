// struct User {
//     username: String,
//     email: String,
//     sign_in_account: u64,
//     active: bool
// }

// fn build_user(email: String, username: String) -> User {
//     User {
//         email,
//         username,
//         active: true,
//         sign_in_count: 1
//     }
// }

// fn main() {
//     let user1 = build_user("someshit@gmail.com", "someshit");
//     let user2 = build_user("somenewshit@gmail.com", "user1.username");
//     let black = Color(0, 0, 0);
//     let origin = Point(0, 0, 0);
// }

// struct Color(i32, i32, i32);
// struct Point(i32, i32, i32);

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

fn main() {
    let rect1 = Rectangle { width: 30, height: 50 };

    println!("rect is {:#?}", rect1);

    println!(
        "The area of the rectange is {} square pixels",
        rect1.area()
    );
}

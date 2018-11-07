// use self::Lists::{Cons, Nil};
// use std::ops::Deref;
// use std::boxed::Box;

// enum Lists {
//     Cons(i32, Box<Lists>),
//     Nil
// }
// struct MyBox<T>(T);

// impl<T> Deref for MyBox<T> {
//     type Target = T;

//     fn deref(&self) -> &T {
//         &self.0
//     }
    
//     fn new(x: T) -> MyBox<T> {
//         MyBox(x)
//     }
// }
// fn main() {
//     let _list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
//     let x = 5;
//     let y = MyBox::new(x);

//     assert_eq!(5, x);
//     assert_eq!(5, *y);
// }
// use std::mem::drop;
// struct CustomSmartPointer {
//     data: String,
// }

// impl Drop for CustomSmartPointer {
//     fn drop(&mut self) {
//         println!("Dropping CustomSmartPointer with data `{}`!", self.data);
//     }
// }

// fn main() {
//     let a = CustomSmartPointer { data: String::from("my stuff 1") };
//     let b = CustomSmartPointer { data: String::from("other stuff 2") };
//     let c = CustomSmartPointer { data: String::from("my stuff 3") };
//     let d = CustomSmartPointer { data: String::from("other stuff 4") };
//     drop(c);
//     println!("CustomSmartPointers created.");
// }

// enum List {
//     Cons(i32, Rc<List>),
//     Nil
// }
// use self::List::{Cons, Nil};
// use std::rc::Rc;

// fn main() {
//     let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
//     println!("count after creating a = {}", Rc::strong_count(&a));
//     let b = Cons(3, Rc::clone(&a));
//     println!("count after creating b = {}", Rc::strong_count(&a));
//     {
//         let c = Cons(4, Rc::clone(&a));
//         println!("count after creating c = {}", Rc::strong_count(&a));
//     }
//      println!("count after c goes out of scope = {}", Rc::strong_count(&a));
// }
fn main() {
    println!("hello world");
}

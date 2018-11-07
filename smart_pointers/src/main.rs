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
#[derive(Debug)]
enum List {
    Cons(Rc<RefCell<i32>>, Rc<List>),
    Nil
}

use self::List::{Cons, Nil};
use std::rc::Rc;
use std::cell::RefCell;

fn main() {
    let value = Rc::new(RefCell::new(5));
    let a = Rc::new(Cons(Rc::clone(&value), Rc::new(Nil)));
    println!("count after creating a = {}", Rc::strong_count(&a));
    let b = Cons(Rc::new(RefCell::new(6)), Rc::clone(&a));
    println!("count after creating b = {}", Rc::strong_count(&a));
    {
        let c = Cons(Rc::new(RefCell::new(10)), Rc::clone(&a));
        println!("count after creating c = {}", Rc::strong_count(&a));
    }
    *value.borrow_mut() *= 10;
    println!("count after c goes out of scope = {}", Rc::strong_count(&a));
    println!("a after = {:?}", a);
    println!("b after = {:?}", b);
}

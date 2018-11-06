struct Point<T> {
    x: T,
    y: T
}

fn main() {
   let _integer = Point { x: 5, y: 10 };
   let _float = Point { x: 5.0, y: 10.0 };
}
use std::io;
use std::io::Read;
use std::fs::File;

// fn read_username_from_file() -> Result<String, io::Error> {
//   fs::read_to_string("hello.txt")
// }
fn main() {
// let _v: Result<String, io::Error> = read_username_from_file();
   let f = File::open("hello.txt")?;
}

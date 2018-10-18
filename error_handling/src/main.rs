use std::fs::File;
use std::io::ErrorKind;

fn main() {
    let f = File::open("hello.txt").map_err(|error| {
    	if error.kind() == ErrorKind::NotFound {
	   File::create("hello.txt").unwrap_or_else(|error| {
	       panic!("Tried to create file, but there was a problem {:?}", error);
	   })
	} else {
	  panic!("There was a problem in opening the file {:?}", error);
	}
    });
}

extern crate code;
use std::env;
use code::*;

fn main() {
    let files = env::args().skip(1);
    for path in files {
        match resize_image(path.as_ref()) {
            Ok(_) => {}
            Err(e) => println!("{}: error {:?}", path, e)
        }
    }
}

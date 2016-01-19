extern crate simple_parallel;
extern crate code;
use std::env;
use code::*;

fn main() {
    let files = env::args().skip(1);

    simple_parallel::for_(files, |s| {
        match resize_image(s.as_ref()) {
            Ok(_) => {}
            Err(e) => println!("{}: error {:?}", s, e)
        }
    });
}

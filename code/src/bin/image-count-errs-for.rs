extern crate code;
use std::env;
use code::*;

fn main() {
    let files = env::args().skip(1);

    let number_of_errors =
        files
        .map(|s| resize_image(s.as_ref()))
        .filter(|e| e.is_err())
        .count();

    println!("{} errors occurred", number_of_errors);
}

extern crate code;
use std::env;
use code::*;

fn main() {
    let files = env::args().skip(1);

    let number_of_errors =
        files
        .map(|path| resize_image(path.as_ref()))
        .filter(|res| res.is_err())
        .count();

    println!("{} errors occurred", number_of_errors);
}

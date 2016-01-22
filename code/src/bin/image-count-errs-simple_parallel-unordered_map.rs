extern crate simple_parallel;
extern crate crossbeam;
extern crate code;
use std::env;
use code::*;

fn main() {
    let files = env::args().skip(1).collect::<Vec<_>>();

    let number_of_errors = crossbeam::scope(|scope| {

        simple_parallel::unordered_map(scope,
                                       files,
                                       |s| resize_image(s.as_ref()))
            .filter(|e| e.1.is_err())
            .count()

    });

    println!("{} errors occurred", number_of_errors);
}

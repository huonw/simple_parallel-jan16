extern crate crossbeam;
extern crate simple_parallel;
extern crate code;
use std::env;
use code::*;

fn main() {
    let files = env::args().skip(1).collect::<Vec<_>>();

    let number_of_errors = crossbeam::scope(|scope| {

        simple_parallel::map(scope,
                             files,
                             |path| resize_image(path.as_ref()))
            .filter(|res| res.is_err())
            .count()

    });

    println!("{} errors occurred", number_of_errors);
}

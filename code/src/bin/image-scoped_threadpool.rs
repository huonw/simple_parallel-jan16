extern crate scoped_threadpool;
extern crate code;
use std::env;
use code::*;

fn main() {
    let files = env::args().skip(1);

    let mut pool = scoped_threadpool::Pool::new(4);
    pool.scoped(|scope| {
        for s in files {
            scope.execute(move || {
                match resize_image(s.as_ref()) {
                    Ok(_) => {}
                    Err(e) => println!("{}: error {:?}", s, e)
                }
            })
        }
    });
}

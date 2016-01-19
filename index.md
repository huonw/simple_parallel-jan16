% simple_parallel

Huon Wilson

<br>

![](parallel.png)

[<small>huonw.github.io/simple_parallel-jan16</small>](http://huonw.github.io/simple_parallel-jan16)

[<small>crates.io/crates/simple_parallel</small>](https://crates.io/crates/simple_parallel)

# Slooow, Simple: `for`

```rust
extern crate image;
use std::path::Path;

fn resize_image(path: &Path) -> image::ImageResult<()> {
    // load the file as an image
    let img = try!(image::open(path));

    // resize it
    let smaller = img.resize(400, 400, image::Lanczos3);

    // and save it with the same name in /tmp
    let output = Path::new("/tmp/").join(path.file_name().unwrap());
    let mut f = try!(File::create(output));
    smaller.save(&mut f, image::JPEG)
}

fn main() {
    // command line arguments
    let files = env::args().skip(1);

    for s in files {
        match resize_image(s.as_ref()) {
            Ok(_) => {}
            Err(e) => println!("{}: error {:?}", s, e)
        }
    }
}
```

# Slooow, Simple: `for`

```rust
let files = env::args().skip(1);

for s in files {
    match resize_image(s.as_ref()) {
        Ok(_) => {}
        Err(e) => println!("{}: error {:?}", s, e)
    }
}
```

# Fast, Complicated: threading

```rust
extern crate scoped_threadpool;

let files = env::args().skip(1);

// set up the threads
let mut pool = scoped_threadpool::Pool::new(4);
pool.scoped(|scope| {
    // run over the images
    for s in files {
        // spawning a job for each one
        scope.execute(move || {
            match resize_image(s.as_ref()) {
                Ok(_) => {}
                Err(e) => println!("{}: error {:?}", s, e)
            }
        })
    }
});
```


# Fast, Simple: *magic* threading

```rust
extern crate simple_parallel;

let files = env::args().skip(1);

simple_parallel::for_(files, |s| {
    match resize_image(s.as_ref()) {
        Ok(_) => {}
        Err(e) => println!("{}: error {:?}", s, e)
    }
})
```

# How does it work?

```rust
pub fn for_<I, F>(iter: I, f: F)
    where I: IntoIterator, // yields...
          I::Item: Send + Sync, // which are passed to...
          F: Fn(I::Item) + Sync
```

TODO: diagram showing iterator & some representation of parallelism

TODO: remove Sync when crossbeam loses it

# Sharing

Safe:

```rust
let mut data = [0; 10];
let outside = 1;

simple_parallel::for_(&mut data, |elem| *elem += outside);
```

<hr class="pause"></hr>

Unsafe:

```rust
let mut data = [0; 10];
let mut outside = 0;

simple_parallel::for_(&mut data, |elem| outside += *elem);
```

<hr class="pause"></hr>

```error
error: cannot assign to data in a captured outer variable in an `Fn` closure
     simple_parallel::for_(&mut data, |elem| outside += *elem);
                                             ^~~~~~~~~~~~~~~~

```

# Fast?

|   |   |
|---|---|
| `for ... in` | 19.7s |
| `simple_parallel::for_` | 5.5s |

3.6&times; faster.

# Iterators do more than `for`

```rust
let number_of_errors =
    files
        .map(|s| resize_image(s.as_ref()
        .filter(|e| e.is_err())
        .count();

println!("{} errors occurred", number_of_errors);
```

# `simple_parallel` does more than `for`!

```rust
let number_of_errors = crossbeam::scope(|scope| {

    simple_parallel::map(scope,
            files,
            |s| resize_image(s.as_ref()))
        .filter(|e| e.is_err())
        .count()

});

println!("{} errors occurred", number_of_errors);
```

# Ordering?

Some jobs finish faster than others: `unordered_map`.

```rust
let number_of_errors = crossbeam::scope(|scope| {

    simple_parallel::unordered_map(scope,
            files,
            |s| resize_image(s.as_ref()))
        .filter(|e| e.1.is_err())
        .count()

});

println!("{} errors occurred", number_of_errors);
```

TODO benchmark of this & `map`.

# TODO picture?

(mirrored Good, Bad, Ugly poster)

# The Ugly

- The internals
- Panics in a job generally cascade to become an abort: need stable
  `std::panic::recover` to be reliable.
- To be generically useful, the `Iterator` is a black box

# The Bad

Inflexible: designed for flat, embarrassingly parallel jobs (unlike rayon).

TODO: comparison picture of flat vs. nested.

# The Good

TODO: summary

extern crate image;

use std::fs::File;
use std::path::Path;

pub fn resize_image(path: &Path) -> image::ImageResult<()> {
    let img = try!(image::open(path));
    let smaller = img.resize(400, 400, image::Lanczos3);

    let output = Path::new("/tmp/").join(path.file_name().unwrap());
    let mut f = try!(File::create(output));
    smaller.save(&mut f, image::JPEG)
}

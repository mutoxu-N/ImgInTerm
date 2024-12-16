use std::io::Result;

use crossterm::terminal;
use image::open;

fn main() -> Result<()> {
    let image = match open("image.png") {
        Ok(image) => image.to_rgba8(),
        Err(e) => panic!("{}", e),
    };
    display(image);
    Ok(())
}

fn display(image: image::RgbaImage) {
    let term_size = terminal::size().unwrap();
    println!("term_size: {}x{}", term_size.0, term_size.1);

    println!("dims: {:?}", image.dimensions());
}

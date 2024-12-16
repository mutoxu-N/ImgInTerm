use std::io::Result;

use crossterm::terminal;
use image::GenericImageView;

fn main() -> Result<()> {
    let term_width = terminal::size()?.0;
    let term_height = terminal::size()?.1;
    println!("{}x{}", term_width, term_height);

    let image = match image::open("image.png") {
        Ok(image) => image,
        Err(e) => panic!("{}", e),
    };
    println!("dims: {:?}", image.dimensions());
    Ok(())
}

use std::io::Result;

use crossterm::terminal;

fn main() -> Result<()> {
    let term_width = terminal::size()?.0;
    let term_height = terminal::size()?.1;
    println!("{}x{}", term_width, term_height);
    Ok(())
}

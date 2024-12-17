mod mod_display;
mod mod_events;

use std::io::stdout;
use std::thread::sleep;
use std::time::Duration;

use crossterm::cursor::MoveToColumn;
use crossterm::execute;
use crossterm::style::Print;
use crossterm::terminal::{
    disable_raw_mode, enable_raw_mode, Clear, ClearType, EnterAlternateScreen, LeaveAlternateScreen,
};
use image::open;

fn main() {
    let args = std::env::args().collect::<Vec<String>>();
    println!("args: {:#?}", args);

    let image_path = if args.len() > 1 {
        &args[1]
    } else {
        "sample.png"
    };

    // set terminal
    execute!(stdout(), EnterAlternateScreen).unwrap();
    enable_raw_mode().unwrap();

    // main process
    let image = open(image_path);
    let info = mod_display::DisplayInfo {
        image_file_path: image_path.to_string(),
    };
    mod_display::display(image, info);
    sleep(Duration::from_millis(1000));

    loop {
        match mod_events::handle_events() {
            Ok(true) => (),
            Ok(false) => break,
            Err(e) => execute!(
                stdout(),
                MoveToColumn(0),
                Print(format!("[info]: {}", e)),
                Clear(ClearType::UntilNewLine),
            )
            .unwrap(),
        }
    }

    // reset terminal
    execute!(stdout(), LeaveAlternateScreen).unwrap();
    disable_raw_mode().unwrap();
}

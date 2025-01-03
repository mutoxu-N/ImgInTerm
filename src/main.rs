mod mod_display;
mod mod_events;

use std::io::stdout;

use crossterm::cursor::MoveToColumn;
use crossterm::execute;
use crossterm::style::Print;
use crossterm::terminal::{
    disable_raw_mode, enable_raw_mode, Clear, ClearType, EnterAlternateScreen, LeaveAlternateScreen,
};

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
    let mut info = mod_display::DisplayInfo {
        image_file_path: image_path.to_string(),
        magnify: 1.0,
        center: (-1.0, -1.0),
    };
    mod_display::display(&mut info);
    let mut current_info = info.clone();

    loop {
        match mod_events::handle_events(&mut info) {
            Ok(true) => {
                if current_info != info {
                    mod_display::display(&mut info);
                    current_info = info.clone();
                }
            }
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

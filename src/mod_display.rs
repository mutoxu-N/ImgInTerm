use std::io::{stdout, Write};

use crossterm::cursor::MoveTo;
use crossterm::style::{Color, Print, ResetColor, SetBackgroundColor, SetForegroundColor};
use crossterm::{queue, terminal};
use image::{DynamicImage, GenericImageView, ImageError};

pub struct DisplayInfo {
    pub image_file_path: String,
}

pub fn display(image: Result<DynamicImage, ImageError>, info: DisplayInfo) {
    // clear terminal
    // queue!(stdout(), terminal::Clear(terminal::ClearType::All)).unwrap();

    // get terminal size
    let term_size = terminal::size().unwrap();
    let term_size = (term_size.0 as f64, (term_size.1 * 2) as f64);
    println!("term_size: {}x{}", term_size.0, term_size.1);

    if image.is_ok() {
        let image = image.unwrap();

        // get image size
        let image_size = image.dimensions();
        let image_size = (image_size.0 as f64, image_size.1 as f64);
        println!("image_size: {}x{}", image_size.0, image_size.1);

        // resize image to fit screen with keeping aspect ratio
        let mut display_size = (image_size.0 / image_size.1 * term_size.1, term_size.1);
        if display_size.0 > term_size.0 {
            display_size.0 = term_size.0;
            display_size.1 = display_size.0 / image_size.0 * image_size.1;
        }
        println!("display_size: {}x{}", display_size.0, display_size.1);

        // convert to u32
        let term_size = (term_size.0 as u32, term_size.1 as u32);
        let display_size = (display_size.0 as u32, display_size.1 as u32);

        // anchor point
        let anchor_point = (
            (term_size.0 - display_size.0) / 2,
            (term_size.1 - display_size.1) / 2,
        );

        // resize image
        let image = image.resize(
            display_size.0,
            display_size.1,
            image::imageops::FilterType::Nearest,
        );

        println!(
            "print size: {}x{}",
            image.dimensions().0,
            image.dimensions().1
        );

        for _ in 0..(display_size.1 / 2) {
            println!();
        }

        // print image
        let image = image.to_rgba8();
        for y in 0..(display_size.1 / 2) {
            for x in 0..display_size.0 {
                let true_y = 2 * y;
                let upper_pixel = image.get_pixel(x, true_y);
                let (r1, g1, b1) = (upper_pixel[0], upper_pixel[1], upper_pixel[2]);

                let upper_color = Color::Rgb {
                    r: r1,
                    g: g1,
                    b: b1,
                };

                let lower_color = if true_y + 1 < (image_size.1 as u32) {
                    let lower_pixel = image.get_pixel(x, true_y + 1);
                    let (r2, g2, b2) = (lower_pixel[0], lower_pixel[1], lower_pixel[2]);
                    Color::Rgb {
                        r: r2,
                        g: g2,
                        b: b2,
                    }
                } else {
                    Color::Rgb { r: 0, g: 0, b: 0 }
                };

                queue!(
                    stdout(),
                    MoveTo((anchor_point.0 + x) as u16, (anchor_point.1 + y) as u16),
                    SetForegroundColor(upper_color),
                    SetBackgroundColor(lower_color),
                    Print("\u{2580}"),
                    ResetColor,
                )
                .unwrap();
            }
        }
        stdout().flush().unwrap();
    } else {
        // Image open error
        println!("Error: {}", image.unwrap_err());
        println!("Image path: {}", info.image_file_path);
    }
}

use std::io::{stdout, Write};

use crossterm::cursor::MoveTo;
use crossterm::style::{Color, Print, ResetColor, SetBackgroundColor, SetForegroundColor};
use crossterm::{queue, terminal};
use image::{open, DynamicImage, GenericImageView, RgbaImage};
use image::{GenericImage, Rgba};

#[derive(Debug, PartialEq, Clone)]
pub struct DisplayInfo {
    pub image_file_path: String,
    pub magnify: f64,
    pub center: (f64, f64),
    pub clip_size: (f64, f64),
    pub img_size: (u32, u32),
}

pub fn display(info: &mut DisplayInfo) {
    // clear terminal
    queue!(stdout(), terminal::Clear(terminal::ClearType::All)).unwrap();

    let image = open(&info.image_file_path);

    // get terminal size
    if image.is_ok() {
        // get terminal size
        let (term_width, term_height) = terminal::size().unwrap();

        // calculate window_size
        let (win_width, win_height) = (term_width, (term_height - 1) * 2);
        let (win_width, win_height) = (win_width as u32, win_height as u32);

        // create background
        let bg_color = Rgba([0, 0, 0, 255]);
        let mut bg = DynamicImage::ImageRgba8(RgbaImage::new(win_width, win_height));
        for y in 0..win_height {
            for x in 0..win_width {
                bg.put_pixel(x, y, bg_color);
            }
        }

        // load, resize and clip image
        let mut image = image.unwrap();
        let image = if info.center.0 < 0.0 || info.center.1 < 0.0 {
            // if default size
            info.center = (image.width() as f64 / 2.0, image.height() as f64 / 2.0);
            info.clip_size = (image.width() as f64, image.height() as f64);
            info.img_size = (image.width(), image.height());
            image.resize(win_width, win_height, image::imageops::FilterType::Nearest)
        } else {
            // if clipping needed
            let (img_width, img_height) = image.dimensions();
            let (img_width, img_height) = (img_width as f64, img_height as f64);
            let (clip_width, clip_height) =
                if img_height / img_width > win_height as f64 / win_width as f64 {
                    // fit height
                    (
                        img_height / win_height as f64 * win_width as f64,
                        img_height,
                    )
                } else {
                    // fit width
                    (img_width, img_width / win_width as f64 * win_height as f64)
                };
            let (clip_width, clip_height) = (clip_width / info.magnify, clip_height / info.magnify);
            info.clip_size = (clip_width, clip_height);

            let (l, t) = (
                (info.center.0 - clip_width / 2.0) as u32,
                (info.center.1 - clip_height / 2.0) as u32,
            );
            image
                .crop(l, t, clip_width as u32, clip_height as u32)
                .resize(win_width, win_height, image::imageops::FilterType::Nearest)
        };
        let (img_width, img_height) = image.dimensions();

        // create buffer
        let mut buffer = DynamicImage::ImageRgba8(RgbaImage::new(win_width, win_height));

        let (anchor_x, anchor_y) = ((win_width - img_width) / 2, (win_height - img_height) / 2);
        for y in 0..img_height {
            for x in 0..img_width {
                let pixel = image.get_pixel(x, y);
                buffer.put_pixel(x + anchor_x, y + anchor_y, pixel);
            }
        }

        // display image in terminal from buffer
        for y in 0..(term_height - 1) {
            for x in 0..term_width {
                let (x, y) = (x as u32, y as u32);
                let true_y = y * 2;

                let upper_pixel = buffer.get_pixel(x, true_y);
                let upper_bg = bg.get_pixel(x, true_y);
                let upper_color = blend(upper_pixel, upper_bg);

                let lower_pixel = buffer.get_pixel(x, true_y + 1);
                let lower_bg = bg.get_pixel(x, true_y + 1);
                let lower_color = blend(lower_pixel, lower_bg);

                queue!(
                    stdout(),
                    MoveTo(x as u16, y as u16),
                    SetForegroundColor(upper_color),
                    SetBackgroundColor(lower_color),
                    Print("\u{2580}"),
                    ResetColor,
                )
                .unwrap();
            }
        }
        queue!(
            stdout(),
            MoveTo(0, term_height - 1),
            // Print(format!("Image \"{}\" displayed!", info.image_file_path)),
            Print(format!(
                "magnify: x{:.2}, center: ({:.2}, {:.2})",
                info.magnify, info.center.0, info.center.1
            )),
            MoveTo(0, term_height - 1),
        )
        .unwrap();
        stdout().flush().unwrap();
    } else {
        // Image open error
        println!("Error: {}", image.unwrap_err());
        println!("Image path: {}", info.image_file_path);
    }
}

fn blend(pixel: Rgba<u8>, bg: Rgba<u8>) -> Color {
    let alpha = pixel[3] as f64 / 255.0;
    Color::Rgb {
        r: (pixel[0] as f64 * alpha + bg[0] as f64 * (1.0 - alpha)) as u8,
        g: (pixel[1] as f64 * alpha + bg[1] as f64 * (1.0 - alpha)) as u8,
        b: (pixel[2] as f64 * alpha + bg[2] as f64 * (1.0 - alpha)) as u8,
    }
}

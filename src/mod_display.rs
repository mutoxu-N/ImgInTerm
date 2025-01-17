use std::io::{stdout, Write};

use crossterm::cursor::MoveTo;
use crossterm::style::{Color, Print, ResetColor, SetBackgroundColor, SetForegroundColor};
use crossterm::{queue, terminal};
use image::{DynamicImage, GenericImageView, ImageError, RgbaImage};
use image::{GenericImage, Rgba};

#[derive(Debug, PartialEq, Clone)]
pub struct DisplayInfo {
    pub image_file_path: String,
    pub magnify: f64,
    pub center: (f64, f64),
    pub clip_size: (f64, f64),
    pub img_size: (u32, u32),
    pub show_help: bool,
}

pub fn display(image: &Result<DynamicImage, ImageError>, info: &mut DisplayInfo) {
    // clear terminal
    queue!(stdout(), terminal::Clear(terminal::ClearType::All)).unwrap();

    // get terminal size
    if image.is_ok() {
        // get terminal size
        let (term_width, term_height) = terminal::size().unwrap();

        // calculate window_size
        let (win_width, win_height) = (term_width, (term_height - 1) * 2);
        let (win_width, win_height) = (win_width as u32, win_height as u32);

        // create background
        let bg_color_light = Rgba([153, 153, 153, 0]);
        let bg_color_dark = Rgba([102, 102, 102, 0]);
        let mut bg = DynamicImage::ImageRgba8(RgbaImage::new(win_width, win_height));
        for y in 0..win_height {
            for x in 0..win_width {
                let bg_color = if (x + y) % 2 == 0 {
                    bg_color_light
                } else {
                    bg_color_dark
                };
                bg.put_pixel(x, y, bg_color);
            }
        }

        // load, resize and clip image
        let img = image.as_ref().unwrap();
        let img = if info.center.0 < 0.0 || info.center.1 < 0.0 {
            // if default size
            info.center = (img.width() as f64 / 2.0, img.height() as f64 / 2.0);
            info.clip_size = (img.width() as f64, img.height() as f64);
            info.img_size = (img.width(), img.height());
            img.resize(win_width, win_height, image::imageops::FilterType::Nearest)
        } else {
            // if clipping needed
            let (img_width, img_height) = img.dimensions();
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
            let mut img = img.clone();
            img.crop(l, t, clip_width as u32, clip_height as u32)
                .resize(win_width, win_height, image::imageops::FilterType::Nearest)
        };
        let (img_width, img_height) = img.dimensions();

        // create buffer
        let mut buffer = DynamicImage::ImageRgba8(RgbaImage::new(win_width, win_height));

        let (anchor_x, anchor_y) = ((win_width - img_width) / 2, (win_height - img_height) / 2);
        for y in 0..img_height {
            for x in 0..img_width {
                let pixel = img.get_pixel(x, y);
                buffer.put_pixel(x + anchor_x, y + anchor_y, pixel);

                let bg_pixel = bg.get_pixel(x + anchor_x, y + anchor_y);
                bg.put_pixel(
                    x + anchor_x,
                    y + anchor_y,
                    Rgba([bg_pixel[0], bg_pixel[1], bg_pixel[2], 255]),
                );
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

        // show help
        if info.show_help {
            let version = env!("CARGO_PKG_VERSION");

            let title = format!("Image In Terminal - v{}", version);
            let msgs = [
                "w/W: zoom in".to_string(),
                "s/S: zoom out".to_string(),
                "h/H: move left".to_string(),
                "l/L: move right".to_string(),
                "j/J: move down".to_string(),
                "k/K: move up".to_string(),
                " y : hide help".to_string(),
                " o : open image".to_string(),
                " q : exit".to_string(),
            ];

            let (mut w2, h) = (0, msgs.len() as u16 + 2);
            for msg in &msgs {
                w2 = w2.max(msg.len() as u16);
            }
            let w1 = w2.max(title.len() as u16);

            let (anchor_help_w, anchor_help_h) = (
                (term_width - w1 - 4) / 2,
                (term_height - h - 2) / 2,
            );

            let help_padding = (w1 - w2) / 2;

            // show border
            for x in 0..(w1 + 4) {
                queue!(
                    stdout(),
                    MoveTo(anchor_help_w + x, anchor_help_h),
                    ResetColor,
                    Print(if x == 0 || x == w1 + 3 { "\u{2588}" } else { "\u{2580}" }),
                )
                .unwrap();
            }

            for y in 0..h {
                for x in 0..(w1 + 4) {
                    queue!(
                        stdout(),
                        MoveTo(anchor_help_w + x, anchor_help_h+1 + y),
                        ResetColor,
                        Print(if x == 0 || x == w1 + 3 { "\u{2588}" } else { " " }),
                    )
                    .unwrap();
                }
            }
            for x in 0..(w1 + 4) {
                queue!(
                    stdout(),
                    MoveTo(anchor_help_w + x, anchor_help_h+h+1),
                    ResetColor,
                    Print(if x == 0 || x == w1 + 3 { "\u{2588}" } else { "\u{2584}" }),
                )
                .unwrap();
            }
            // show title
            queue!(
                stdout(),
                MoveTo(anchor_help_w, anchor_help_h + 1),
                Print("\u{2588} "),
                MoveTo(anchor_help_w+2,  anchor_help_h + 1),
                Print(&title),
                MoveTo(anchor_help_w + 2 + w1,  anchor_help_h + 1),
                Print(" \u{2588}"),
            )
            .unwrap();

            // show help message

            for (i, msg) in msgs.iter().enumerate() {
                queue!(
                    stdout(),
                    MoveTo(anchor_help_w + 2 + help_padding, anchor_help_h + 3 + i as u16),
                    Print(msg),
                )
                .unwrap();
            }
        }

        let show_hint_msg = format!(
            "Press 'y' to {} help",
            if info.show_help { "hide" } else { "show" }
        );
        queue!(
            stdout(),
            MoveTo(0, term_height - 1),
            Print(format!(
                "magnify: x{:.2}, center: ({:.2}, {:.2})",
                info.magnify, info.center.0, info.center.1
            )),
            MoveTo(term_width - (show_hint_msg.len() as u16), term_height - 1),
            Print(show_hint_msg),
            MoveTo(0, term_height - 1),
        )
        .unwrap();
        stdout().flush().unwrap();
    } else {
        let (_, term_height) = terminal::size().unwrap();

        // Image open error
        let err = image.as_ref().unwrap_err();
        queue!(
            stdout(),
            MoveTo(0, term_height - 3),
            Print(format!("Error: {}", err)),
            MoveTo(0, term_height - 2),
            Print(format!("Image path: {}", info.image_file_path)),
            MoveTo(0, term_height - 1),
            Print("Press 'o' to type file path or 'q' to exit."),
            MoveTo(0, term_height - 1),
        )
        .unwrap();
        stdout().flush().unwrap();
    }
}

fn blend(pixel: Rgba<u8>, bg: Rgba<u8>) -> Color {
    if bg[3] == 0 {
        Color::Rgb { r: 0, g: 0, b: 0 }
    } else {
        let alpha = pixel[3] as f64 / 255.0;
        Color::Rgb {
            r: (pixel[0] as f64 * alpha + bg[0] as f64 * (1.0 - alpha)) as u8,
            g: (pixel[1] as f64 * alpha + bg[1] as f64 * (1.0 - alpha)) as u8,
            b: (pixel[2] as f64 * alpha + bg[2] as f64 * (1.0 - alpha)) as u8,
        }
    }
}

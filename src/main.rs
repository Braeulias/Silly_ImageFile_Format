extern crate image;

use std::{env, fs};
use std::fs::OpenOptions;
use std::io::Write;
use std::io::Error;
use std::path::PathBuf;
use image::{GenericImageView, ImageBuffer, Pixel, RgbaImage, DynamicImage, Rgba};
use eframe::{egui, Frame};
use eframe::egui::CursorIcon::Default;
use eframe::egui::{Context, Image};
use skia_safe::{AlphaType, Color4f, ColorType, ImageInfo, Paint, Rect, Surface};
use css_color_parser::Color as CssColor;



fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 || args.len() > 3 {
        eprintln!("Usage: cargo run conv <path-to-image> or cargo run <path-to-silly>");
        std::process::exit(1);
    }


    if args[1] == "conv" {
        let path_buf = PathBuf::from(&args[2]);

        match path_buf.extension().and_then(|s| s.to_str()) {
            Some("jpg") | Some("jpeg") => {
                match jpeg_to_silly(path_buf) {
                    Ok(()) => println!("Conversion to .silly successful!"),
                    Err(e) => eprintln!("Error during .silly conversion: {}", e),
                }
            }
            Some("silly") => {
                match silly_to_jpeg(path_buf) {
                    Ok(()) => println!("Conversion to .jpg successful!"),
                    Err(e) => eprintln!("Error during .jpg conversion: {}", e),
                }
            }
            _ => {
                eprintln!("Unsupported file extension. Please provide a .jpg/.jpeg or a .silly file.");
                std::process::exit(1);
            }
        }
    } else {
        let path_buf = PathBuf::from(&args[1]);
        match silly_to_jpeg(path_buf) {
            Ok(()) => println!("Conversion to JPEG successful!"),
            Err(e) => eprintln!("Error during .silly to JPEG conversion: {}", e),
        }
    }


}

fn jpeg_to_silly(path_buf: PathBuf) -> Result<(), Error>{
    let img = image::open(&path_buf).expect("Failed to open img");
    let (width, height) = img.dimensions();
    let mut hex_str = String::new();


    for y in 0..height {
        for x in 0..width {
            let pixel = img.get_pixel(x, y);
            let rgba = pixel.to_rgba();
            hex_str.push_str(&format!("{:02X}{:02X}{:02X}", rgba[0], rgba[1], rgba[2]));
        }
        hex_str.push_str("\n")
    }


    let height_byt = height.to_ne_bytes();
    let width_byt = width.to_ne_bytes();

    let silly_path = path_buf.with_extension("silly");

    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .open(silly_path)
        .expect("Could not open file");

    let string_bytes = Vec::from(hex_str.as_bytes());

    file.write_all(&width_byt).unwrap();
    file.write_all(&height_byt).unwrap();
    file.write_all(&string_bytes).unwrap();
    file.flush().unwrap();

    Ok(())
}

fn silly_to_jpeg(path_buf: PathBuf) -> Result<(), Error>{

    let contents = fs::read(&path_buf)?;

    if contents.len() < 8 {
        return Err(Error::new(std::io::ErrorKind::InvalidData, "File too short"));
    }

    let width = ne_vec_to_u32(&contents[0..4]);
    let height = ne_vec_to_u32( &contents[4..8]);

    let hex_data = String::from_utf8_lossy(&contents[8..]).replace("\n","");
    let result: Vec<&str> = hex_data
        .as_bytes()
        .chunks(6)
        .map(std::str::from_utf8)
        .collect::<Result<_, _>>()
        .expect("Inavlid UTF-8 sequence in input Stream");

    let info = ImageInfo::new(
        (width as i32, height as i32),
        ColorType::RGBA8888,
        AlphaType::Opaque,
        None,
    );

    let mut surface = Surface::new_raster(&info, None, None).unwrap();
    let canvas = surface.canvas();

    for (i, color) in result.iter().enumerate() {
        let hex = "#".to_owned() + color;

        let parsed_color = hex
            .parse::<CssColor>()
            .expect("Failed to conv HEX to RGB");

        let color4f = Color4f::new(
            parsed_color.r as f32,
            parsed_color.g as f32,
            parsed_color.b as f32,
            0.004 as f32,
        );

        let paint = Paint::new(color4f, None);

        let x = i % width as usize;
        let y = i / width as usize;

        let rect = Rect::from_point_and_size((x as f32, y as f32), (1.0, 1.0));
        canvas.draw_rect(rect, &paint);

    }

    let image = surface.image_snapshot();

    if let Some(data) = image.encode(None, skia_safe::EncodedImageFormat::JPEG, 100) {
        fs::write("cat2.jpg", &*data).expect("Failed to write image data to file");
    }

    Ok(())
}

fn ne_vec_to_u32(bytes: &[u8]) -> u32{
    let mut res = [0u8; 4];
    res.copy_from_slice(bytes);
    u32::from_ne_bytes(res)
}









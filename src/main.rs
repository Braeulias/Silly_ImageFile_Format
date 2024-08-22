extern crate image;

use std::{env, fs};
use std::default::Default;
use std::fs::OpenOptions;
use std::io::Write;
use std::io::Error;
use std::path::PathBuf;
use image::{GenericImageView, Pixel};
use eframe::{egui, Frame, App};
use eframe::egui::{ColorImage, Context, IconData, TextureOptions, ViewportBuilder};
use skia_safe::{AlphaType, ColorType};


struct MyApp {
    color_image: Option<ColorImage>,
    texture_handle: Option<egui::TextureHandle>,
    width: usize,
    height: usize,
}

impl MyApp {
    fn new(path_buf: PathBuf) -> Self {
        let color_image = silly_to_egui_image(path_buf.clone()).ok();
        let (width, height) = color_image.as_ref()
            .map(|img| (img.size[0], img.size[1]))
            .unwrap_or((0, 0));

        MyApp {
            color_image,
            texture_handle: None,
            width,
            height,
        }
    }
}


impl App for MyApp {
    //set img
    fn update(&mut self, ctx: &Context, _: &mut Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            if let Some(color_image) = &self.color_image {
                if self.texture_handle.is_none() {
                    self.texture_handle = Some(ctx.load_texture(
                        "silly_image",
                        color_image.clone(),
                        TextureOptions::default(),
                    ));
                }

                if let Some(texture_handle) = &self.texture_handle {
                    ui.image(texture_handle);
                }
            }
        });
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 || args.len() > 3 {
        eprintln!("Usage: cargo run conv <path-to-image> or cargo run <path-to-silly>");
        std::process::exit(1);
    }

    //checks if it should open or convert
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
        let app = MyApp::new(path_buf.clone());

        if let file_name = path_buf.clone().file_stem().and_then(|s| s.to_str()).unwrap_or("My egui App") {

            let viewport= ViewportBuilder::default()
                .with_title(file_name)
                .with_inner_size((app.width as f32, app.height as f32))
                .with_resizable(false);



            let options = eframe::NativeOptions {
                viewport,
                ..Default::default()
            };


            eframe::run_native(
                file_name,
                options,
                Box::new(|_cc| Ok(Box::new(app))),
            );
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
            hex_str.push_str(&format!("{:02X}{:02X}{:02X}{:02X}", rgba[0], rgba[1], rgba[2], rgba[3]));
        }
    }


    let height_byt = height.to_ne_bytes();
    let width_byt = width.to_ne_bytes();

    let silly_path = path_buf.with_extension("silly");

    let mut file = OpenOptions::new()       //create/open a writable file
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

    let hex_data = String::from_utf8_lossy(&contents[8..]).replace("\n",""); //converts utf8 to String but wont stop if invalid char

    let expected_length = (width * height * 4) as usize;
    if hex_data.len() != expected_length * 2 {
        return Err(Error::new(std::io::ErrorKind::InvalidData, "Hex data length mismatch"));    //checks len
    }

    let mut pixels = Vec::with_capacity(expected_length);

    for chunk in hex_data.as_bytes().chunks(8) { // 8 characters (4 bytes of RGBA)
        let hex_str = std::str::from_utf8(chunk).expect("Invalid UTF-8 sequence");
        // Parse RGBA values from the hex string
        let r = u8::from_str_radix(&hex_str[0..2], 16).expect("Invalid hex string");
        let g = u8::from_str_radix(&hex_str[2..4], 16).expect("Invalid hex string");
        let b = u8::from_str_radix(&hex_str[4..6], 16).expect("Invalid hex string");
        let a = u8::from_str_radix(&hex_str[6..8], 16).unwrap_or(255); // Default to 255 if alpha is not present

        pixels.push(r);
        pixels.push(g);
        pixels.push(b);
        pixels.push(a);
    }

    if pixels.len() != expected_length {
        return Err(Error::new(std::io::ErrorKind::InvalidData, "Image size mismatch"));
    }

    let data = skia_safe::Data::new_copy(&pixels);

    let image_info = skia_safe::ImageInfo::new(
        (width as i32, height as i32),
        ColorType::RGBA8888,
        AlphaType::Premul,
        None,
    );

    let image = skia_safe::Image::from_raster_data(
        &image_info,
        data,
        (width as i32 * 4) as usize,
    ).ok_or_else(|| Error::new(std::io::ErrorKind::InvalidData, "Failed to create Skia image"))?;

    let mut output_path = path_buf.clone();
    output_path.set_extension("jpg");

    // Encode and save the image as JPEG
    if let Some(data) = image.encode(None, skia_safe::EncodedImageFormat::JPEG, 100) {
        fs::write(output_path, &*data).expect("Failed to write image data to file");
    }

    Ok(())
}

fn silly_to_egui_image(path_buf: PathBuf) -> Result<ColorImage, Error> {
    let contents = fs::read(&path_buf)?;

    if contents.len() < 8 {
        return Err(Error::new(std::io::ErrorKind::InvalidData, "File too short"));
    }

    let width = ne_vec_to_u32(&contents[0..4]);
    let height = ne_vec_to_u32( &contents[4..8]);

    let hex_data = String::from_utf8_lossy(&contents[8..]).replace("\n","");

    let expected_length = (width * height * 4) as usize;
    if hex_data.len() != expected_length * 2 {
        return Err(Error::new(std::io::ErrorKind::InvalidData, "Hex data length mismatch"));
    }

    let mut result = Vec::with_capacity(expected_length);

    for chunk in hex_data.as_bytes().chunks(8) {
        let hex_str = std::str::from_utf8(chunk).expect("Invalid UTF-8 sequence");

        let r = u8::from_str_radix(&hex_str[0..2], 16).expect("Invalid hex string");
        let g = u8::from_str_radix(&hex_str[2..4], 16).expect("Invalid hex string");
        let b = u8::from_str_radix(&hex_str[4..6], 16).expect("Invalid hex string");
        let a = u8::from_str_radix(&hex_str[6..8], 16).unwrap_or(255); // Default to 255 if alpha is not present

        result.push(r);
        result.push(g);
        result.push(b);
        result.push(a);
    }

    println!("Width: {}, Height: {}, Data Length: {}", width, height, result.len());
    println!("Expected Length: {}", (width * height * 4) as usize);


    if result.len() != expected_length {
        return Err(Error::new(std::io::ErrorKind::InvalidData, "Image size mismatch"));
    }

    let color_image = ColorImage::from_rgba_unmultiplied([width as usize, height as usize], &result);

    Ok(color_image)

}


fn ne_vec_to_u32(bytes: &[u8]) -> u32{
    let mut res = [0u8; 4];
    res.copy_from_slice(bytes);
    u32::from_ne_bytes(res)
}









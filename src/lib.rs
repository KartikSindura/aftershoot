use clap::Args;
use clap::Subcommand;
use image::{
    GenericImageView, ImageBuffer, Luma,
    imageops::{FilterType::Lanczos3, grayscale, resize},
};
use std::ops::Deref;
use std::ops::DerefMut;
use std::path::Path;

#[derive(Debug, Clone, Subcommand)]
#[clap(rename_all = "kebab_case")]
pub enum Style {
    Acerola,
    Me,
    More,
    Custom(AddArgs),
}

#[derive(Args, Debug, Clone)]
pub struct AddArgs {
    chars: String,
}

impl Deref for AddArgs {
    type Target = String;
    fn deref(&self) -> &Self::Target {
        &self.chars
    }
}
impl DerefMut for AddArgs {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.chars
    }
}

impl Style {
    pub fn chars(&self) -> Vec<char> {
        // TODO: Send reference to a string or char array
        match self {
            Self::Acerola => vec![' ', '.', ';', 'c', 'o', 'P', 'O', '?', '@', '█'],
            Self::Me => vec![' ', '.', '-', '+', '*', '%', '#', '&', '@', '█'],
            Self::More => vec![' ', '.', '-', '+', '*', '%', '#', '?', '&', '@', '█'],
            Self::Custom(inner) => inner.chars.chars().collect::<Vec<char>>(),
        }
    }
}

pub trait Rounding {
    fn custom_clamp(&self, ceil: bool, floor: bool) -> Self;
}

impl Rounding for f32 {
    fn custom_clamp(&self, ceil: bool, floor: bool) -> Self {
        match (ceil, floor) {
            (true, false) => self.ceil(),
            (false, true) => self.floor(),
            _ => self.round(),
        }
    }
}

pub fn convert_video_to_ascii() {}

pub fn render_html(res: String) -> String {
    let mut res_div = String::new();
    for line in res.lines() {
        res_div += &format!("<div class='barcode'>{}</div>", line);
    }

    let html = format!(
        r#"<!DOCTYPE html>
<html lang="en">
<head>
<style>

.barcode {{
    font-family:Courier;
    white-space:pre; 
    line-height: 0.7;
    letter-spacing: 1.6px;
    color: white;
    background-color: black;

}}

</style>
  <meta charset="utf-8">
  <title></title>
</head>
<body>
{res_div}
</body>
</html>"#
    );
    html
}

// FIX: too many arguments needs refractoring
pub fn convert_image_to_ascii(
    path: &Path,
    new_height: u32,
    ascii_chars: Style,
    color: bool,
    quantization_factor: Option<u32>,
    brighten: bool,
    floor: bool,
    invert: bool,
) -> String {
    let img = image::open(path).expect("Failed to open image");
    let mut ascii_chars = ascii_chars.chars();

    if invert {
        ascii_chars.reverse();
    }

    let (width, height) = img.dimensions();

    // resize to custom ratio
    let new_width = width * new_height / height;
    let resized = resize(&img, new_width, new_height, Lanczos3);
    let mut final_image = String::new();

    if color {
        let colored_image = &resized;
        for y in 0..new_height {
            for x in 0..new_width {
                let pixel = colored_image.get_pixel(x, y).0;
                let (mut r, mut g, mut b) = (pixel[0] as f32, pixel[1] as f32, pixel[2] as f32);
                let lum = 0.2126 * r + 0.7152 * g + 0.0722 * b;
                let mapped_index = (lum / (256.0_f32) * ascii_chars.len() as f32)
                    .custom_clamp(brighten, floor) as usize;
                let ascii_char = ascii_chars[mapped_index.min(ascii_chars.len() - 1)];

                if let Some(quantization_factor) = quantization_factor {
                    let quantization_distance = 256.0 / quantization_factor as f32;
                    r = (r / (quantization_distance)).custom_clamp(brighten, floor)
                        * quantization_distance;
                    g = (g / (quantization_distance)).custom_clamp(brighten, floor)
                        * quantization_distance;
                    b = (b / (quantization_distance)).custom_clamp(brighten, floor)
                        * quantization_distance;
                }

                final_image.push_str(&format!(
                    r#"<span style='color: rgb({},{},{})'>{}</span>"#,
                    r, g, b, ascii_char
                ));
            }
            final_image.push('\n');
        }
        final_image
    } else {
        let mut grayscaled_image: ImageBuffer<Luma<u8>, Vec<u8>> = grayscale(&resized);

        for y in 0..new_height {
            for x in 0..new_width {
                let pixel = grayscaled_image.get_pixel_mut(x, y).0[0];
                let mapped_index = (((pixel as f32).powf(1.7) / (256.0_f32).powf(1.7))
                    * ascii_chars.len() as f32)
                    .floor() as usize;
                // return 9 if out of bounds
                let ascii_char = ascii_chars[mapped_index.min(ascii_chars.len() - 1)];
                final_image.push(ascii_char);
            }
            final_image.push('\n');
        }

        final_image
    }
}

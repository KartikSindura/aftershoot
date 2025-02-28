use clap::ValueEnum;
use image::{
    GenericImageView, ImageBuffer, Luma,
    imageops::{FilterType::Lanczos3, grayscale, resize},
};
use std::{fs, path::Path};

#[derive(ValueEnum, Debug, Clone)] // ArgEnum here
#[clap(rename_all = "kebab_case")]
pub enum AsciiChars {
    Acerola,
    Me,
    More,
}

impl AsciiChars {
    pub fn return_ascii(&self) -> &[char] {
        match self {
            Self::Acerola => &[' ', '.', ';', 'c', 'o', 'P', 'O', '?', '@', '█'],
            Self::Me => &[' ', '.', '-', '+', '*', '%', '#', '&', '@', '█'],
            Self::More => &[' ', '.', '-', '+', '*', '%', '#', '?', '&', '@', '█'],
        }
    }
}

pub fn convert_image_to_ascii(path: &Path, new_height: u32, ascii_chars: AsciiChars) -> String {
    let img = image::open(path).expect("Failed to open image");
    let ascii_chars = ascii_chars.return_ascii();

    let (width, height) = img.dimensions();
    // let new_width = width / 10;
    // let new_height = height / 10;
    let new_width = width * new_height / height;
    dbg!(new_height, new_width);

    let resized = resize(&img, new_width, new_height, Lanczos3);
    let mut grayscaled_image: ImageBuffer<Luma<u8>, Vec<u8>> = grayscale(&resized);

    let mut final_image = String::new();

    for y in 0..new_height {
        for x in 0..new_width {
            let pixel = grayscaled_image.get_pixel_mut(x, y).0[0];
            let mapped_index = (((pixel as f32).powf(1.7) / (255.0_f32).powf(1.7))
                * ascii_chars.len() as f32)
                .floor() as usize;
            // return 9 if out of bounds
            let ascii_char = ascii_chars[mapped_index.min(ascii_chars.len() - 1)];
            final_image.push(ascii_char);
        }
        final_image.push('\n');
    }

    fs::write("final_image.txt", &final_image).unwrap();
    final_image
}


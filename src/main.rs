use aftershoot::convert_image_to_ascii;
use aftershoot::render_html;
use clap::{ArgAction, Parser};

use aftershoot::Style;
use std::fs;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Image path
    #[arg(short, long)]
    path: PathBuf,
    /// Output path
    #[arg(short, long)]
    out: PathBuf,
    /// Output image height
    #[arg(short, long, default_value_t = 256)]
    height: u32,
    /// Ascii character set
    // #[arg(value_enum, short, long, default_value_t = Some(Style::Acerola))]
    #[command(subcommand)]
    style: Option<Style>,
    /// Enable color mode
    #[arg(short, long, action = ArgAction::SetTrue)]
    color: bool,
    #[arg(short, long, requires = "color")]
    /// Quantize colors
    quant: Option<u32>,
    #[arg(short, long, action = ArgAction::SetTrue)]
    /// Ceil quantized colors
    brighten: bool,
    #[arg(short, long, action = ArgAction::SetTrue)]
    /// Floor quantized colors
    floor: bool,
    #[arg(short, long, action = ArgAction::SetTrue)]
    invert: bool,
}

fn main() {
    let args = Args::parse();
    let res = convert_image_to_ascii(
        &args.path,
        args.height,
        args.style.unwrap_or(Style::Acerola),
        args.color,
        args.quant,
        args.brighten,
        args.floor,
        args.invert,
    );

    let html = render_html(res);
    fs::write(args.out, html).unwrap();
}

use aftershoot::convert_image_to_ascii;
use clap::Parser;

use aftershoot::AsciiChars;
use std::fs;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    path: PathBuf,
    #[arg(short, long)]
    out: PathBuf,
    #[arg(short, long, default_value_t = 256)]
    iheight: u32,
    #[arg(value_enum, short, long, default_value_t = AsciiChars::Acerola)]
    chars: AsciiChars,
}

fn main() {
    let args = Args::parse();
    let res = convert_image_to_ascii(&args.path, args.iheight, args.chars);
    let mut res2 = String::new();
    for line in res.lines() {
        res2 += &format!("<div class='barcode'>{}</div>", line);
    }
    let html = format!(
        r#"<!DOCTYPE html>
<html lang="en">
<head>
  <meta charset="utf-8">
  <title></title>
  <link href="style.css" rel="stylesheet" />
</head>
<body>
{res2}
</body>
</html>"#
    );
    fs::write(args.out, html).unwrap();
}

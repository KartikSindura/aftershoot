## Usage

```
cargo run --release -- --path <IMAGE> --out <OUTPUT.html> [OPTIONS]

Required:
  -p, --path <PATH>       Input image path
  -o, --out <PATH>        Output HTML file path

Options:
  -h, --height <NUM>      Output image height [default: 256]
  -s, --style <STYLE>     Ascii character set [default: acerola] [possible values: custom, me, more]
  -c, --color             Enable color mode
  -q, --quant <NUM>       Quantize colors
  -b, --brighten          Ceil quantized colors
  -f, --floor             Floor quantized colors
  -i, --invert            Invert the character set
  -h, --help              Print help
  -V, --version           Print version
```
## Example
```
cargo run --release -- --path image.jpg --out ascii.html --color --floor --quant 8
```

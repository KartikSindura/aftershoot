## Usage

```
cargo run --release -- --path <IMAGE> --out <OUTPUT.html> [OPTIONS]

Required:
  -p, --path <PATH>       Input image path
  -o, --out <PATH>        Output HTML file path

Options:
  -i, --iheight <NUM>     Output height (default: 256)
  -s, --style <STYLE>     Character set (acerola|me|more)
  -c, --color             Enable colored output
  -q, --quant <NUM>       Quantize colors
  -b, --brighten          Ceil quantized colors
  -f, --floor             Floor quantized colors
```
## Example
```
cargo run --release -- --path image.jpg --out ascii.html --color --floor --quant 8
```
## Styles
- `acerola`: ` .;coPO?@█`
- `me`: ` .-+*%#&@█`
- `more`: ` .-+*%#?&@█`


## Usage

```
cargo run --release -- --path <IMAGE> --out <OUTPUT.html> [OPTIONS]

Required:
  -p, --path <PATH>       Input image path
  -o, --out <PATH>        Output HTML file path

Options:
  -h, --height <NUM>      Output height (default: 256)
  -c, --color             Enable colored output
  -q, --quant <NUM>       Quantize colors
  -b, --brighten          Ceil quantized colors
  -f, --floor             Floor quantized colors
  -i, --invert            Invert the character set

Commands:
  acerola                  .;coPO?A█
  me                       .-*%#&@█
  more                     .-*%#?&@█
  custom                  Provide your own ASCII characters
  help     Print this message or the help of the given subcommand(s)
```
## Example
```
cargo run --release -- --path image.jpg --out ascii.html --color --floor --quant 8
```

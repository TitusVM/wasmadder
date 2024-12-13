# wasmadder

`wasmadder` is a tiny command-line tool for appending data to custom sections in WebAssembly binaries.

## Features

- Append data to a custom section in a WebAssembly binary
- Specify input and output files

## Installation

Build from source using [Cargo](https://doc.rust-lang.org/cargo/):

```bash
git clone https://github.com/yourusername/wasmadder.git
cd wasmadder
cargo build --release
```

## Usage

```bash
wasmadder append --input <input.wasm> --data <data.bin> --section <section_name> --output <output.wasm>
```

### Arguments

- `-i`, `--input`: Input WebAssembly binary file (required)
- `-d`, `--data`: Data file to be appended (required)
- `-s`, `--section`: Custom section name (required)
- `-o`, `--output`: Output WebAssembly file (required)

### Example

```bash
wasmadder append \
  --input input.wasm \
  --data data.bin \
  --section my_custom_section \
  --output output.wasm
```

This command appends the contents of 

data.bin

 into the custom section `my_custom_section` of 

input.wasm

 and saves the result as 

output.wasm

.

## License

This project is licensed under the MIT License or the Apache License 2.0, at your option.
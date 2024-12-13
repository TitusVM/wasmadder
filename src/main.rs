use clap::{Arg, Command};
use std::path::PathBuf;

fn main() {
    let matches = Command::new("Wasmadder")
        .version("0.1.0")
        .about("CLI tool to append data to a custom section in a WebAssembly binary")
        .subcommand(
            Command::new("append")
                .about("Append data to a custom section in a WebAssembly binary")
                .arg(
                    Arg::new("input")
                        .short('i')
                        .long("input")
                        .help("Input WebAssembly binary file")
                        .required(true)
                        .value_parser(clap::value_parser!(PathBuf)),
                )
                .arg(
                    Arg::new("data")
                        .short('d')
                        .long("data")
                        .help("Data file to be appended")
                        .required(true)
                        .value_parser(clap::value_parser!(PathBuf)),
                )
                .arg(
                    Arg::new("section")
                        .short('s')
                        .long("section")
                        .help("Custom section name")
                        .required(true),
                )
                .arg(
                    Arg::new("output")
                        .short('o')
                        .long("output")
                        .help("Output file")
                        .required(true)
                        .value_parser(clap::value_parser!(PathBuf)),
                ),
        )
        .get_matches();

    if let Some(sub_matches) = matches.subcommand_matches("append") {
        let input_file = sub_matches.get_one::<PathBuf>("input").unwrap();
        let data_file = sub_matches.get_one::<PathBuf>("data").unwrap();
        let section_name = sub_matches.get_one::<String>("section").unwrap();
        let output_file = sub_matches.get_one::<PathBuf>("output").unwrap();

        handle_append(input_file, data_file, section_name, output_file);
    } else {
        eprintln!("No subcommand was used");
    }
}

fn handle_append(
    input: &PathBuf,
    data: &PathBuf,
    section: &str,
    output: &PathBuf,
) {
    let mut wasm_bytes = std::fs::read(input).expect("Failed to read input file");
    let data_bytes = std::fs::read(data).expect("Failed to read data file");

    wasm_gen::write_custom_section(&mut wasm_bytes, section, &data_bytes);
    std::fs::write(output, wasm_bytes).expect("Failed to write output file");
}
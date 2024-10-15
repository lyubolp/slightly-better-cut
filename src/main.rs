mod cli;
mod cut;
mod range_parser;

use std::fs::File;
use std::io::{self, BufRead};

use cut::{cut_line_with_bytes, cut_line_with_characters, cut_line_with_delimiter};
use range_parser::parse_range;

fn handle_bytes(bytes: Vec<u8>) -> String {
    match String::from_utf8(bytes.clone()) {
        Ok(result) => result,
        Err(_) => {
            let string_representation: Vec<String> =
                bytes.iter().map(|byte| byte.to_string()).collect();
            string_representation.join("0x")
        }
    }
}
fn main() {
    let args = cli::build_cli().get_matches();

    let default_file = String::from("-");
    let file_path = args.get_one::<String>("FILE").unwrap_or(&default_file);

    if *file_path == default_file {
        unimplemented!()
    }

    let file = File::open(file_path).unwrap();
    let reader = io::BufReader::new(file);

    let lines: Vec<String> = reader.lines().map(|l| l.unwrap()).collect();

    let actions = (
        args.get_one::<String>("bytes"),
        args.get_one::<String>("characters"),
        args.get_one::<String>("fields"),
    );

    let delimiter = args.get_one::<String>("delimiter").unwrap().clone();

    for line in lines {
        let n = line.split(&delimiter).count();

        let output = match actions {
            (Some(fields), _, _) => {
                let ranges = parse_range(fields, n);

                match ranges {
                    Ok(ranges) => {
                        let items: Vec<String> = ranges
                            .iter()
                            .map(|range| cut_line_with_bytes(&line, *range))
                            .map(|bytes| handle_bytes(bytes))
                            .collect();

                        items.join(" ")
                    }
                    Err(error) => error,
                }
            }
            (_, Some(fields), _) => {
                let ranges = parse_range(fields, n);

                match ranges {
                    Ok(ranges) => {
                        let items: Vec<String> = ranges
                            .iter()
                            .map(|range| cut_line_with_characters(&line, *range).iter().collect())
                            .collect();

                        items.join(" ")
                    }
                    Err(error) => error,
                }
            }
            (_, _, Some(fields)) => {
                let ranges = parse_range(fields, n);

                match ranges {
                    Ok(ranges) => {
                        let items: Vec<String> = ranges
                            .iter()
                            .map(|range| {
                                cut_line_with_delimiter(&line, *range, delimiter.clone()).join(" ")
                            })
                            .collect();

                        items.join(" ")
                    }
                    Err(error) => error,
                }
            }
            _ => unreachable!(),
        };
        println!("{}", output);
    }
}

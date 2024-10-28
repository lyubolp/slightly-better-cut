mod cli;
mod cut;
mod range_parser;

use std::fs::File;
use std::io::{self, BufRead};

use cut::{cut_line_with_bytes, cut_line_with_characters, cut_line_with_delimiter};
use range_parser::parse_range;

#[derive(Clone, Copy)]
pub enum CutType {
    BYTES,
    CHARACTERS,
    FIELDS,
}

fn cut_line(
    cut_type: CutType,
    ranges: Vec<range_parser::Range>,
    line: String,
    delimiter: &String,
    output_delimiter: &String,
) -> String {
    let ranges_iter = ranges.iter();

    let items: Vec<String> = match cut_type {
        CutType::BYTES => ranges_iter
            .map(|range| cut_line_with_bytes(&line, *range))
            .map(|items| items.join(&output_delimiter))
            .collect(),
        CutType::CHARACTERS => ranges_iter
            .map(|range| cut_line_with_characters(&line, *range))
            .map(|items| items.join(&output_delimiter))
            .collect(),
        CutType::FIELDS => ranges_iter
            .map(|range| cut_line_with_delimiter(&line, *range, delimiter.clone()))
            .map(|items| items.join(&output_delimiter))
            .collect(),
    };

    items.join(" ")
}

fn main() {
    let args = cli::build_cli().get_matches();

    let default_file = String::from("-");
    let file_path = args.get_one::<String>("FILE").unwrap_or(&default_file);

    if *file_path == default_file {
        // TODO - stdin implementation
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

    let cut_information = match actions {
        (Some(fields), _, _) => (CutType::BYTES, fields),
        (_, Some(fields), _) => (CutType::CHARACTERS, fields),
        (_, _, Some(fields)) => (CutType::FIELDS, fields),
        _ => unreachable!(),
    };

    let delimiter = args.get_one::<String>("delimiter").unwrap().clone();

    for line in lines {
        let (cut_type, fields) = cut_information;
        let n = line.split(&delimiter).count();
        let ranges = parse_range(fields, n);

        let output = match ranges {
            Ok(ranges) => cut_line(cut_type, ranges, line, &delimiter, &String::from(" ")),
            Err(error) => error,
        };
        println!("{}", output);
    }
}

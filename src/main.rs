mod cli;
mod cut;
mod range_parser;

use std::fs::File;
use std::io::{self, Read};
use std::process::exit;

use cut::{cut_line_with_bytes, cut_line_with_characters, cut_line_with_delimiter};
use range_parser::parse_range;

fn main() {
    let args = cli::build_cli().get_matches();

    let default_file = String::from("-");
    let file_path = args.get_one::<String>("FILE").unwrap_or(&default_file);

    let content_result = read_content(file_path, &default_file);

    if content_result.is_err() {
        exit(1);
    }
    let content = content_result.unwrap();


    let is_using_nul_as_line_delimiter = args.get_flag("zero_terminated");

    let pattern_to_split = if !is_using_nul_as_line_delimiter {
        '\n'
    }
    else {
        0 as char
    };

    let mut lines: Vec<String> = content.split(pattern_to_split).map(|item| String::from(item)).collect();

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

    let is_showing_complement = args.get_flag("complement");
    let is_showing_only_delimited_lines = args.get_flag("only_delimited");

    let output_delimiter = match args.get_one::<String>("output_delimiter") {
        Some(delimiter) => delimiter.clone(),
        None => delimiter.clone()
    };


    cut_lines(&mut lines, cut_information, is_showing_only_delimited_lines, &delimiter, &output_delimiter, is_showing_complement);
}

fn cut_lines(lines: &mut Vec<String>, cut_information: (CutType, &String), is_showing_only_delimited_lines: bool, delimiter: &String, output_delimiter: &String, is_showing_complement: bool) {
    if lines.last().is_some_and(|line| line == "") {
        lines.pop();
    };

    for line in lines {
        let (cut_type, fields) = cut_information;

        if cut_type == CutType::FIELDS && is_showing_only_delimited_lines && !line.contains(delimiter) {
            continue;
        }
        let n = line.split(delimiter).count();
        let ranges = parse_range(fields, n);

        let output = match ranges {
            Ok(ranges) => cut_line(cut_type, ranges, line, delimiter, output_delimiter, is_showing_complement),
            Err(error) => error,
        };
        println!("{}", output);
    }
}

fn read_content(file_path: &String, default_file: &String) -> Result<String, String> {
    let mut buffer = String::new();
    if *file_path == *default_file {
        let mut buffer = String::new();
        match io::stdin().read_to_string(&mut buffer) {
            Ok(_) => Ok(buffer),
            Err(_) => Err(String::from("Can't read input file"))
        }
    }
    else {
        // Read from the file
        let file = File::open(file_path).unwrap();
        let mut reader = io::BufReader::new(file);
        match reader.read_to_string(&mut buffer) {
            Ok(_) => Ok(buffer),
            Err(_) => Err(String::from("Error when reading from stdin"))
        }
    }
}

#[derive(Clone, Copy, PartialEq, PartialOrd)]
pub enum CutType {
    BYTES,
    CHARACTERS,
    FIELDS,
}

fn cut_line(
    cut_type: CutType,
    ranges: Vec<range_parser::Range>,
    line: &String,
    delimiter: &str,
    output_delimiter: &String,
    is_showing_complement: bool,
) -> String {
    let ranges_iter = ranges.iter();

    let items: Vec<String> = match cut_type {
        CutType::BYTES => ranges_iter
            .map(|range| cut_line_with_bytes(line, *range, is_showing_complement))
            .map(|items| items.join(output_delimiter))
            .collect(),
        CutType::CHARACTERS => ranges_iter
            .map(|range| cut_line_with_characters(line, *range, is_showing_complement))
            .map(|items| items.join(output_delimiter))
            .collect(),
        CutType::FIELDS => ranges_iter
            .map(|range| {
                cut_line_with_delimiter(line, *range, delimiter.to_owned(), is_showing_complement)
            })
            .map(|items| items.join(output_delimiter))
            .collect(),
    };

    items.join(output_delimiter)
}
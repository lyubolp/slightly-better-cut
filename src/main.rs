mod cut;
mod cli;
mod range_parser;

use std::fs::File;
use std::io::{self, BufRead};

use cut::cut::{cut_line_with_bytes, cut_line_with_characters, cut_line_with_delimiter};
use range_parser::parse_range;

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

    let actions = (args.get_one::<String>("bytes"), args.get_one::<String>("characters"), args.get_one::<String>("fields"));

    println!("{:?}", actions);
    let output = match actions {
        (Some(fields), _, _) => println!("Bytes {}", fields),
        (_, Some(fields), _) => println!("Characters {}", fields),
        (_, _, Some(fields)) => {
            println!("Fields {}", fields);
            let range = parse_range(fields);
            //cut_line_with_delimiter(line, range, delimiter)
        }
        _ => unreachable!()
    };
}

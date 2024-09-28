mod cut;
mod cli;

use clap::Parser;

fn main() {
    let args = cli::build_cli().get_matches();

    let default_file = String::from("-");
    let file_path = args.get_one::<String>("FILE").unwrap_or(&default_file);

    println!("Splitting {} on:", file_path);

    let actions = (args.get_flag("bytes"), args.get_flag("characters"), args.get_one::<String>("fields"));

    match actions {
        (true, _, _) => println!("Bytes"),
        (_, true, _) => println!("Characters"),
        (_, _, Some(fields)) => println!("Fields {}", fields),
        _ => unreachable!()
    };
}

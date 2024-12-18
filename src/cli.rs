use clap::{arg, Command};
use clap::{ArgAction, ArgGroup};

pub fn build_cli() -> Command {
    Command::new("sbcut")
        .version("0.1")
        .about("Slightly better cut")
        .arg(arg!(-b --bytes <LIST>).allow_negative_numbers(true))
        .arg(arg!(-c --characters <LIST>).allow_negative_numbers(true))
        .arg(
            arg!(-d --delimiter <DELIM>)
                .requires("fields")
                .default_value("\t"),
        )
        .arg(arg!(-f --fields <LIST>).allow_negative_numbers(true))
        .arg(arg!(--complement).action(ArgAction::SetTrue))
        .arg(arg!(-s --only_delimited).action(ArgAction::SetTrue))
        .arg(arg!(--always_show_no_delimited_lines).action(ArgAction::SetTrue)) // If a line is not delimited, always show it in full (cut compatibility)
        .arg(arg!(--output_delimiter <DELIM>))
        .arg(arg!(-z --zero_terminated).action(ArgAction::SetTrue))
        .arg(arg!([FILE]).default_value("-"))
        .group(
            ArgGroup::new("action")
                .required(true)
                .args(["bytes", "characters", "fields"]),
        )
}

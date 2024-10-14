use clap::{arg, Command};
use clap::{ArgAction, ArgGroup, Parser};

pub fn build_cli() -> Command {
    Command::new("sbcut")
        .version("0.1")
        .about("Slightly better cut")
        .arg(arg!(-b --bytes <LIST>))
        .arg(arg!(-c --characters <LIST>))
        .arg(
            arg!(-d --delimiter <DELIM>)
                .requires("fields")
                .default_value("\t"),
        )
        .arg(arg!(-f --fields <LIST>))
        .arg(arg!(--complement).action(ArgAction::SetTrue))
        .arg(arg!(-s - -only_delimited).action(ArgAction::SetTrue))
        .arg(arg!(--output_delimiter <DELIM>).default_value("\n"))
        .arg(arg!(-z - -zero_terminated).action(ArgAction::SetTrue))
        .arg(arg!([FILE]).default_value("-"))
        .group(
            ArgGroup::new("action")
                .required(true)
                .args(["bytes", "characters", "fields"]),
        )
}

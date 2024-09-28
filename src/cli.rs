use clap::Parser;

#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct CLI {
    #[arg(short, long, group="action")]
    pub bytes: bool,

    #[arg(short, long, group="action")]
    pub characters: bool,

    #[arg(short, long, default_value  = "\t")]
    pub delimiter: Option<char>,

    #[arg(short, long, group="action")]
    pub fields: bool,

    #[arg(long)]
    pub complement: bool,

    #[arg(short, long)]
    pub only_delimited: bool,

    #[arg(long, default_value  = "\n")]
    pub output_delimiter: Option<char>,

    #[arg(short, long)]
    pub zero_terminated: bool,

}
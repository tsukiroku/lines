use clap::Parser;

#[derive(Parser, Debug)]
#[clap(author)]
pub struct Args {
    #[clap(short, long, default_value = "./")]
    pub directory: String,

    #[clap(short, long)]
    pub file: Option<String>,

    #[clap(short, long)]
    pub ignore: Option<String>,
}

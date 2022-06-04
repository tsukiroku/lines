mod args_parse;
mod core;

use crate::{
    args_parse::Args,
    core::{lines, LinesOption},
};
use clap::Parser;

fn main() {
    let args = Args::parse();
    println!(
        "{}",
        lines(LinesOption {
            directory: args.directory,
            ignore: args.ignore,
        })
    );
}

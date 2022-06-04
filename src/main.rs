mod args_parse;
mod core;
mod util;

use crate::{
    args_parse::Args,
    core::{lines, read_lines, LinesOption, ResultLines},
    util::print,
};
use clap::Parser;

fn main() {
    let args = Args::parse();

    if let Some(file) = args.file {
        print(
            ResultLines {
                total: read_lines(file.clone()),
                files: 1,
            },
            Some(file),
        );
        return;
    }

    print(
        lines(LinesOption {
            directory: args.directory,
            ignore: args.ignore,
        }),
        None,
    );
}

use encoding_rs::UTF_8;
use encoding_rs_io::DecodeReaderBytesBuilder;
use std::{
    fs::{self, File},
    io::Read,
};

pub struct LinesOption<T> {
    pub directory: T,
    pub ignore: Option<T>,
}

pub fn read_lines(path: String) -> usize {
    let file = File::open(path).unwrap();
    let mut reader = DecodeReaderBytesBuilder::new()
        .encoding(Some(UTF_8))
        .build(file);
    let mut buffer = Vec::new();
    reader.read_to_end(&mut buffer).unwrap();
    String::from_utf8(buffer).unwrap().lines().count()
}

pub fn lines(options: LinesOption<String>) -> usize {
    let mut count: usize = 0;
    for file in fs::read_dir(options.directory).unwrap() {
        let path = file.unwrap().path();
        if let Some(ref ignore) = options.ignore {
            if (ignore.split(',').map(|s| s.to_string().replace(" ", "")))
                .collect::<Vec<String>>()
                .contains(&path.to_str().unwrap().to_string())
            {
                continue;
            }
        }
        if path.is_dir() {
            count += lines(LinesOption {
                directory: path.to_str().unwrap().to_string(),
                ignore: None,
            });
        } else {
            count += read_lines(path.to_str().unwrap().to_string());
        };
    }
    count
}

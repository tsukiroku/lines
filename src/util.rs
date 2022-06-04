use crate::core::ResultLines;

pub fn print(options: ResultLines<usize>, l: Option<String>) {
    println!(
        "{} lines in {} files{}",
        options.total,
        options.files,
        if let Some(m) = l {
            format!(" ({}).", m)
        } else {
            String::from(".")
        }
    );
}

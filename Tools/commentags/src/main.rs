mod tag_finder;
mod commentags;

use std::{path::PathBuf, error::Error, fs};

use clap::{Parser, arg};
use tag_finder::TagFinder;

#[derive(Parser, Debug)]
/// Applies Commentags to a file.
struct CmdArgs {
    /// Target file.
    file: PathBuf,
    /// Fill @template placeholders.
    /// Follows the format `key=value`.
    #[arg(short, long, value_parser = parse_key_value::<String, String>)]
    fill: Vec<(String, String)>,
}

/// Based on https://github.com/clap-rs/clap/blob/master/examples/typed-derive.rs
fn parse_key_value<TKey, TVal>(s: &str) -> Result<(TKey, TVal), Box<dyn Error + Send + Sync + 'static>>
where
    TKey: std::str::FromStr,
    TKey::Err: Error + Send + Sync + 'static,
    TVal: std::str::FromStr,
    TVal::Err: Error + Send + Sync + 'static,
{
    let delimiter_index = s
        .find("=")
        .ok_or_else(|| format!("Expected `{s}` to follow the `key=value` format"))?;
    Ok((s[..delimiter_index].parse()?, s[delimiter_index + 1..].parse()?))
}

fn main() {
    let args = CmdArgs::parse();

    if !args.file.is_file() {
        panic!("Invalid file path: {:?}", args.file);
    }

    let finder = TagFinder::from_file_type(&args.file)
        .expect("Expected the file to have a valid extension");
    let target = fs::read_to_string(args.file)
        .expect("Expected to be able to read the file");

    finder.find_tags(&target);
}

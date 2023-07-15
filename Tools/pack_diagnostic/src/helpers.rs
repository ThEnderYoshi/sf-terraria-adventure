/*!
 * Generic gelper functions.
 */

use std::{path::PathBuf, str::FromStr, fmt::Debug, io::{self, Write}};

use ansi_term::Color;

use crate::slop::{SLOP, SLOPValue};

/// Converts the path buf to a key name used in the reference SLOPs.
pub fn path_buf_to_key_name(path_buf: &PathBuf) -> String {
    let mut key_name = String::from("/");
    key_name.push_str(path_buf.to_str()
        .expect("Expected path to be a valid UTF-8 string."));
    key_name.replace("\\", "/")
}

/// Gets a string KV and parses it into the provided type.
pub fn get_parsed_string<T>(slop: &SLOP, key: &str) -> Option<T>
where
    T: FromStr,
    <T as FromStr>::Err: Debug,
{
    if let Some(value) = slop.get(key) {
        if let SLOPValue::String(value) = value {
            Some(value.parse()
                .expect(&format!("Expected '{key}' to parse to a specific value")))
        } else {
            panic!("Expected '{key}' to be a string");
        }
    } else {
        None
    }
}

/// Prints a simple progress update.
///
/// Once you're done updating, use [`println!()`] to finish up the line.
///
/// [`println!()`]: println!
pub fn update_progress(progress: u32) {
    print!(
        "\rFound {} items. {}",
        Color::Yellow.bold().paint(&progress.to_string()),
        Color::Black.paint("(Updates every 100)"),
    );
    io::stdout().flush().unwrap();
}

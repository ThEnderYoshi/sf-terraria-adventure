pub mod images;
pub mod texts;

use ansi_term::Color;
//use std::{path::{Path, PathBuf}, fs, io};

pub fn print_divider(message: &str) {
    println!(
        "{}",
        Color::Black.paint(format!("---- {message} ----")),
    );
}

// fn crawl<F>(content_dir: &Path, subdir: &str, condition: F) -> io::Result<Option<(u32, u32)>>
// where
//     F: Fn(PathBuf) -> bool,
// {
//     let dir = content_dir.join(subdir);
//     let subdir = Color::Blue
//         .paint(format!("Content/{subdir}/"));

//     if !dir.is_dir() {
//         println!("No {subdir} directory, skipping!");
//         return Ok(None);
//     }

//     print_divider(&format!("Scanning {subdir}"));

//     let mut valid_item_count = 0;
//     let mut invalid_item_count = 0;

//     for entry in fs::read_dir(dir)? {
//         let path = entry?.path();

//         if path.is_file() && condition(path) {
//             valid_item_count += 1;
//         } else {
//             invalid_item_count += 1;
//         }
//     }

//     print_divider(&format!("Finished scanning {subdir}"));
//     Ok(Some((valid_item_count, invalid_item_count)))
// }

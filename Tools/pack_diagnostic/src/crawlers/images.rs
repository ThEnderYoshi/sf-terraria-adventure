/*!
 * Crawls over the Images/ directory, looking for `.png` files who's
 * names don't start with `!`.
 */

use ansi_term::{ANSIStrings, Color};
use clap::builder::OsStr;
use walkdir::WalkDir;
use std::{io, path::{Path, PathBuf}, cmp::Ordering};

use crate::{slop::SLOP, helpers};

use super::print_divider;

pub fn crawl_images(content_dir: &Path, slop_dir: &PathBuf) -> io::Result<()> {
    print_divider("Scanning your Resource Pack...");

    let reference = load_reference(slop_dir, "images.slop")?;
    let desktop_ini = OsStr::from("desktop.ini");

    let total_count: u32 = helpers::get_parsed_string(&reference, "!count")
        .expect("Expected SLOP to have a '!count' KV");

    let mut images_dir = content_dir.clone().to_path_buf();
    images_dir.push(Path::new("Images"));
    let mut data = CrawlData::new(total_count);

    for entry in WalkDir::new(images_dir.clone()) {
        let entry = entry?;
        let path = entry.path().to_path_buf();

        if path.is_dir() || path.file_name() == Some(&desktop_ini) {
            continue;
        }
        if is_item_valid(&path, &images_dir, &reference) {
            data.valid_count += 1;
        } else {
            data.invalid_count += 1;
            data.invalid_items.push(String::from(path.to_str().unwrap()));
        }

        let joined_count = data.valid_count + data.invalid_count;
        if joined_count % 100 == 0 {
            helpers::update_progress(joined_count);
        }
    }

    helpers::update_progress(data.valid_count + data.invalid_count);
    println!();
    print_divider("Scanned your Resource Pack.");

    report_findings(&data);
    report_invalid_items(&data);
    println!();
    Ok(())
}

struct CrawlData {
    total_count: u32,
    valid_count: u32,
    invalid_count: u32,
    invalid_items: Vec<String>,
}

impl CrawlData {
    fn new(total_count: u32) -> Self {
        Self {
            total_count,
            valid_count: 0,
            invalid_count: 0,
            invalid_items: Vec::new(),
        }
    }
}

fn is_item_valid(path: &PathBuf, images_dir: &Path, reference: &SLOP) -> bool {
    match path.extension() {
        Some(extension) => {
            if extension.to_str() != Some("png") {
                false
            } else {
                let path = path.strip_prefix(images_dir)
                    .expect("Expected path to be a child of content_dir")
                    .to_path_buf();
                is_item_in_reference(&path, reference)
            }
        },
        None => false,
    }
}

fn is_item_in_reference(path: &PathBuf, reference: &SLOP) -> bool {
    let dir = path.parent()
        .expect("Expected file path to have a parent")
        .to_path_buf();
    let dir = &helpers::path_buf_to_key_name(&dir);
    let path = path.file_name()
        .expect("Expected path to not end in a root.")
        .to_str()
        .expect("Expected path to be a valid UTF-8 string");

    //println!("{dir:?}\t{path:?}");

    if let Some(list) = reference.get_list(dir) {
        list.iter().any(|item| item == path)
    } else {
        false
    }
}

fn load_reference(reference_dir: &PathBuf, file_name: &str) -> io::Result<SLOP> {
    let mut path = reference_dir.clone();
    path.push(Path::new(file_name));
    SLOP::from_file(path)
}

fn report_findings(data: &CrawlData) {
    let valid_count = data.valid_count as f32;
    let total_percent = valid_count / data.total_count as f32 * 100.0;
    let total_percent = format!("{total_percent:.2}");
    let milestone_percent = (data.valid_count % 1000) as f32 / 10.0;

    let green = Color::Green.bold();
    let gray = Color::Black;
    let dash = Color::Blue
        .paint("-");

    println!(
        "Found {}/{} images. {}",
        green.paint(data.valid_count.to_string()),
        green.paint(data.total_count.to_string()),
        ANSIStrings(&[
            gray.paint("("),
            Color::Green.paint(total_percent),
            gray.paint("% of the way!)"),
        ]),
    );
    println!(
        "{dash} {}% to the next 1000!",
        green.paint(milestone_percent.to_string()),
    );
}

fn report_invalid_items(data: &CrawlData) {
    let count = data.invalid_count;
    let dash = if count == 0 { Color::Blue } else { Color::Red }
        .paint("-");

    let count = Color::Red.bold().paint(count.to_string());

    match data.invalid_count.cmp(&1) {
        Ordering::Less => {
            println!("{dash} No invalid items found!");
            return;
        },
        Ordering::Equal => println!("{dash} Found {count} invalid item."),
        Ordering::Greater => println!("{dash} Found {count} invalid items."),
    }
    for item in &data.invalid_items {
        println!(
            "  {dash} {}",
            item.replace("\\", "/"),
        );
    }
}

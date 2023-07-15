/*!
 * Crawls through the language files.
 */

use std::{path::PathBuf, ffi::OsString, error::Error, cmp::Ordering};

use ansi_term::{Color, ANSIStrings};
use csv;
use walkdir::WalkDir;

use crate::helpers;


pub fn crawl_texts(texts_dir: &PathBuf) -> Result<(), Box<dyn Error>> {
    super::print_divider("Scanning Localization files...");

    let desktop_ini = OsString::from("desktop.ini");
    let walk_dir = WalkDir::new(texts_dir.clone()).max_depth(1);
    let mut crawl_data = CrawlData::new();
    
    for entry in walk_dir {
        let entry = entry?;
        let path = entry.path().to_path_buf();

        if should_ignore_file(&path, &desktop_ini) {
            continue;
        }

        crawl_csv(&path, &mut crawl_data)?;
        update_progress(&crawl_data);
    }
    
    helpers::update_progress(crawl_data.valid_count + crawl_data.invalid_items.len() as u32);
    println!();
    super::print_divider("Scanned Localization files.");
    
    report_findings(&crawl_data);
    println!();
    Ok(())
}

fn should_ignore_file(path: &PathBuf, desktop_ini: &OsString) -> bool {
    if path.is_dir() {
        true
    } else if let Some(file_name) = path.file_name() {
        if file_name == desktop_ini {
            true
        } else {
            match file_name.to_str() {
                Some(file_name) => &file_name[0..1] == "!",
                None => true,
            }
        }
    } else {
        true
    }
}

fn crawl_csv(file_path: &PathBuf, crawl_data: &mut CrawlData) -> Result<(), Box<dyn Error>> {
    let mut reader = csv::Reader::from_path(file_path)?;

    for result in reader.records() {
        let record = result?;
        let key = record.get(0)
            .expect("Expected record to have at least one field");

        if key.get(0..1) != Some("#") {
            crawl_data.valid_count += 1;
        }
    }

    Ok(())
}

fn report_findings(crawl_data: &CrawlData) {
    let valid_count = crawl_data.valid_count;

    println!(
        "Found {}/{} text entries. {}",
        Color::Green.bold().paint(valid_count.to_string()),
        Color::Green.paint("NA"),
        ANSIStrings(&[
            Color::Black.paint("("),
            Color::Green.paint("NA"),
            Color::Black.paint("% of the way!)"),
        ]),
    );
    report_errors(crawl_data);
}

fn report_errors(crawl_data: &CrawlData) {
    let count = crawl_data.invalid_items.len();
    let dash = (if count == 0 { Color::Blue } else { Color::Red })
        .paint("-");

    let count_text = Color::Red.bold()
        .paint(count.to_string());

    match count.cmp(&1) {
        Ordering::Less => {
            println!("{dash} No invalid entries found!");
            return;
        },
        Ordering::Equal => println!("{dash} Found {count_text} invalid entry."),
        Ordering::Greater => println!("{dash} Found {count_text} invalid entries."),
    }
    for item in &crawl_data.invalid_items {
        println!("  {dash} {}", item.replace("\\", "/"));
    }
}

fn update_progress(crawl_data: &CrawlData) {
    let joined_count = crawl_data.valid_count + crawl_data.invalid_items.len() as u32;
    if joined_count % 100 == 0 {
        helpers::update_progress(joined_count);
    }
}

struct CrawlData {
    valid_count: u32,
    invalid_items: Vec<String>,
}

impl CrawlData {
    fn new() -> Self {
        Self { valid_count: 0, invalid_items: Vec::new() }
    }
}

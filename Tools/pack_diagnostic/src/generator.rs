/*!
 * Generates a set of SLOP files from images extracted from TConvert.
 */

use std::{io, path::{PathBuf, Path}, fs};

use walkdir::WalkDir;

use crate::{slop::{SLOP, SLOPValue}, helpers, crawlers::print_divider};

const IMAGES_SLOP_FILE_NAME: &str = "images.slop";

pub fn generate_from_extracted_files(input_dir: &PathBuf, output_dir: &PathBuf) -> io::Result<()> {
    let mut images = SLOP::new();
    let mut image_count: u32 = 0;

    print_divider("Crawling through extracted files...");

    for entry in WalkDir::new(input_dir.clone()) {
        let entry = entry?;
        let path = entry.path();
        let mut path_dir = path.to_path_buf();

        if path_dir.is_dir() {
            continue;
        }

        let file_name = path_dir.file_name()
            .expect("Expected path to end in a file name")
            .to_str()
            .expect("Expected file name to be a valid UTF-8 string.")
            .to_owned();

        path_dir = sanitize_path(&mut path_dir, &input_dir);
        let path_dir = helpers::path_buf_to_key_name(&mut path_dir);
        register_item(&mut images, &path_dir, &file_name);

        image_count += 1;
        if image_count % 100 == 0 {
            helpers::update_progress(image_count);
        }
    }

    helpers::update_progress(image_count);
    println!();
    print_divider("Crawled through extracted files.");

    images.insert_str("!count", &image_count.to_string());

    print_divider("Writing findings to disk...");
    write_findings(&images, &output_dir);
    print_divider("Wrote findings to disk.");

    Ok(())
}

// Expects path_dir to lead to a file, and input dir to be a valid base
// for path_dir.
fn sanitize_path(path_dir: &mut PathBuf, input_dir: &PathBuf) -> PathBuf {
    path_dir.pop();
    path_dir.strip_prefix(&input_dir)
        .expect("Expected path_dir to be a valid base for input_dir")
        .to_path_buf()

}

fn register_item(slop: &mut SLOP, key: &str, item: &str) {
    if slop.contains_key(key) {
        slop.push_to_list_kv(key, String::from(item));
    } else {
        slop.insert(key, SLOPValue::List(vec![String::from(item)]));
    }
}

fn write_findings(slop: &SLOP, output_dir: &PathBuf) {
    let mut file_path = output_dir.clone();
    file_path.push(Path::new(IMAGES_SLOP_FILE_NAME));
    fs::write(file_path, slop.serialize())
        .expect("Failed to write to disk :) ");
}

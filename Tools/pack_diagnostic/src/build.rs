/*!
 * Creates a copy of the resource pack prepared for release.
 */

use std::{
    path::{PathBuf, Path},
    io::{
        self,
        Write,
    },
    fs, ffi::OsStr,
};

use lazy_static::lazy_static;
use regex::Regex;
use walkdir::WalkDir;

use crate::{slop::SLOP, crawlers::{self, images}};

macro_rules! path_vec {
    ($($path_str:expr),*) => {
        vec![$(PathBuf::from($path_str)),*]
    };
}

pub fn build_resource_pack(
    orig_root: &PathBuf,
    copy_root: &PathBuf,
    refs: &PathBuf,
) -> crate::GeneralResult<()> {
    prepare_copy_dir(&copy_root)?;

    //copy_root_dir(&orig_root, &copy_root)?;
    copy_images_dir(&orig_root, &copy_root, &refs)?;
    copy_localization_dir(&orig_root, &copy_root)?;
    copy_by_extension(
        &orig_root,
        &copy_root,
        false,
        "Content/Music",
        &vec!["ogg", "wav"],
    )?;
    copy_by_extension(&orig_root, &copy_root, true, "Content/Sounds", &vec!["xnb"])?;

    Ok(())
}

fn prepare_copy_dir(copy_root: &PathBuf) -> io::Result<()> {
    print!("Preparing output directory...");
    io::stdout().flush()?;
    fs::create_dir_all(copy_root)?;
    println!("Done.");
    Ok(())
}

// fn copy_root_dir(orig_root: &PathBuf, copy_root: &PathBuf) -> crate::GeneralResult<()> {
//     crawlers::print_divider("Copying <root>...");
//     let root_files = path_vec!["icon.png", "pack.json", "workshop.json"];
//     copy_files_if(
//         &orig_root,
//         &copy_root,
//         false,
//         |path| root_files.iter().any(|x| x == path),
//     )?;

//     Ok(())
// }

fn copy_images_dir(
    orig_root: &PathBuf,
    copy_root: &PathBuf,
    refs: &PathBuf,
) -> crate::GeneralResult<()> {
    crawlers::print_divider("Copying <root>/Content/Images...");

    let slop = SLOP::from_file(appended_copy(&refs, "images.slop"))?;
    let orig_images = appended_copy(&orig_root, "Content/Images");
    let copy_images = appended_copy(&copy_root, "Content/Images");

    copy_files_if(
        &orig_images,
        &copy_images,
        true,
        |path| {
            //HACK: We're adding <parent> to <path>, then removing it again in is_item_valid().
            //      Too lazy right now to make a relative version of the func.
            let path = appended_copy(&orig_images, path.to_str().unwrap());
            images::is_item_valid(&path, &orig_images, &slop)
        },
    )?;

    Ok(())
}

fn copy_localization_dir(orig_root: &PathBuf, copy_root: &PathBuf) -> crate::GeneralResult<()> {
    lazy_static! {
        //TODO: Include all language codes in regex.
        static ref RE_FILENAME: Regex = Regex::new(r"(?:en-US|pt-BR)-[\w]+\.(?:csv|json)").unwrap();
    }

    crawlers::print_divider("Copying <root>/Content/Localization...");
    let orig_localization = appended_copy(&orig_root, "Content/Localization");
    let copy_localization = appended_copy(&copy_root, "Content/Localization");

    copy_files_if(
        &orig_localization,
        &copy_localization,
        false,
        |path| if let Some(path) = path.to_str() {
            RE_FILENAME.is_match(path)
        } else {
            false
        },
    )?;

    Ok(())
}

fn copy_by_extension<P>(
    orig_root: &PathBuf,
    copy_root: &PathBuf,
    is_recursive: bool,
    path: P,
    extensions: &Vec<&str>,
) -> crate::GeneralResult<()>
where
    P: AsRef<Path>
{
    let path_str: &str = path.as_ref().to_str().unwrap();
    crawlers::print_divider(&format!("Copying <root>/{path_str}..."));
    let orig = appended_copy(orig_root, &path);
    let copy = appended_copy(copy_root, &path);

    copy_files_if(
        &orig,
        &copy,
        is_recursive,
        |path| extensions.iter().any(|x| Some(OsStr::new(x)) == path.extension()),
    )?;

    Ok(())
}

fn appended_copy<P>(parent: &PathBuf, suffix: P) -> PathBuf
where
    P: AsRef<Path>,
{
    let mut parent = parent.clone();
    parent.push(suffix);
    parent
}

fn copy_files_if<F>(
    from: &PathBuf,
    to: &PathBuf,
    is_recursive: bool,
    condition: F,
) -> crate::GeneralResult<()>
where
    F: for<'a> Fn(&'a PathBuf) -> bool,
{
    let mut valid_files: Vec<PathBuf> = vec![];
    let walk_dir =
        if is_recursive { WalkDir::new(from) } else { WalkDir::new(from).max_depth(1) };

    fs::create_dir_all(to)?;

    for entry in walk_dir {
        let entry = entry?;
        let path = entry
            .path()
            .to_path_buf()
            .strip_prefix(from)?
            .to_path_buf();

        if condition(&path) {
            valid_files.push(path);
        }
    }

    if valid_files.is_empty() {
        println!("No files in {from:?}!");
    } else {
        fs::create_dir_all(to)?;
        for path in valid_files {
            let original = appended_copy(from, &path);
            let copy = appended_copy(to, &path);

            if let Some(path) = copy.parent() {
                if !path.is_dir() {
                    fs::create_dir_all(path)?;
                }
            }
            fs::copy(original, copy)?;
        }
    }

    Ok(())
}

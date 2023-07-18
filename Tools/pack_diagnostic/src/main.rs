pub mod crawlers;
mod generator;
mod slop;
mod helpers;
mod build;
//mod user_interface;

use std::{io, path::{PathBuf, Path}, str::FromStr, error::Error};

use ansi_term::Style;
use clap::{arg, Parser};
use crawlers::{images, print_divider, texts};

type GeneralResult<T> = Result<T, Box<dyn Error>>;

fn get_default_arg_path() -> PathBuf {
    PathBuf::from_str(".").unwrap()
}

/// Diagnostic tool for Terraria Resource Packs.
#[derive(Parser)]
struct CliArgs {
    /// The action to be performed.
    /// See `README.md` for info.
    action: String,
    /// Input path directory. Not used by all commands.
    #[arg(short, long, default_value = get_default_arg_path().into_os_string())]
    input: PathBuf,
    /// Output path directory. Not used by all commands.
    #[arg(short, long, default_value = get_default_arg_path().into_os_string())]
    output: PathBuf,
    /// Reference path directory. Not used by all commands.
    #[arg(short, long, default_value = get_default_arg_path().into_os_string())]
    reference: PathBuf,
}

fn main() -> GeneralResult<()> {
    let args = CliArgs::parse();
    println!("Welcome to the Resource Pack Diagnostic Tool version 0.2.0");

    match args.action.as_str() {
        "gen" => generate_references(&args.input, &args.output)?,
        "scan" => scan_directory(&args.input, &args.output)?,
        "build" => build_resource_pack(&args.input, &args.output, &args.reference)?,
        action => panic!("Invalid action '{action}'. See README.md for a list"),
    }

    end_program();
    Ok(())
}

fn scan_directory(input_dir: &PathBuf, output_dir: &PathBuf) -> GeneralResult<()> {
    print_divider("OPERATION : Scan Directory");
    images::crawl_images(input_dir, output_dir)?;
    let mut texts_dir = input_dir.clone().to_path_buf();
    texts_dir.push(Path::new("Localization"));
    texts::crawl_texts(&texts_dir)?;
    Ok(())
}

fn generate_references(input_dir: &PathBuf, output_dir: &PathBuf) -> io::Result<()> {
    print_divider("OPERATION : Generate References");
    generator::generate_from_extracted_files(input_dir, output_dir)?;
    Ok(())
}

fn build_resource_pack(
    input_dir: &PathBuf,
    output_dir: &PathBuf,
    refs: &PathBuf
) -> GeneralResult<()> {
    print_divider("OPERATION : Build Resource Pack");
    build::build_resource_pack(input_dir, output_dir, refs)?;
    Ok(())
}

fn end_program() {
    print_divider(&Style::new().bold().paint("Diagnostic complete!").to_string());
}

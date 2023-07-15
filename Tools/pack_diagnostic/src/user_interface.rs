use ansi_term::Color;
use std::{io::{self, Write}, path::{Path, PathBuf}};

pub fn ask(question: &str) -> io::Result<String> {
    print!("{question}\n{} ", Color::Cyan.paint(">"));
    io::stdout().flush()?;

    let mut answer = String::new();
    io::stdin().read_line(&mut answer)?;
    Ok(answer)
}

pub fn ask_choice(question: &str, choices: &[&str]) -> io::Result<String> {
    let question = format_choice_question(question, &choices);
    let mut lowercase_choices: Vec<String> = Vec::new();

    for choice in choices {
        lowercase_choices.push(choice.to_lowercase());
    }

    loop {
        let answer = ask(&question)?.trim().to_lowercase();

        for choice in &lowercase_choices {
            if choice.eq_ignore_ascii_case(&answer) {
                return Ok(answer);
            }
        }
        println!("{}", Color::Yellow.paint("Invalid command."));
    }
}

pub fn ask_bool(question: &str, yes_is_default: bool) -> io::Result<bool> {
    let choices = match yes_is_default {
        true => ["Y", "n"],
        false => ["y", "N"],
    };
    let question = format_choice_question(question, &choices);

    match ask(&question)?.as_str().trim() {
        "y" | "yes" => Ok(true),
        "n" | "no" => Ok(false),
        _ => Ok(yes_is_default),
    }
}

pub fn ask_for_dir(question: &str) -> io::Result<PathBuf> {
    loop {
        let path = ask(question)?;
        let path = path.trim();
        let path = Path::new(&path);

        if path.is_dir() {
            return Ok(path.to_path_buf());
        } else {
            println!("{}", Color::Yellow.paint("Path is not a valid directory."));
        }
    }
}

fn format_choice_question(question: &str, choices: &[&str]) -> String {
    format!(
        "{question} {}",
        Color::Black.paint(format!("({ })", choices.join("|"))),
    )
}

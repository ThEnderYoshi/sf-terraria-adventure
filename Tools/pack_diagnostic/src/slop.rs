/*!
 * Implementation of the Sans' Lovely orOPerties language.
 */

use lazy_static::lazy_static;
use regex::Regex;
use std::{collections::HashMap, io::{self, BufRead, BufReader}, fs::File, path::Path};

/// Represents a Sans' Lovely prOPerties object.
pub struct SLOP {
    items: HashMap<String, SLOPValue>,
}

impl SLOP {
    /// Constructs a new, empty SLOP object.
    pub fn new() -> Self {
        Self { items: HashMap::new() }
    }

    /// Constructs a new SLOP object from the provided lines.
    ///
    /// The lines, when joined together, should represent a valid SLOP
    /// string.
    pub fn from_lines(lines: Vec<String>) -> Self {
        let mut slop = Self::new();
        slop.parse_lines(lines);
        slop
    }

    /// Constructs a new SLOP object by reading the contents of a SLOP
    /// file.
    pub fn from_file<P>(file_path: P) -> io::Result<Self>
    where
        P: AsRef<Path>,
    {
        Ok(Self::from_lines(read_lines(file_path)?))
    }

    /// Converts the object to a valid SLOP string.
    pub fn serialize(self: &Self) -> String {
        let mut result = String::new();

        for (key, value) in self.items.iter() {
            match value {
                SLOPValue::String(value)
                    => result.push_str(&format!("{key}={value}\n")),
                SLOPValue::List(value)
                    => result.push_str(&format!(
                        "{key}{{\n    {}\n}}\n",
                        value.join("\n    "),
                    )),
            }
        }

        result
    }

    /// Creates/overrides a KV.
    ///
    /// Note that the SLOP object will then start owning the value.
    pub fn insert(self: &mut Self, key: &str, value: SLOPValue) {
        self.items.insert(String::from(key), value);
    }

    /// Helper function that inserts a string value.
    pub fn insert_str(self: &mut Self, key: &str, value: &str) {
        self.insert(key, SLOPValue::String(String::from(value)));
    }

    pub fn contains_key(self: &Self, key: &str) -> bool {
        self.items.contains_key(key)
    }

    /// Returns the value associated with the specified key.
    pub fn get(self: &Self, key: &str) -> Option<&SLOPValue> {
        let key = String::from(key);
        match self.items.get(&key) {
            Some(value) => Some(&value),
            None => None,
        }
    }

    /// Returns the value if it is a list, or [None] if it is a string
    /// or doesn't exist.
    ///
    /// You should only use this if you expect the value to be a list.
    pub fn get_list(self: &Self, key: &str) -> Option<&Vec<String>> {
        if let Some(SLOPValue::List(list)) = self.get(key) {
            Some(list)
        } else {
            None
        }
    }

    pub fn push_to_list_kv(self: &mut Self, key: &str, new_value: String) {
        let key = String::from(key);
        match self.items.get(&key) {
            Some(value) => match value {
                SLOPValue::List(list) => {
                    // Inefficient? What's that?
                    // Hacked together? I hardly know 'er!
                    let mut list = list.clone();
                    list.push(new_value);
                    self.items.insert(key, SLOPValue::List(list));
                },
                SLOPValue::String(_) => panic!("{key} is a String KV."),
            },
            None => panic!("Non-existent KV {key}."),
        }
    }

    /// Parses the lines and adds the resulting values to the object.
    ///
    /// The lines, when joined together, should represent a valid SLOP
    /// string.
    pub fn parse_lines(self: &mut Self, lines: Vec<String>) {
        let mut skip_lines: usize = 0;


        for i in 0..lines.len() {
            let line = clean_up_line(&lines[i]);

            if skip_lines > 0 {
                skip_lines -= 1;
                continue;
            }
            if line.is_empty() || &line[0..0] == "#" {
                continue;
            }

            if let ParseData::String(key, value) = parse_string_kv(&line) {
                self.items.insert(key, SLOPValue::String(value));
            } else if let ParseData::List(key, values, skip) = parse_list_kv(&lines, i) {
                self.items.insert(key, SLOPValue::List(values));
                skip_lines = skip;
            } else {
                panic!("Malformed SLOP: Line #{} is invalid: '{}'", i + 1, line);
            }
        }
    }
}

fn clean_up_line(line: &str) -> String {
    String::from(line.trim_start())
}

fn parse_string_kv(line: &str) -> ParseData {
    lazy_static! {
        static ref RE_STRING_KV: Regex = Regex::new(r"^([^=\}]*)=(.*)$").unwrap();
    }
    if !RE_STRING_KV.is_match(line) {
        ParseData::Invalid
    } else {
        let captures = RE_STRING_KV.captures(line).unwrap();
        ParseData::String(
            String::from(&captures[1].to_owned()),
            String::from(&captures[2].to_owned()),
        )
    }
}

fn parse_list_kv(lines: &Vec<String>, start_index: usize) -> ParseData {
    lazy_static! {
        static ref RE_LIST_KV_START: Regex = Regex::new(r"^([^=\}]*)\{\s*$").unwrap();
        static ref RE_LIST_KV_END: Regex = Regex::new(r"^\}\s*$").unwrap();
    }
    if !RE_LIST_KV_START.is_match(&lines[start_index]) {
        ParseData::Invalid
    } else {
        let mut values: Vec<String> = Vec::new();

        for i in start_index..lines.len() {
            let line = clean_up_line(&lines[i]);
            if RE_LIST_KV_END.is_match(&line) {
                let captures = RE_LIST_KV_START.captures(&lines[start_index]).unwrap();
                return ParseData::List(
                    String::from(&captures[1]),
                    values,
                    i - start_index,
                );
            }
            values.push(line);
        }
        ParseData::Invalid
    }
}

/// The value portion of a SLOP Key-value.
///
/// Can be either a string or a list.
/// (In Rust's case, that's a [`Vec<String>`])
pub enum SLOPValue {
    String(String),
    List(Vec<String>),
}

enum ParseData {
    String(String, String),
    List(String, Vec<String>, usize),
    Invalid,
}

// From: https://stackoverflow.com/questions/30801031/read-a-file-and-get-an-array-of-strings
fn read_lines<P>(file_path: P) -> io::Result<Vec<String>>
where
    P: AsRef<Path>,
{
    BufReader::new(File::open(file_path)?).lines().collect()
}

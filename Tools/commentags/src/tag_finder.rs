/*!
 * This module is dedicated to finding tags.
 * How each tag is applied is handled by [applier] and its sub-modules.
 */

use std::path::PathBuf;

use regex::Regex;

/// Expands to a [CommentType] enum.
///
/// ## Examples
///
/// ```rust
/// let inline = CommentType::Inline(String::from("//"));
/// let multiline = CommentType::Multiline(String::from("/*"), String::from("*/"))
/// assert_eq!(comment_type!("//"), inline);
/// assert_eq!(comment_type!("/*", "*/"), multiline);
/// ```
#[macro_export]
macro_rules! comment_type {
    ($x:expr) => {
            CommentType::Inline(String::from($x))
    };
    ($x:expr, $y:expr) => {
            CommentType::Multiline(String::from($x), String::from($y))
    };
}

/// A type of comment for the language the target file is in.
///
/// Consider using [comment_type!] to construct this enum.
pub enum CommentType {
    /// `Inline("//") -> // ...`
    Inline(String),
    /// `Mutliline("/*", "*/") -> /* ... */`
    Multiline(String, String),
}

impl CommentType {
    /// Creates a [Regex] struct from the comment type.
    ///
    /// The contents of the comment are captured.
    /// (i.e. `comment_type!("//") -> r"//(.*?)\n"`)
    fn to_regex(self: &Self) -> Regex {
        let re_string = match self {
            Self::Inline(start) => format!(r"{start}(.*?)\n"),
            Self::Multiline(start, end) => format!(r"{start}([\s\S]*?){end}"),
        };
        Regex::new(&re_string).unwrap()
    }
}

/// Data related to a comment in a target.
///
/// `'a` is the lifetime of the comment's type.
#[derive(Clone, Copy)]
pub struct CommentData<'a> {
    comment_type: &'a CommentType,
    start_index: usize,
    end_index: usize,
}

impl<'a> CommentData<'a> {
    fn new(comment_type: &'a CommentType, start_index: usize, end_index: usize) -> Self {
        Self { comment_type, start_index, end_index }
    }

    fn get_inner_indices(self: &Self) -> (usize, usize) {
        let relative_indices = match self.comment_type {
            CommentType::Inline(start) => (start.len(), 0),
            CommentType::Multiline(start, end) => (start.len(), end.len()),
        };
        (self.start_index + relative_indices.0, self.end_index - relative_indices.1)
    }

    fn get_comment_string<'s>(self: &Self, from_target: &'s str) -> &'s str {
        &from_target[self.start_index..self.end_index]
    }

    fn get_comment_contents<'s>(self: &Self, from_target: &'s str) -> &'s str {
        let (start, end) = self.get_inner_indices();
        &from_target[start..end]
    }
}

struct TagData<'a> {
    comment_data: &'a CommentData<'a>,
}

/// Struct that searches through a target file and finds commentags.
///
/// Note that [TagFinder] does not discriminate comment syntax inside i.e. strings.
pub struct TagFinder {
    comment_types: Vec<CommentType>,
}

impl TagFinder {
    /// Creates a new [TagFinder].
    pub fn new(comment_types: Vec<CommentType>) -> Self {
        Self { comment_types }
    }

    /// Creates a new [TagFinder] with comment types
    /// based on the file's extension.
    ///
    /// Returns [None] if the file doesn't have a valid extension.
    ///
    /// ## Valid file exensions
    ///
    /// | Extension(s) | Comment type(s) |
    /// |--------------|-----------------|
    /// | `md`, `xml`, `html` | `<!-- ... -->` |
    /// | `c`, `cpp`, `cs`, `java`, `js`, `ts`, `rs` | `// ...`, `/* ... */` |
    /// | `py` | `# ...` |
    /// | `lua` | `-- ...`, `--[[ ... ]]` |
    ///
    /// ## Example
    ///
    /// ```rust
    /// let path = PathBuf::from("foo.py");
    /// let tag_finder = TagFinder::from_file_type(&path);
    /// let raw_finder = TagFinder::new(vec![comment_type("#")]);
    /// assert_eq!(tag_finder, raw_finder);
    /// ```
    pub fn from_file_type(file_path: &PathBuf) -> Option<Self> {
        let types = match file_path.extension()?.to_str()? {
            "md" | "xml" | "html"
                => Some(vec![comment_type!("<!--", "-->")]),
            "c" | "cpp" | "cs" | "java" | "js" | "ts" | "rs"
                => Some(vec![comment_type!("//"), comment_type!("/*", "*/")]),
            "py"
                => Some(vec![comment_type!("#")]),
            "lua"
                => Some(vec![comment_type!("--[[", "]]"), comment_type!("--")]),
            &_
                => None,
        }?;
        Some(Self::new(types))
    }

    /// Searches `target` and returns a [Vec] representing all of its comments. _(NOT tags!)_
    pub fn find_comments(self: &Self, target: &str) -> Vec<CommentData> {
        let mut comments: Vec<CommentData> = vec![];

        for comment_type in &self.comment_types {
            let re_comment = comment_type.to_regex();

            for re_match in re_comment.find_iter(target) {
                // let captures = re_comment.captures_at(target, re_match)
                //     .expect("Expected a match at the specified re_match");
                let data =
                    CommentData::new(comment_type, re_match.start(), re_match.end());

                if !comments.iter().any(|&x| {
                    x.start_index <= data.start_index && x.end_index >= data.start_index
                }) {
                    comments.push(data);
                }
            }
        }

        comments
    }

    pub fn find_tags(self: &Self, target: &str) {
        let comments = self.find_comments(target);

        for comment in comments {
            println!("--- {} ---", comment.get_comment_contents(target));
        }
    }
}

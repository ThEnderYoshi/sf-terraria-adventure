/*!
 * Definitions for all the commentags.
 */

const TAG_PREFIX: &str = "@";

/// Structs implementing this represent commentags.
trait Commentag {
    fn string_name() -> String;
}

struct Template {
    id: String,
    //contents: Vec<dyn TargetChunk>,
}

struct Region {
    id: String,
    //contents: Vec<dyn TargetChunk>,
}

impl Commentag for Template {
    fn string_name() -> String {
        String::from("template")
    }
}

impl Commentag for Region {
    fn string_name() -> String {
        String::from("region")
    }
}

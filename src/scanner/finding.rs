use crate::matching::scout::{Scout, ScoutRule};
#[derive(Debug)]
pub struct Finding<'a> {
    pub path: String,
    pub line: usize,
    pub line_text: String,
    pub scout_name: String,
    pub matches: Vec<&'a ScoutRule>,
}

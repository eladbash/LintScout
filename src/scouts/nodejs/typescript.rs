use regex::Regex;

use crate::matching::{
    fileExtension::FileExtension,
    language::Language,
    scout::{Scout, ScoutRule},
};

pub fn new() -> Scout {
    let rules = vec![ScoutRule::new(
        "@ts-nocheck".to_string(),
        "TypeScript 3.7 allows us to add // @ts-nocheck comments to the top of TypeScript files to disable semantic checks. Historically this comment was only respected in JavaScript source files in the presence of checkJs, but we’ve expanded support to TypeScript files to make migrations easier for all users".to_string(),
        Regex::new("// @ts-nocheck").unwrap()
    ),ScoutRule::new(
        "@ts-ignore".to_string(),
        "TypeScript 2.6 support suppressing errors in .ts files using // @ts-ignore comments placed above the offending lines.".to_string(),
        Regex::new("// @ts-ignore").unwrap()
    )];

    Scout::new(
        Language::NodeJS,
        "TYPESCRIPT".to_string(),
        vec![Regex::new("node_moduels").unwrap()],
        vec![FileExtension::from("tsx"), FileExtension::from("ts")],
        rules,
    )
}

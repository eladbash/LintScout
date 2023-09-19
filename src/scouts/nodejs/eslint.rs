use regex::Regex;

use crate::matching::{
    fileExtension::FileExtension,
    language::Language,
    scout::{Scout, ScoutRule},
};

pub fn new() -> Scout {
    let rules = vec![ScoutRule::new(
        "no-await-in-loop".to_string(),
        "Performing an operation on each element of an iterable is a common task. However, performing an await as part of each operation is an indication that the program is not taking full advantage of the parallelization benefits of async/await.".to_string(),
        Regex::new("eslint-disable-next-line no-await-in-loop").unwrap()
    )];

    Scout::new(
        Language::NodeJS,
        "ESLINT".to_string(),
        vec![Regex::new("node_moduels").unwrap()],
        vec![FileExtension::from("js"), FileExtension::from("ts")],
        rules,
    )
}

use ignore::WalkBuilder;
use std::thread::sleep;
use std::time::Duration;
use std::{fs::read_to_string, ops::Deref};

use crate::matching::scout::ScoutRule;
use crate::{
    matching::{fileExtension::FileExtension, scout::Scout},
    scanner::finding::Finding,
};

use super::stats::ScanStats;

pub struct Scanner<'a> {
    root_path: &'a str,
    scouts: &'a Vec<Scout>,
}

impl Scanner<'_> {
    pub fn new<'a>(root_path: &'a str, scouts: &'a Vec<Scout>) -> Scanner<'a> {
        Scanner { root_path, scouts }
    }

    pub fn run(&self) -> (Vec<Finding>, ScanStats) {
        let mut stats = ScanStats::new();
        let mut findings: Vec<Finding> = vec![];
        //decided to merge all the exclude_paths from all scouts -
        // so we can walk one time instead of walk for each scout
        let ignore_paths: Vec<regex::Regex> = self
            .scouts
            .iter()
            .map(|s| return s.get_exclude_paths())
            .flatten()
            .collect();

        let mut walker = WalkBuilder::new(&self.root_path).build();
        // for ignore in ignore_paths {
        //     walker.add_ignore(ignore.as_str());
        // }
        //let walk = walker.filter_entry(|e| e.path().is_file()).build();

        for file in walker {
            match file {
                Ok(entry) => {
                    // println!("{}", entry.path().display().to_string());
                    if let Some(file_ext) = entry.path().extension() {
                        if (entry.path().is_file()) {
                            stats.files_scanned += 1;
                            // println!("Scanning {}", entry.path().display());
                            let file_path = entry.path().display().to_string();

                            let matching_scouts: Vec<&Scout> = self
                                .scouts
                                .iter()
                                .filter(|scout| {
                                    scout.appliest_to_ext(&FileExtension::from(
                                        file_ext.to_str().unwrap(),
                                    ))
                                })
                                .collect();
                            //There are no matching scouts for this file - we are doing nothing
                            if !matching_scouts.is_empty() {
                                //We have matching scout for this file
                                let mut line = 1;
                                for line_text in read_to_string(&file_path)
                                    .expect(&format!("failed with {}", file_path))
                                    .lines()
                                {
                                    for scout in &matching_scouts {
                                        let matches: Vec<&ScoutRule> = scout.get_matches(line_text);
                                        if !matches.is_empty() {
                                            for scout_match in &matches {
                                                findings.push(Finding {
                                                    path: file_path.clone(),
                                                    line: line.clone(),
                                                    line_text: line_text.to_string(),
                                                    scout_name: scout.name(),
                                                    matches: matches.clone(),
                                                });
                                                stats.findings_count += 1;
                                            }
                                        }
                                    }
                                    line += 1;
                                }
                            } else {
                                // println!("no matching scouts for {}", file_ext.to_str().unwrap());
                            }
                        }
                    }
                }
                Err(err) => {
                    eprintln!("Error: {}", err);
                }
            }
        }
        println!("______________________________");
        println!("FILES {}", stats.get_files_scanned());
        (findings, stats)
        //walk and keep only relevant paths
        //for each path we want to check if the matches some scout and some rule and get results
    }
}

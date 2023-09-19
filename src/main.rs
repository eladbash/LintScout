use std::time::Instant;

use lint_scount::{scanner::scanner::Scanner, scouts::nodejs};

fn main() {
    //We have matchers {targets_extensions: Vec[string], patterns: Vec<string>, name: String}
    //We want to get a command like `lintscout --path path/root --matchers=*/m1,m2` and scan recursivelty all the files in this path
    //For each file - we want to run a set of rules (matchers patterns) to find lines where might contain an ignore. the matcher must match in the pattern + file extension
    //We would like to show list of files, with lines and the kind of matcher
    println!("Let's catch those lint ignores 👁️‍🗨️");
    let scouts = vec![nodejs::eslint::new(), nodejs::typescript::new()];
    let scouts_names: Vec<String> = scouts.iter().map(|s| s.name()).collect();
    let root_path = String::from("./");
    let scanner = Scanner::new(&root_path, &scouts);

    println!(
        "Performing scan on '{}' with scouts [{}]",
        root_path,
        scouts_names.join(",")
    );
    let start_time = Instant::now();

    let (findings, stats) = scanner.run();

    if !findings.is_empty() {
        for result in findings.iter() {
            println!("{:#?}", result)
        }

        let scanned_files = stats.get_files_scanned();
        let findings_count = stats.get_findings_count();
        println!(
            "Stats: \n - files scanned:{} \n - findings:{} ",
            scanned_files.to_string(),
            findings_count.to_string()
        );
        for scout_stat in &scouts {
            let count = findings
                .iter()
                .filter(|f| f.scout_name == scout_stat.name())
                .count();
            println!("{} - {}", scout_stat.name(), count);
        }
    } else {
        println!("🔍 Scan complete and all clear! Great job! 👍")
    }

    let end_time = Instant::now();
    let elapsed_time = end_time - start_time;
    println!("Execution time: {:.2} seconds", elapsed_time.as_secs_f64());
    println!("Execution time: {} milliseconds", elapsed_time.as_millis());
}

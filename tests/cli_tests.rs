use assert_cmd::cargo::cargo_bin_cmd;
use predicates::prelude::*;

fn cmd() -> assert_cmd::Command {
    cargo_bin_cmd!("lintscout")
}

#[test]
fn scan_fixtures_default() {
    cmd()
        .arg("tests/fixtures")
        .assert()
        .failure()
        .code(1)
        .stdout(predicate::str::contains("Findings:"));
}

#[test]
fn scan_fixtures_json_output() {
    let output = cmd()
        .args(["tests/fixtures", "--format", "json"])
        .assert()
        .failure()
        .code(1)
        .get_output()
        .stdout
        .clone();
    let json: serde_json::Value = serde_json::from_slice(&output).expect("valid JSON output");
    assert!(json["findings"].is_array());
    assert!(json["stats"]["findings_count"].as_u64().unwrap() > 0);
}

#[test]
fn scan_fixtures_count_output() {
    cmd()
        .args(["tests/fixtures", "--format", "count"])
        .assert()
        .failure()
        .code(1)
        .stdout(predicate::str::is_match(r"^\d+$").unwrap());
}

#[test]
fn pass_threshold_high_exits_zero() {
    cmd()
        .args(["tests/fixtures", "--pass-threshold", "999"])
        .assert()
        .success()
        .code(0);
}

#[test]
fn pass_threshold_zero_exits_one() {
    cmd()
        .args(["tests/fixtures", "--pass-threshold", "0"])
        .assert()
        .failure()
        .code(1);
}

#[test]
fn scout_filter() {
    let output = cmd()
        .args(["tests/fixtures", "--scouts", "eslint", "--format", "json"])
        .assert()
        .get_output()
        .stdout
        .clone();
    let json: serde_json::Value = serde_json::from_slice(&output).expect("valid JSON");
    if let Some(findings) = json["findings"].as_array() {
        for f in findings {
            assert_eq!(f["linter"].as_str().unwrap(), "eslint");
        }
    }
}

#[test]
fn exclude_scouts() {
    let output = cmd()
        .args([
            "tests/fixtures",
            "--exclude-scouts",
            "eslint,typescript,biome,prettier,jshint,stylelint",
            "--format",
            "json",
        ])
        .assert()
        .get_output()
        .stdout
        .clone();
    let json: serde_json::Value = serde_json::from_slice(&output).expect("valid JSON");
    if let Some(findings) = json["findings"].as_array() {
        for f in findings {
            assert_ne!(f["linter"].as_str().unwrap(), "eslint");
        }
    }
}

#[test]
fn custom_config_loads() {
    // Config sets pass_threshold=100, so this should succeed (findings < 100)
    // The custom scout adds a console.log finding, proving config was loaded
    let output = cmd()
        .args([
            "tests/fixtures",
            "--config",
            "tests/fixtures/lintscout.yml",
            "--format",
            "json",
        ])
        .assert()
        .success()
        .get_output()
        .stdout
        .clone();
    let json: serde_json::Value = serde_json::from_slice(&output).expect("valid JSON");
    let findings = json["findings"].as_array().unwrap();
    // Should contain the custom scout finding for console.log
    assert!(findings
        .iter()
        .any(|f| f["scout_name"].as_str() == Some("custom-test")));
}

#[test]
fn no_findings_exits_zero() {
    // Create a temp dir with no matching files
    let dir = tempfile::tempdir().unwrap();
    std::fs::write(dir.path().join("readme.txt"), "hello world").unwrap();
    cmd().arg(dir.path()).assert().success().code(0);
}

#[test]
fn quiet_mode_suppresses_clean_output() {
    let dir = tempfile::tempdir().unwrap();
    std::fs::write(dir.path().join("readme.txt"), "hello world").unwrap();
    cmd()
        .args([dir.path().to_str().unwrap(), "--quiet"])
        .assert()
        .success()
        .stdout(predicate::str::is_empty());
}

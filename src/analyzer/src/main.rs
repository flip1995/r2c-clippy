use serde::{Deserialize, Serialize};
use serde_json;
use std::fs;
use std::path::Path;
use std::process::Command;
use toml;

#[derive(Serialize, Deserialize)]
struct JsonResult {
    results: Vec<ResultEntry>,
}

#[derive(Serialize, Deserialize)]
enum ErrorType {
    ICE,
    Error,
    Warning,
    None,
}

#[derive(Serialize, Deserialize)]
struct ResultEntry {
    check_id: String,
    extra: Extra,
}

impl ResultEntry {
    fn new(repo: String, error_type: ErrorType) -> Self {
        ResultEntry {
            check_id: "clippy_results".to_string(),
            extra: Extra { repo, error_type },
        }
    }
}

#[derive(Serialize, Deserialize)]
struct Extra {
    repo: String,
    error_type: ErrorType,
}

#[derive(Deserialize)]
struct CargoToml {
    package: Package,
}

#[derive(Deserialize)]
struct Package {
    name: String,
}

fn main() {
    let stderr = Command::new("cargo")
        .arg("clippy")
        .output()
        .ok()
        .map(|out| out.stderr);
    let toml_str = fs::read_to_string(Path::new("Cargo.toml")).expect("Unable to read Cargo.toml");
    let toml: CargoToml = toml::from_str(&toml_str).expect("Unable to parse Cargo.toml");
    let repo = toml.package.name;
    let error_type = if let Some(stderr) = stderr {
        let stderr = String::from_utf8_lossy(&stderr);
        if stderr.contains("internal compiler error") {
            ErrorType::ICE
        } else if stderr.contains("error:") {
            ErrorType::Error
        } else if stderr.contains("warning:") {
            ErrorType::Warning
        } else {
            ErrorType::None
        }
    } else {
        ErrorType::None
    };

    let result_entry = ResultEntry::new(repo, error_type);

    let output_json = fs::read_to_string(Path::new("/analysis/output/output.json"))
        .ok()
        .unwrap_or_else(String::new);
    let result: Option<JsonResult> = serde_json::from_str(&output_json).ok();

    let result = if let Some(mut result) = result {
        result.results.push(result_entry);
        result
    } else {
        JsonResult {
            results: vec![result_entry],
        }
    };

    let json = serde_json::to_string(&result).expect("Unable to serialize JsonResult");

    fs::write(Path::new("/analysis/output/output.json"), json)
        .expect("Unable to write to output.json");
}

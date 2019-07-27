use clap::{App, Arg};
use crates_io_api::{Error, ListOptions, Sort, SyncClient};
use serde::Serialize;
use serde_json;
use std::fs;
use std::path::Path;
use std::process::{Command, Stdio};
use std::str::FromStr;

#[derive(Serialize)]
struct Crates {
    name: String,
    version: String,
    description: String,
    readme: Option<String>,
    inputs: Vec<Input>,
}

impl Crates {
    fn new(inputs: &[Input]) -> Self {
        Self {
            name: String::from("crates-io"),
            version: String::from("0.0.1"),
            description: String::from("Crates from crates.io"),
            readme: None,
            inputs: inputs.to_vec(),
        }
    }
}

#[derive(Serialize, Clone)]
struct Input {
    repo_url: String,
    commit_hash: String,
    input_type: String,
}

impl Input {
    fn new(repo_url: &str, commit_hash: &str) -> Self {
        Self {
            repo_url: repo_url.to_string(),
            commit_hash: commit_hash.to_string(),
            input_type: String::from("GitRepoCommit"),
        }
    }
}

fn main() -> Result<(), Error> {
    let matches = App::new("input_gen")
        .arg(
            Arg::with_name("number")
                .short("n")
                .required(true)
                .default_value("10"),
        )
        .get_matches();
    let n_crates = matches
        .value_of("number")
        .map(|n| u64::from_str(n).expect("not a number"))
        .unwrap();

    let client = SyncClient::new();

    let crates = client
        .crates(ListOptions {
            sort: Sort::Downloads,
            per_page: n_crates,
            page: 1,
            query: None,
        })?
        .crates;

    let inputs = crates
        .iter()
        .filter_map(|krate| krate.repository.clone())
        .map(|repo| {
            let head = Command::new("git")
                .arg("ls-remote")
                .arg(&repo)
                .stdout(Stdio::piped())
                .arg("grep")
                .arg("HEAD")
                .stdout(Stdio::piped())
                .output()
                .expect("Unable to grep the commit hash")
                .stdout;
            let head = String::from_utf8_lossy(&head);
            let commit_hash = head.split('\t').next().unwrap();

            Input::new(&repo, &commit_hash)
        })
        .collect::<Vec<_>>();

    let crates = Crates::new(&inputs);

    let json = serde_json::to_string_pretty(&crates).expect("Unable to serialize the input");

    fs::write(Path::new("input.json"), json).expect("Unable to write to input.json");

    Ok(())
}

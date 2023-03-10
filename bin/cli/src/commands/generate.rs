use std::{io::Write, path::PathBuf, str::FromStr};

use clap::{ArgMatches, Command};
use harmony::{config::Config, repo::Repository};

#[must_use]
pub fn cli() -> Command {
    Command::new("generate").about("Generate a Repo in the current directory")
}

pub fn execute(_matches: &ArgMatches) {
    let path = PathBuf::from("harmony.toml");
    if !path.exists() {
        eprintln!("err: No `harmony.toml` in the current directory");
        return;
    }
    let config =
        Config::from_str(&std::fs::read_to_string(path).expect("Failed to read `harmony.yoml`"))
            .unwrap();
    println!("Generating {}", config.unit().name());
    let repo: Repository = config.try_into().unwrap();
    let mut out = std::fs::File::create("harmony.mpk").unwrap();
    out.write_all(&repo.to_blob()).unwrap();
    println!("`harmony.mpk` Created!")
}

mod api;
mod cli;
mod domain;
mod repositories;

use std::sync::Arc;

use repositories::pokemon::{InMemoryRepository, Repository};

#[macro_use]
extern crate rouille;
extern crate clap;
extern crate serde;

use clap::{crate_authors, crate_name, crate_version, App, Arg, SubCommand};

use crate::repositories::pokemon::SqliteRepository;

fn main() {
    let matches = App::new(crate_name!())
        .version(crate_version!())
        .author(crate_authors!())
        .subcommand(SubCommand::with_name("cli").about("Use cli <name>!"))
        .subcommand(SubCommand::with_name("api").about("Use api <name>!"))
        .arg(Arg::with_name("sqlite").long("sqlite").value_name("PATH"))
        .get_matches();

    let repo = build_repo(matches.value_of("sqlite"));

    match matches.subcommand() {
        ("cli", Some(_)) => {
            run_cli(repo);
        }
        ("api", Some(_)) => {
            run_api(repo);
        }
        _ => unreachable!(),
    };
}

fn build_repo(sqlite_value: Option<&str>) -> Arc<dyn Repository> {
    if let Some(path) = sqlite_value {
        match SqliteRepository::try_new(path) {
            Ok(repo) => return Arc::new(repo),
            Err(_) => panic!("Error while creating sqlite repo"),
        }
    }

    Arc::new(InMemoryRepository::new())
}

fn run_api(repo: Arc<dyn Repository>) {
    let url = "localhost:8080";
    println!("http://{}", &url);

    api::serve(url, repo);
}

fn run_cli(repo: Arc<dyn Repository>) {
    cli::run(repo);
}

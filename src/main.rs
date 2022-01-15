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

use clap::{crate_authors, crate_name, crate_version, App, Arg};

fn main() {
    let repo = Arc::new(InMemoryRepository::new());

    let matches = App::new(crate_name!())
        .version(crate_version!())
        .author(crate_authors!())
        .arg(
            Arg::with_name("cli")
                .short("c")
                .long("cli")
                .help("Runs in CLI mode"),
        )
        .arg(
            Arg::with_name("serve")
                .short("s")
                .long("serve")
                .help("Runs in server mode"),
        )
        .get_matches();

    match matches.occurrences_of("cli") {
        0 => run_api(repo),
        _ => run_cli(repo),
    }
}

fn run_api(repo: Arc<dyn Repository>) {
    let url = "localhost:8080";
    println!("http://{}", &url);

    api::serve(url, repo);
}

fn run_cli(repo: Arc<dyn Repository>) {
    cli::run(repo);
}

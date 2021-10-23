mod api;
mod domain;
mod repositories;

use std::sync::Arc;

use repositories::pokemon::InMemoryRepository;

#[macro_use]
extern crate rouille;
extern crate serde;

fn main() {
    let url = "localhost:8080";
    println!("http://{}", &url);

    let repo = Arc::new(InMemoryRepository::new());
    api::serve(url, repo);
}

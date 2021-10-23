use std::sync::Arc;

use crate::{api::Status, domain::create_pokemon, repositories::pokemon::Repository};
use rouille;
use serde::{Deserialize, Serialize};

#[derive(Serialize)]
struct Response {
    message: String,
}

#[derive(Deserialize, Debug)]
struct Request {
    number: u16,
    name: String,
    types: Vec<String>,
}

pub fn serve(repo: Arc<dyn Repository>, req: &rouille::Request) -> rouille::Response {
    println!("");
    let resReq = rouille::input::json_input::<Request>(req);
    println!("{:#?}", &resReq);
    let reqRepo = match resReq {
        Ok(req) => create_pokemon::Request {
            number: req.number,
            name: req.name,
            types: req.types,
        },
        Err(e) => return rouille::Response::from(Status::BadRequest),
    };

    match create_pokemon::execute(repo, reqRepo) {
        create_pokemon::Response::Ok(number) => rouille::Response::json(&Response {
            message: number.to_string(),
        }),
        create_pokemon::Response::BadRequest => rouille::Response::from(Status::BadRequest),
        create_pokemon::Response::Conflict => rouille::Response::from(Status::Conflict),
        create_pokemon::Response::Error => rouille::Response::from(Status::InternalServerError),
    }
}

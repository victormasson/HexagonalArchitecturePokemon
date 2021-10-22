use std::{convert::TryFrom, fmt::Error};

use crate::repositories::pokemon::{Insert, Repository};

use super::entities::{PokemonName, PokemonNumber, PokemonTypes};

struct Request {
    number: u16,
    name: String,
    types: Vec<String>,
}

enum Response {
    Ok(u16),
    BadRequest,
    Conflict,
    Error,
}

fn execute(repo: &mut dyn Repository, req: Request) -> Response {
    match (
        PokemonNumber::try_from(req.number),
        PokemonName::try_from(req.name),
        PokemonTypes::try_from(req.types),
    ) {
        (Ok(number), Ok(name), Ok(types)) => match repo.insert(number, name, types) {
            Insert::Ok(number) => Response::Ok(u16::from(number)),
            Insert::Conflict => Response::Conflict,
            Insert::Error => Response::Error,
        },
        _ => Response::BadRequest,
    }
}

#[cfg(test)]
mod tests {

    use std::convert::TryFrom;

    use crate::{
        domain::{
            create_pokemon::{execute, Response},
            entities::{PokemonName, PokemonNumber, PokemonTypes},
        },
        repositories::pokemon::{InMemoryRepository, Repository},
    };

    use super::Request;

    #[test]
    fn should_return_number() {
        let mut repo = InMemoryRepository::new();

        let n = 25;
        let q = Request {
            number: n,
            name: "Pikachu".to_string(),
            types: vec!["Electric".to_string()],
        };

        let res = execute(&mut repo, q);

        match res {
            Response::Ok(res_numb) => assert_eq!(res_numb, n),
            _ => assert!(false),
        }
    }

    #[test]
    fn should_return_bad_request() {
        let mut repo = InMemoryRepository::new();

        let q = Request {
            number: 25,
            name: "".to_string(),
            types: vec!["Electric".to_string()],
        };

        let res = execute(&mut repo, q);

        match res {
            Response::BadRequest => assert!(true),
            _ => assert!(false),
        }
    }

    #[test]
    fn should_return_conflict_exist() {
        let number = PokemonNumber::try_from(25).unwrap();
        let name = PokemonName::try_from("Pikachu".to_string()).unwrap();
        let types = PokemonTypes::try_from(vec!["Electric".to_string()]).unwrap();

        let mut repo = InMemoryRepository::new();
        repo.insert(number, name, types);

        let req = Request {
            number: 25,
            name: String::from("Charmander"),
            types: vec![String::from("Fire")],
        };

        let res = execute(&mut repo, req);

        match res {
            Response::Conflict => {}
            _ => unreachable!(),
        }
    }

    #[test]
    fn should_return_error_when_unexpected_error_happends() {
        let mut repo = InMemoryRepository::new().with_error();
        let number = 25;
        let req = Request {
            number,
            name: String::from("Pikachu"),
            types: vec![String::from("Electric")],
        };

        let res = execute(&mut repo, req);

        match res {
            Response::Error => {}
            _ => unreachable!(),
        };
    }
}

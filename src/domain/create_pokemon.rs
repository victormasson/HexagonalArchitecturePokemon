use std::{convert::TryFrom, sync::Arc};

use crate::repositories::pokemon::{Insert, Repository};

use super::entities::{PokemonName, PokemonNumber, PokemonTypes};

pub struct Request {
    pub number: u16,
    pub name: String,
    pub types: Vec<String>,
}

pub enum Response {
    Ok(u16),
    BadRequest,
    Conflict,
    Error,
}

pub fn execute(repo: Arc<dyn Repository>, req: Request) -> Response {
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

    use std::{convert::TryFrom, sync::Arc};

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
        let repo = Arc::new(InMemoryRepository::new());

        let n = 25;
        let q = Request {
            number: n,
            name: "Pikachu".to_string(),
            types: vec!["Electric".to_string()],
        };

        let res = execute(repo, q);

        match res {
            Response::Ok(res_numb) => assert_eq!(res_numb, n),
            _ => assert!(false),
        }
    }

    #[test]
    fn should_return_bad_request() {
        let repo = Arc::new(InMemoryRepository::new());

        let q = Request {
            number: 25,
            name: "".to_string(),
            types: vec!["Electric".to_string()],
        };

        let res = execute(repo, q);

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

        let repo = Arc::new(InMemoryRepository::new());

        repo.insert(number, name, types);

        let req = Request {
            number: 25,
            name: String::from("Charmander"),
            types: vec![String::from("Fire")],
        };

        let res = execute(repo, req);

        match res {
            Response::Conflict => {}
            _ => unreachable!(),
        }
    }

    #[test]
    fn should_return_error_when_unexpected_error_happends() {
        let repo = Arc::new(InMemoryRepository::new().with_error());
        let number = 25;
        let req = Request {
            number,
            name: String::from("Pikachu"),
            types: vec![String::from("Electric")],
        };

        let res = execute(repo, req);

        match res {
            Response::Error => {}
            _ => unreachable!(),
        };
    }
}

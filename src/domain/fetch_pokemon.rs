use crate::{
    domain::entities::{Pokemon, PokemonNumber},
    repositories::pokemon::{FetchOneError, Repository},
};
use std::{convert::TryFrom, sync::Arc};

pub struct Request {
    pub number: u16,
}

pub enum Error {
    Unknown,
    BadRequest,
    NotFound,
}

pub struct Response {
    pub number: u16,
    pub name: String,
    pub types: Vec<String>,
}

pub fn execute(repo: Arc<dyn Repository>, req: Request) -> Result<Response, Error> {
    match PokemonNumber::try_from(req.number) {
        Ok(number) => match repo.fetch_one(number) {
            Ok(Pokemon {
                number,
                name,
                types,
            }) => Ok(Response {
                number: number.to_u16(),
                name: name.to_string(),
                types: types.to_vec_string(),
            }),

            Err(FetchOneError::NotFound) => Err(Error::NotFound),
            Err(FetchOneError::Unknown) => Err(Error::Unknown),
        },
        _ => Err(Error::BadRequest),
    }
}

#[cfg(test)]
mod tests {

    use crate::{
        domain::entities::{PokemonName, PokemonTypes},
        repositories::pokemon::InMemoryRepository,
    };

    use super::*;

    #[test]
    fn it_should_return_an_unknown_error_when_an_unexpected_error_happens() {
        let repo = Arc::new(InMemoryRepository::new().with_error());
        let req = Request::new(PokemonNumber::pikachu());
        let res = execute(repo, req);
        match res {
            Err(Error::Unknown) => {}
            _ => unreachable!(),
        };
    }

    #[test]
    fn it_should_return_a_bad_request_error_when_request_is_invalid() {
        let repo = Arc::new(InMemoryRepository::new());
        let req = Request::new(PokemonNumber::bad());

        let res = execute(repo, req);

        match res {
            Err(Error::BadRequest) => {}
            _ => unreachable!(),
        }
    }

    #[test]
    fn it_should_return_a_not_found_error_when_the_repo_does_not_contain_the_pokemon() {
        let repo = Arc::new(InMemoryRepository::new());
        let req = Request::new(PokemonNumber::pikachu());

        let res = execute(repo, req);

        match res {
            Err(Error::NotFound) => {}
            _ => unreachable!(),
        }
    }

    #[test]
    fn it_should_return_the_pokemon_otherwise() {
        let repo = Arc::new(InMemoryRepository::new());
        repo.insert(
            PokemonNumber::pikachu(),
            PokemonName::pikachu(),
            PokemonTypes::pikachu(),
        )
        .ok();

        let req = Request::new(PokemonNumber::pikachu());

        let res = execute(repo, req);

        match res {
            Ok(res) => {
                assert_eq!(res.number, PokemonNumber::pikachu().to_u16());
                assert_eq!(res.name, PokemonName::pikachu().to_string());
                assert_eq!(res.types, PokemonTypes::pikachu().to_vec_string());
            }
            _ => unreachable!(),
        }
    }

    impl Request {
        fn new(number: PokemonNumber) -> Self {
            Self {
                number: number.to_u16(),
            }
        }
    }
}

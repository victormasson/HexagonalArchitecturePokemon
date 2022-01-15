use std::sync::Arc;

use crate::repositories::pokemon::{FetchAllError, Repository};

pub enum Error {
    Unknown,
}

#[derive(Debug)]
pub struct Response {
    pub number: u16,
    pub name: String,
    pub types: Vec<String>,
}

pub fn execute(repo: Arc<dyn Repository>) -> Result<Vec<Response>, Error> {
    match repo.fetch_all() {
        Ok(pokemons) => Ok(pokemons
            .into_iter()
            .map(|p| Response {
                number: p.number.to_u16(),
                name: p.name.to_string(),
                types: p.types.to_vec_string(),
            })
            .collect::<Vec<Response>>()),
        Err(FetchAllError::Unknown) => Err(Error::Unknown),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;

    use crate::{
        domain::entities::{PokemonName, PokemonNumber, PokemonTypes},
        repositories::pokemon::InMemoryRepository,
    };

    #[test]
    fn it_should_return_an_unknown_error_when_an_unexpected_error_happens() {
        let repo = Arc::new(InMemoryRepository::new().with_error());
        let res = execute(repo);

        match res {
            Err(Error::Unknown) => {}
            _ => unreachable!(),
        };
    }

    #[test]
    fn it_should_return_all_the_pokemons_ordered_by_increasing_number_otherwise() {
        let repo = Arc::new(InMemoryRepository::new());
        repo.insert(
            PokemonNumber::pikachu(),
            PokemonName::pikachu(),
            PokemonTypes::pikachu(),
        )
        .ok();

        repo.insert(
            PokemonNumber::charmander(),
            PokemonName::charmander(),
            PokemonTypes::charmander(),
        )
        .ok();

        let res = execute(repo);

        match res {
            Ok(res) => {
                assert_eq!(res[0].number, PokemonNumber::charmander().to_u16());
                assert_eq!(res[0].name, PokemonName::charmander().to_string());
                assert_eq!(res[0].types, PokemonTypes::charmander().to_vec_string());
                assert_eq!(res[1].number, PokemonNumber::pikachu().to_u16());
                assert_eq!(res[1].name, PokemonName::pikachu().to_string());
                assert_eq!(res[1].types, PokemonTypes::pikachu().to_vec_string());
            }
            _ => unreachable!(),
        };
    }
}

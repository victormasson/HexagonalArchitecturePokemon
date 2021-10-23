use std::sync::Mutex;

use crate::domain::entities::{PokemonName, PokemonNumber, PokemonTypes};

pub struct InMemoryRepository {
    pokemons: Mutex<Vec<Pokemon>>,
    error: bool,
}

impl InMemoryRepository {
    pub fn new() -> Self {
        let pokemons = Mutex::new(vec![]);
        Self {
            pokemons,
            error: false,
        }
    }

    pub fn with_error(self) -> Self {
        Self {
            error: true,
            ..self
        }
    }
}

pub trait Repository: Send + Sync {
    fn insert(&self, number: PokemonNumber, name: PokemonName, types: PokemonTypes) -> Insert;
}

impl Repository for InMemoryRepository {
    fn insert(&self, number: PokemonNumber, name: PokemonName, types: PokemonTypes) -> Insert {
        if self.error {
            return Insert::Error;
        }

        let mut pokemons_lock = match self.pokemons.lock() {
            Ok(lock) => lock,
            _ => return Insert::Error,
        };

        if pokemons_lock.iter().any(|pokemon| pokemon.number == number) {
            return Insert::Conflict;
        }

        let number_clone = number.clone();
        let p = Pokemon::new(number, name, types);
        pokemons_lock.push(p);
        Insert::Ok(number_clone)
    }
}

pub struct Pokemon {
    pub number: PokemonNumber,
    name: PokemonName,
    types: PokemonTypes,
}

impl Pokemon {
    pub fn new(number: PokemonNumber, name: PokemonName, types: PokemonTypes) -> Self {
        Self {
            number,
            name,
            types,
        }
    }
}

pub enum Insert {
    Ok(PokemonNumber),
    Conflict,
    Error,
}

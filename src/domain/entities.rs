use std::cmp::PartialEq;
use std::convert::TryFrom;
use std::fmt;

#[derive(PartialEq, Clone, Debug)]
pub struct PokemonNumber(u16);

impl PokemonNumber {
    pub fn to_u16(self) -> u16 {
        self.0
    }
}

impl TryFrom<u16> for PokemonNumber {
    type Error = ();

    fn try_from(n: u16) -> Result<Self, Self::Error> {
        if n > 0 && n < 899 {
            Ok(Self(n))
        } else {
            Err(())
        }
    }
}

impl From<PokemonNumber> for u16 {
    fn from(n: PokemonNumber) -> Self {
        n.0
    }
}

#[derive(Clone, Debug)]
pub struct PokemonName(String);

impl PokemonName {
    pub fn to_string(self) -> String {
        self.0
    }
}

impl TryFrom<String> for PokemonName {
    type Error = ();

    fn try_from(n: String) -> Result<Self, Self::Error> {
        if n.is_empty() {
            Err(())
        } else {
            Ok(Self(n))
        }
    }
}

#[derive(Clone, Debug)]
pub struct PokemonTypes(Vec<PokemonType>);

impl PokemonTypes {
    pub fn to_vec_string(self) -> Vec<String> {
        let a: Vec<String> = self
            .0
            .into_iter()
            .map(|t| -> String { String::from(t) })
            .collect();
        a
    }
}

impl TryFrom<Vec<String>> for PokemonTypes {
    type Error = ();

    fn try_from(ts: Vec<String>) -> Result<Self, Self::Error> {
        if ts.is_empty() {
            Err(())
        } else {
            let mut pts = vec![];
            for t in ts.iter() {
                match PokemonType::try_from(String::from(t)) {
                    Ok(pt) => pts.push(pt),
                    _ => return Err(()),
                }
            }
            Ok(Self(pts))
        }
    }
}

#[derive(Clone, Debug)]
enum PokemonType {
    Electric,
    Fire,
}

impl PokemonType {
    pub fn get_value(self) -> String {
        match self {
            Electric => "Electric".to_string(),
            Fire => "Fire".to_string(),
        }
    }
}

impl TryFrom<String> for PokemonType {
    type Error = ();

    fn try_from(t: String) -> Result<Self, Self::Error> {
        match t.as_str() {
            "Electric" => Ok(Self::Electric),
            "Fire" => Ok(Self::Fire),
            _ => Err(()),
        }
    }
}

impl From<PokemonType> for String {
    fn from(t: PokemonType) -> Self {
        String::from(match t {
            PokemonType::Electric => PokemonType::Electric.get_value(),
            PokemonType::Fire => PokemonType::Fire.get_value(),
        })
    }
}

#[derive(Clone)]
pub struct Pokemon {
    pub number: PokemonNumber,
    pub name: PokemonName,
    pub types: PokemonTypes,
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

#[cfg(test)]
impl PokemonNumber {
    pub fn pikachu() -> Self {
        Self(25)
    }

    pub fn charmander() -> Self {
        Self(3)
    }
}

#[cfg(test)]
impl PokemonName {
    pub fn pikachu() -> Self {
        Self("Pikachu".to_string())
    }

    pub fn charmander() -> Self {
        Self("Charmander".to_string())
    }

    pub fn bad() -> Self {
        Self("".to_string())
    }
}

#[cfg(test)]
impl PokemonTypes {
    pub fn pikachu() -> Self {
        Self(vec![PokemonType::Electric])
    }

    pub fn charmander() -> Self {
        Self(vec![PokemonType::Fire])
    }
}

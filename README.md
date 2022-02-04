# Hexagonal Architecture Pokemon

Tuto from [here](https://alexis-lozano.com/hexagonal-architecture-in-rust-1/)

## Run

- Cli

```sh
cargo run -- --sqlite ./database.sqlite cli
```

- Api

```sh
cargo run -- --sqlite ./database.sqlite api
```

## Database

Create database

``` bash
sqlite3 ./database.sqlite
```

Add tables

``` bash
pragma foreign_keys = 1;

create table pokemons (
    number integer primary key,
    name text
);

create table types (
    pokemon_number integer,
    name text,
    foreign key (pokemon_number) references pokemons (number) on delete cascade,
    primary key (pokemon_number, name)
);
```

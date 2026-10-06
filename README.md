# Crab-wORM

Crab-wORM is a Rust library for registering structs and enums as metadata and automatically generating an SQL schema and an HTML page.

## Usage

Add the dependency to your application and derive `crab_worm` on the desired types:

```rust
use crab_worm::crab_worm;

#[derive(crab_worm)]
enum Status {
	Ativo,
	Inativo,
}

#[derive(crab_worm)]
struct Usuario {
	id: i64,
	nome: String,
	idade: Option<i32>,
	status: Status,
	tags: Vec<String>,
}
```

The derive accepts structs with named fields and enums with unit variants. Supported scalar types are integers, `bool`, `f32`, `f64`, `String`, and `char`. `Option<T>`, `Vec<T>`, and references to other registered types are also supported.

## File generation

`parasitize()` creates:

- `schema.sql`, containing tables, foreign keys, and auxiliary tables for `Vec<T>`;
- `index.html`, containing the generated HTML elements for the registered types.

You can also generate the content directly:

## SQL

Structs generate tables with an `id INTEGER PRIMARY KEY` column. Scalar fields become SQL columns; `Option<T>` allows null values; referenced enums and structs generate foreign keys. Types are ordered according to their dependencies to facilitate correct table creation.

Enums generate a table with `id` and `name`, along with the corresponding values. `Vec<T>` fields generate auxiliary tables with a position and a reference to the parent record.

## Limitations

- Structs must contain only named fields.
- Enums must contain only unit variants.
- `Vec<Vec<T>>` and vectors nested in `Option` are not supported.
- Unregistered types and unknown scalars are not supported.

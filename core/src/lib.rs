pub mod connector;
pub mod factory;
pub mod meta;
pub use inventory;

pub use factory::{generate_html, generate_sql};
pub use meta::{EnumMetadata, FieldMetadata, StructMetadata, TypeMetadata};

use std::{error::Error, fs};

pub fn parasitize() -> Result<(), Box<dyn Error>> {
    fs::write(connector::SCHEMA_FILE, generate_sql())?;
    fs::write("index.html", generate_html())?;
    println!("Crab-wORM: schema.sql e index.html gerados");

    connector::apply_schema_file(connector::SCHEMA_FILE)?;
    Ok(())
}
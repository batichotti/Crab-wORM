pub mod meta;
pub mod factory;
pub use inventory;

pub use factory::generate_sql;
pub use factory::generate_html;
pub use meta::{EnumMetadata, FieldMetadata, StructMetadata, TypeMetadata};

use std::{fs, io};

pub fn parasitize() -> io::Result<()> {
    fs::write("schema.sql", generate_sql())?;
    fs::write("index.html", generate_html())?;

    println!("Crab-wORM: schema.sql e index.html gerados");
    Ok(())
}

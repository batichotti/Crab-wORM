#[derive(Debug)]
pub struct FieldMetadata {
    pub name: &'static str,
    pub ty: &'static str,
}

impl FieldMetadata {
    pub const fn new(name: &'static str, ty: &'static str) -> Self {
        Self { name, ty }
    }

    pub fn print(&self) {
        println!("Name: {}; Type: {}", self.name, self.ty);
    }
}

// STRUCT

#[derive(Debug)]
pub struct StructMetadata {
    pub name: &'static str,
    pub fields: &'static [FieldMetadata],
}

impl StructMetadata {
    pub const fn new(name: &'static str, fields: &'static [FieldMetadata]) -> Self {
        Self { name, fields }
    }

    pub fn print(&self) {
        println!("Struct: {}\nFields:", self.name);
        for field in self.fields {
            print!(" - ");
            field.print();
        }
    }
}

// ENUM

#[derive(Debug)]
pub struct EnumMetadata {
    pub name: &'static str,
    pub variants: &'static [&'static str],
}

impl EnumMetadata {
    pub const fn new(name: &'static str, variants: &'static [&'static str]) -> Self {
        Self { name, variants }
    }

    pub fn print(&self) {
        println!("Enum: {}\nVariants:", self.name);
        for variant in self.variants {
            println!(" - {}", variant);
        }
    }
}

// WRAPPER

#[derive(Debug)]
pub enum TypeMetadata {
    Struct(StructMetadata),
    Enum(EnumMetadata),
}

inventory::collect!(TypeMetadata);

impl TypeMetadata {
    pub fn name(&self) -> &'static str {
        match self {
            Self::Struct(s) => s.name,
            Self::Enum(e) => e.name,
        }
    }

    pub fn print(&self) {
        match self {
            Self::Struct(s) => s.print(),
            Self::Enum(e) => e.print(),
        }
    }
}
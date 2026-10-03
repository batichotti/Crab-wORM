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

    pub fn to_html(&self) -> String {
        // TODO: criar uma estrutura dinamica de HTML para cada tipo base (SQL) dos campos em questao
        // Aqui certos problemas devem ser mitigados:
        // 1. Structs podem ter campos que sao structs (rust nao tem heranca, entao so precisamos lidar com composicao) -> minha sugestao aqui eh uma busca recursiva
        // 2. Cardinalidade
        // 3. Agregados Homogeneos de dados
        String::from("html bem projetadinho e com um css coisa mais linda kkkkkkkkj\n")
    }

    pub fn to_sql(&self) -> String {
        let columns: Vec<String> = self
            .fields
            .iter()
            .filter_map(|f| column_def(f))
            .collect();

        if columns.is_empty() {
            return String::new();
        }

        format!(
            "CREATE TABLE IF NOT EXISTS {} (\n{}\n);",
            self.name,
            columns.join(",\n")
        )
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

    pub fn to_html(&self) -> String {
        let options: String = self
            .variants
            .iter()
            .enumerate()
            .map(|(i, v)| format!("\t<option value=\"{}\">{v}</option>\n", i + 1))
            .collect();

        format!("<select name=\"{}_id\">\n{}</select>\n", self.name, options)
    }

    /// Tabela `(id, name)` + populacao das variantes.
    /// O id e a posicao da variante (1, 2, 3...).
    /// `ON CONFLICT ... DO UPDATE` funciona no SQLite 3.24+ e no Postgres.
    pub fn to_sql(&self) -> String {
        if self.variants.is_empty() {
            return String::new();
        }

        let values: Vec<String> = self
            .variants
            .iter()
            .enumerate()
            .map(|(i, v)| format!("({}, '{}')", i + 1, v))
            .collect();

        format!(
            "CREATE TABLE IF NOT EXISTS {name} (\n\tid INTEGER PRIMARY KEY,\n\tname TEXT NOT NULL UNIQUE\n);\nINSERT INTO {name} (id, name) VALUES {values}\nON CONFLICT (id) DO UPDATE SET name = excluded.name;",
            name = self.name,
            values = values.join(", ")
        )
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

    pub fn to_html(&self) -> String {
        match self {
            Self::Struct(s) => s.to_html(),
            Self::Enum(e) => e.to_html(),
        }
    }

    pub fn to_sql(&self) -> String {
        match self {
            Self::Struct(s) => s.to_sql(),
            Self::Enum(e) => e.to_sql(),
        }
    }
}

// Acessorios

fn last_segment(ty: &str) -> &str {
    ty.rsplit("::").next().unwrap_or(ty).trim()
}

fn sql_type(ty: &str) -> Option<&'static str> {
    match last_segment(ty) {
        "i8" | "i16" | "i32" | "i64" | "i128" | "isize" | "u8" | "u16" | "u32" | "u64" | "u128" | "usize" | "bool" => Some("INTEGER"),
        "f32" | "f64" => Some("REAL"),
        "String" | "char" => Some("TEXT"),
        _ => None,
    }
}

/// Procura um enum registrado pelo nome do tipo.
fn find_enum(ty: &str) -> Option<&'static EnumMetadata> {
    let name = last_segment(ty);
    inventory::iter::<TypeMetadata>
        .into_iter()
        .find_map(|t| match t {
            TypeMetadata::Enum(e) if e.name == name => Some(e),
            _ => None,
        })
}

/// Definicao da coluna de um campo (None se o tipo nao for suportado).
fn column_def(f: &FieldMetadata) -> Option<String> {
    if let Some(ty) = sql_type(f.ty) {
        return Some(format!("\t{} {}", f.name, ty));
    }
    find_enum(f.ty).map(|e| {
        format!(
            "\t{}_id INTEGER NOT NULL REFERENCES {}(id)",
            f.name, e.name
        )
    })
}
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
        // Colunas da tabela principal (tudo que nao e Vec)
        let columns: Vec<String> = self
            .fields
            .iter()
            .filter(|f| f.name != "id") // a PK "id" ja e gerada automaticamente
            .filter(|f| vec_inner(f.ty).is_none())
            .filter_map(|f| single_column(f.name, f.ty))
            .collect();

        // Tabelas filhas (um por campo Vec)
        let children: Vec<String> = self
            .fields
            .iter()
            .filter_map(|f| vec_table_def(self.name, f))
            .collect();

        if columns.is_empty() && children.is_empty() {
            return String::new();
        }

        let mut all_columns = vec![String::from("\tid INTEGER PRIMARY KEY")];
        all_columns.extend(columns);

        let mut statements = vec![format!(
            "CREATE TABLE IF NOT EXISTS {} (\n{}\n);",
            self.name,
            all_columns.join(",\n")
        )];
        statements.extend(children);
        statements.join("\n\n")
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

    pub fn to_sql(&self) -> String {
        if self.variants.is_empty() {
            return String::new();
        }

        let values = self
            .variants
            .iter()
            .enumerate()
            .map(|(i, v)| format!("({}, '{}')", i + 1, v))
            .collect::<Vec<_>>()
            .join(", ");

        format!(
            "CREATE TABLE IF NOT EXISTS {name} (
        id INTEGER PRIMARY KEY,
        name TEXT NOT NULL UNIQUE
    );
    INSERT INTO {name} (id, name) VALUES {values}
    ON CONFLICT (id) DO UPDATE SET name = excluded.name;",
            name = self.name,
            values = values
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
        "i8" | "i16" | "i32" | "i64" | "i128" | "isize" | "u8" | "u16" | "u32" | "u64"
        | "u128" | "usize" | "bool" => Some("INTEGER"),
        "f32" | "f64" => Some("REAL"),
        "String" | "char" => Some("TEXT"),
        _ => None,
    }
}

fn vec_inner(ty: &str) -> Option<String> {
    let t: String = ty.split_whitespace().collect();
    let start = t.find('<')?;
    if last_segment(&t[..start]) != "Vec" || !t.ends_with('>') {
        return None;
    }
    Some(t[start + 1..t.len() - 1].to_string())
}

fn find_enum(ty: &str) -> Option<&'static EnumMetadata> {
    let name = last_segment(ty);
    inventory::iter::<TypeMetadata>
        .into_iter()
        .find_map(|t| match t {
            TypeMetadata::Enum(e) if e.name == name => Some(e),
            _ => None,
        })
}

fn find_struct(ty: &str) -> Option<&'static StructMetadata> {
    let name = last_segment(ty);
    inventory::iter::<TypeMetadata>
        .into_iter()
        .find_map(|t| match t {
            TypeMetadata::Struct(s) if s.name == name => Some(s),
            _ => None,
        })
}

fn single_column(name: &str, ty: &str) -> Option<String> {
    if let Some(sql) = sql_type(ty) {
        return Some(format!("\t{} {}", name, sql));
    }
    if let Some(e) = find_enum(ty) {
        return Some(format!(
            "\t{}_id INTEGER NOT NULL REFERENCES {}(id)",
            name, e.name
        ));
    }
    find_struct(ty).map(|s| {
        format!(
            "\t{}_id INTEGER NOT NULL REFERENCES {}(id)",
            name, s.name
        )
    })
}

fn vec_table_def(parent: &str, f: &FieldMetadata) -> Option<String> {
    let inner = vec_inner(f.ty)?;
    let mut value = single_column("value", &inner)?;
    if find_struct(&inner).is_some() {
        value.push_str(" ON DELETE CASCADE");
    }

    Some(format!(
        "CREATE TABLE IF NOT EXISTS {parent}_{field} (\n\
         \tid INTEGER PRIMARY KEY,\n\
         \t{parent}_id INTEGER NOT NULL REFERENCES {parent}(id) ON DELETE CASCADE,\n\
         \tposition INTEGER NOT NULL,\n\
         {value}\n);",
        parent = parent,
        field = f.name,
        value = value
    ))
}

pub fn generate_sql() -> String {
    let types: Vec<&'static TypeMetadata> = inventory::iter::<TypeMetadata>.into_iter().collect();

    let (enums, structs): (Vec<_>, Vec<_>) = types
        .into_iter()
        .partition(|t| matches!(t, TypeMetadata::Enum(_)));

    enums
        .into_iter()
        .chain(structs)
        .map(|t| t.to_sql())
        .filter(|s| !s.is_empty())
        .collect::<Vec<_>>()
        .join("\n\n")
}
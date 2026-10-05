use std::collections::HashSet;

use crate::meta::{EnumMetadata, FieldMetadata, StructMetadata, TypeMetadata};

// STRUCT

impl StructMetadata {
    pub fn to_html(&self) -> String {
        // TODO: criar uma estrutura dinamica de HTML para cada tipo base (SQL) dos campos em questao
        // Aqui certos problemas devem ser mitigados:
        // 1. Structs podem ter campos que sao structs (rust nao tem heranca, entao so precisamos lidar com composicao) -> minha sugestao aqui eh uma busca recursiva
        // 2. Cardinalidade
        // 3. Agregados Homogeneos de dados
        // 4. Agregados Heterogeno de dados (talvez usar o sql possa ser a solucao)
        String::from("html bem projetadinho e com um css coisa mais linda kkkkkkkkj\n")
    }

    pub fn to_sql(&self) -> String {
        let columns: Vec<String> = self
            .fields
            .iter()
            .filter(|f| f.name != "id")
            .filter(|f| vec_inner(f.ty).is_none())
            .map(|f| single_column(&format!("{}.{}", self.name, f.name), f.name, f.ty))
            .collect();

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

impl EnumMetadata {
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

impl TypeMetadata {
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

pub fn generate_html() -> String {
    inventory::iter::<TypeMetadata>()
        .into_iter()
        .map(|t| t.to_html())
        .filter(|s| !s.is_empty())
        .collect::<Vec<_>>()
        .join("\n\n")
}

pub fn generate_sql() -> String {
    let mut all: Vec<&'static TypeMetadata> =
        inventory::iter::<TypeMetadata>.into_iter().collect();

    all.sort_by_key(|t| (matches!(t, TypeMetadata::Struct(_)), t.name()));

    let mut visited: HashSet<&'static str> = HashSet::new();
    let mut ordered: Vec<&'static TypeMetadata> = Vec::new();

    for t in &all {
        visit(t, &all, &mut visited, &mut ordered);
    }

    ordered
        .into_iter()
        .map(|t| t.to_sql())
        .filter(|s| !s.is_empty())
        .collect::<Vec<_>>()
        .join("\n\n")
}

/// DFS: visita primeiro as dependencias de `t` e so depois o adiciona em `ordered`.
/// O tipo e marcado como visitado antes de descer, o que quebra ciclos.
fn visit(
    t: &'static TypeMetadata,
    all: &[&'static TypeMetadata],
    visited: &mut HashSet<&'static str>,
    ordered: &mut Vec<&'static TypeMetadata>,
) {
    if !visited.insert(t.name()) {
        return; // ja ordenado, ou em andamento (ciclo)
    }

    if let TypeMetadata::Struct(s) = t {
        for f in s.fields {
            let Some(dep_name) = dependency_name(f.ty) else {
                continue;
            };
            if let Some(dep) = all.iter().find(|x| x.name() == dep_name) {
                visit(dep, all, visited, ordered);
            }
        }
    }

    ordered.push(t);
}

fn dependency_name(ty: &str) -> Option<&'static str> {
    let t = strip_option(ty);
    let inner = match vec_inner(&t) {
        Some(v) => strip_option(&v),
        None => t,
    };

    if let Some(e) = find_enum(&inner) {
        return Some(e.name);
    }
    find_struct(&inner).map(|s| s.name)
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

fn generic_inner(ty: &str, wrapper: &str) -> Option<String> {
    let t: String = ty.split_whitespace().collect();
    let start = t.find('<')?;
    if last_segment(&t[..start]) != wrapper || !t.ends_with('>') {
        return None;
    }
    Some(t[start + 1..t.len() - 1].to_string())
}

fn vec_inner(ty: &str) -> Option<String> {
    generic_inner(ty, "Vec")
}

fn option_inner(ty: &str) -> Option<String> {
    generic_inner(ty, "Option")
}

fn strip_option(ty: &str) -> String {
    let mut t = ty.to_string();
    while let Some(inner) = option_inner(&t) {
        t = inner;
    }
    t
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

fn single_column(field: &str, name: &str, ty: &str) -> String {
    column_def(field, name, ty, false)
}

fn column_def(field: &str, name: &str, ty: &str, nullable: bool) -> String {
    if let Some(inner) = option_inner(ty) {
        return column_def(field, name, &inner, true);
    }

    if vec_inner(ty).is_some() {
        panic!(
            "tipo nao suportado: o campo `{field}` tem tipo `{ty}`; `Vec` aninhado \
             ou dentro de `Option` nao e suportado"
        );
    }

    let not_null = if nullable { "" } else { " NOT NULL" };

    if let Some(sql) = sql_type(ty) {
        return format!("\t{} {}", name, sql);
    }
    if let Some(e) = find_enum(ty) {
        return format!("\t{}_id INTEGER{} REFERENCES {}(id)", name, not_null, e.name);
    }
    if let Some(s) = find_struct(ty) {
        return format!("\t{}_id INTEGER{} REFERENCES {}(id)", name, not_null, s.name);
    }
    panic!(
        "tipo nao registrado: o campo `{field}` tem tipo `{ty}`, que nao e um \
         escalar suportado nem possui `#[derive(crab_worm)]`"
    );
}

fn vec_table_def(parent: &str, f: &FieldMetadata) -> Option<String> {
    let inner = vec_inner(f.ty)?;
    let mut value = single_column(&format!("{}.{}", parent, f.name), "value", &inner);
    if find_struct(&strip_option(&inner)).is_some() {
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
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

#[derive(Debug)]
pub struct StructMetadata {
    pub name: &'static str,
    pub fields: &'static [FieldMetadata],
}

inventory::collect!(StructMetadata);

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
        String::from("html bem projetadinho e com um css coisa mais linda kkkkkkkkj\n")
    }

    pub fn to_sql(&self) -> String {
        let columns: Vec<String> = self
            .fields
            .iter()
            .filter_map(|f| sql_type(f.ty).map(|ty| format!("\t{} {}", f.name, ty)))
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

fn sql_type(ty: &str) -> Option<&'static str> {
    let name = ty.rsplit("::").next()?.trim();
    match name {
        "i8" | "i16" | "i32" | "i64" | "i128" | "isize" | "u8" | "u16" | "u32" | "u64" | "u128" | "usize" | "bool" => Some("INTEGER"),
        "f32" | "f64" => Some("REAL"),
        "String" | "char" => Some("TEXT"),
        _ => None,
    }
}
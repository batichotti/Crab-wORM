use std::{env, error::Error, fs, path::Path};

use postgres::{Client, NoTls};

pub const SCHEMA_FILE: &str = "schema.sql";

const DROP_ALL_TABLES: &str = r#"
DO $$
DECLARE r RECORD;
BEGIN
    FOR r IN SELECT tablename FROM pg_tables WHERE schemaname = 'public' LOOP
        EXECUTE format('DROP TABLE IF EXISTS public.%I CASCADE', r.tablename);
    END LOOP;
END $$;
"#;

/// `postgres::Error` imprime so "db error"; isto extrai o motivo real do servidor.
fn pg_msg(e: &postgres::Error) -> String {
    match e.as_db_error() {
        Some(db) => format!("{} (SQLSTATE {})", db.message(), db.code().code()),
        None => match e.source() {
            Some(src) => format!("{e}: {src}"),
            None => e.to_string(),
        },
    }
}


pub fn database_url() -> Result<String, Box<dyn Error>> {
    let _ = dotenvy::dotenv();
    env::var("DATABASE_URL").map_err(|_| -> Box<dyn Error> {
        "DATABASE_URL nao definida: exporte a variavel ou crie um .env \
         (ex.: postgres://usuario:senha@127.0.0.1:5432/banco)"
            .into()
    })
}

pub fn apply_schema_file(path: impl AsRef<Path>) -> Result<(), Box<dyn Error>> {
    let path = path.as_ref();
    let sql = fs::read_to_string(path)
        .map_err(|e| format!("nao consegui ler `{}`: {e}", path.display()))?;

    let reset = env::var("CRAB_RESET").is_ok_and(|v| v == "1");
    let sql = if reset {
        println!("Crab-wORM: CRAB_RESET=1, apagando todas as tabelas de `public` antes de aplicar");
        // NOTIFY avisa o PostgREST para recarregar o cache de colunas
        format!("{DROP_ALL_TABLES}\n{sql}\nNOTIFY pgrst, 'reload schema';")
    } else {
        sql
    };

    let url = database_url()?;
    let mut client = Client::connect(&url, NoTls)
        .map_err(|e| format!("nao consegui conectar no Postgres: {}", pg_msg(&e)))?;

    let mut tx = client.transaction()?;
    tx.batch_execute(&sql).map_err(|e| {
        format!(
            "erro ao executar `{}` (nada foi aplicado): {}",
            path.display(),
            pg_msg(&e)
        )
    })?;
    tx.commit()
        .map_err(|e| format!("erro ao confirmar a transacao: {}", pg_msg(&e)))?;

    fs::remove_file(path)?;
    println!("Crab-wORM: schema aplicado no Postgres e `{}` removido", path.display());
    Ok(())
}
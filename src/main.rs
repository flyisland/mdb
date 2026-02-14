mod db;
mod extractor;
mod query;
mod scanner;

use clap::{Parser, Subcommand};
use std::path::PathBuf;
use std::sync::Mutex;

use crate::db::Database;

#[derive(Parser)]
#[command(name = "mdb")]
#[command(version = "0.1.0")]
#[command(about = "Markdown database CLI - index and query markdown files", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,

    #[arg(short, long, default_value = ".mdb/mdb.duckdb")]
    database: PathBuf,
}

#[derive(Subcommand)]
enum Commands {
    Index {
        #[arg(short = 'b', long = "base-dir", default_value = ".")]
        base_dir: PathBuf,

        #[arg(short, long)]
        force: bool,

        #[arg(short, long)]
        verbose: bool,
    },
    Query {
        #[arg(short, long)]
        query: String,

        #[arg(short, long, default_value = "table")]
        format: String,

        #[arg(short, long, default_value_t = 1000)]
        limit: usize,

        #[arg(short = 'F', long, default_value = "*")]
        fields: String,
    },
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    let db = Mutex::new(Database::new(&cli.database)?);

    match cli.command {
        Commands::Index {
            base_dir,
            force,
            verbose,
        } => {
            let db = db.lock().unwrap();
            scanner::index_directory(&base_dir, &db, force, verbose)?;
        }
        Commands::Query {
            query,
            format,
            limit,
            fields,
        } => {
            let compiled = query::build_sql(&query, &fields).map_err(|e| e.to_string())?;
            let db = db.lock().unwrap();
            let results = db.query(&compiled, &fields, limit)?;
            query::output_results(&results, &format)?;
        }
    }

    Ok(())
}

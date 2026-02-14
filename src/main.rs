mod db;
mod extractor;
mod query;
mod scanner;

use clap::{Parser, Subcommand, ValueEnum};
use std::path::PathBuf;
use std::sync::Mutex;

use crate::db::Database;

#[derive(Clone, ValueEnum, Debug, PartialEq)]
enum OutputFormat {
    Table,
    Json,
    List,
}

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

        #[arg(short = 'o', long = "output-format", default_value = "table")]
        format: OutputFormat,

        #[arg(
            short = 'f',
            long = "output-fields",
            default_value = "file.path, file.mtime"
        )]
        fields: String,

        #[arg(short, long, default_value_t = 1000)]
        limit: usize,
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
            let field_names: Vec<String> =
                fields.split(',').map(|s| s.trim().to_string()).collect();
            let format_str = match format {
                OutputFormat::Table => "table",
                OutputFormat::Json => "json",
                OutputFormat::List => "list",
            };
            let compiled = query::build_sql(&query, &fields).map_err(|e| e.to_string())?;
            let db = db.lock().unwrap();
            let results = db.query(&compiled, &fields, limit)?;
            query::output_results(&results, format_str, &field_names)?;
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_fields_value() {
        let cli = Cli::parse_from(["mdb", "query", "-q", "file.name == 'test'"]);
        if let Commands::Query { fields, .. } = cli.command {
            assert_eq!(fields, "file.path, file.mtime");
        } else {
            panic!("Expected Query command");
        }
    }

    #[test]
    fn test_all_fields_option() {
        let cli = Cli::parse_from(["mdb", "query", "-q", "file.name == 'test'", "-f", "*"]);
        if let Commands::Query { fields, .. } = cli.command {
            assert_eq!(fields, "*");
        } else {
            panic!("Expected Query command");
        }
    }

    #[test]
    fn test_specific_field_option() {
        let cli = Cli::parse_from([
            "mdb",
            "query",
            "-q",
            "file.name == 'test'",
            "--output-fields",
            "file.name",
        ]);
        if let Commands::Query { fields, .. } = cli.command {
            assert_eq!(fields, "file.name");
        } else {
            panic!("Expected Query command");
        }
    }

    #[test]
    fn test_output_format_option() {
        let cli = Cli::parse_from(["mdb", "query", "-q", "file.name == 'test'", "-o", "json"]);
        if let Commands::Query { format, .. } = cli.command {
            assert_eq!(format, OutputFormat::Json);
        } else {
            panic!("Expected Query command");
        }
    }
}

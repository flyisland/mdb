pub mod compiler;
pub mod parser;
pub mod tokenizer;

pub use compiler::build_sql;

pub fn output_results(
    results: &[Vec<String>],
    format: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    match format {
        "json" => output_json(results),
        "list" => output_list(results),
        _ => output_table(results),
    }
}

fn output_json(results: &[Vec<String>]) -> Result<(), Box<dyn std::error::Error>> {
    let json_results: Vec<serde_json::Value> = results
        .iter()
        .map(|row| {
            let obj: serde_json::Map<String, serde_json::Value> = row
                .iter()
                .enumerate()
                .map(|(i, v)| (format!("col{}", i), serde_json::Value::String(v.clone())))
                .collect();
            serde_json::Value::Object(obj)
        })
        .collect();

    println!("{}", serde_json::to_string_pretty(&json_results)?);
    Ok(())
}

fn output_list(results: &[Vec<String>]) -> Result<(), Box<dyn std::error::Error>> {
    for row in results {
        for (i, col) in row.iter().enumerate() {
            if i > 0 {
                println!("{}: {}", i, col);
            } else {
                println!("{}", col);
            }
        }
        println!("---");
    }
    Ok(())
}

fn output_table(results: &[Vec<String>]) -> Result<(), Box<dyn std::error::Error>> {
    if results.is_empty() {
        return Ok(());
    }

    let col_widths: Vec<usize> = (0..results[0].len())
        .map(|i| {
            results
                .iter()
                .map(|row| row.get(i).map_or(0, |s| s.len()))
                .max()
                .unwrap_or(0)
                .max(10)
        })
        .collect();

    for row in results {
        let cells: Vec<String> = row
            .iter()
            .enumerate()
            .map(|(i, cell)| {
                if i < col_widths.len() {
                    format!("{:<width$}", cell, width = col_widths[i])
                } else {
                    cell.clone()
                }
            })
            .collect();
        println!("{}", cells.join(" | "));
    }
    Ok(())
}

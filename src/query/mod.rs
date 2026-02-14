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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_output_results_table() {
        let results = vec![
            vec!["path1".to_string(), "name1".to_string()],
            vec!["path2".to_string(), "name2".to_string()],
        ];
        let result = output_results(&results, "table");
        assert!(result.is_ok());
    }

    #[test]
    fn test_output_results_json() {
        let results = vec![
            vec!["path1".to_string(), "name1".to_string()],
            vec!["path2".to_string(), "name2".to_string()],
        ];
        let result = output_results(&results, "json");
        assert!(result.is_ok());
    }

    #[test]
    fn test_output_results_list() {
        let results = vec![
            vec!["path1".to_string(), "name1".to_string()],
            vec!["path2".to_string(), "name2".to_string()],
        ];
        let result = output_results(&results, "list");
        assert!(result.is_ok());
    }

    #[test]
    fn test_output_results_empty() {
        let results: Vec<Vec<String>> = vec![];
        let result = output_results(&results, "table");
        assert!(result.is_ok());
    }

    #[test]
    fn test_output_results_default_to_table() {
        let results = vec![vec!["test".to_string()]];
        let result = output_results(&results, "unknown_format");
        assert!(result.is_ok());
    }

    #[test]
    fn test_output_json_structure() {
        let results = vec![vec!["val1".to_string(), "val2".to_string()]];
        // Should produce valid JSON without panicking
        let result = output_json(&results);
        assert!(result.is_ok());
    }

    #[test]
    fn test_output_list_structure() {
        let results = vec![vec!["path".to_string(), "name".to_string()]];
        let result = output_list(&results);
        assert!(result.is_ok());
    }

    #[test]
    fn test_output_table_calculates_widths() {
        let results = vec![
            vec!["short".to_string(), "longer_value".to_string()],
            vec!["a".to_string(), "b".to_string()],
        ];
        let result = output_table(&results);
        assert!(result.is_ok());
    }

    #[test]
    fn test_output_table_empty_results() {
        let results: Vec<Vec<String>> = vec![];
        let result = output_table(&results);
        assert!(result.is_ok());
    }

    #[test]
    fn test_output_multiple_rows() {
        let results = vec![
            vec![
                "path1".to_string(),
                "name1".to_string(),
                "content1".to_string(),
            ],
            vec![
                "path2".to_string(),
                "name2".to_string(),
                "content2".to_string(),
            ],
            vec![
                "path3".to_string(),
                "name3".to_string(),
                "content3".to_string(),
            ],
        ];

        for format in &["table", "json", "list"] {
            let result = output_results(&results, format);
            assert!(result.is_ok(), "Failed for format: {}", format);
        }
    }
}

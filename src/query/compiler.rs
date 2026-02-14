use super::parser::AstNode;

const FILE_FIELDS: &[&str] = &["path", "folder", "name", "ext", "size", "ctime", "mtime"];
const NOTE_FIELDS: &[&str] = &[
    "content",
    "tags",
    "links",
    "backlinks",
    "embeds",
    "properties",
];

pub fn resolve_field(field: &str) -> String {
    if field.contains('.') {
        let parts: Vec<&str> = field.split('.').collect();
        if parts.len() == 2 {
            let prefix = parts[0];
            let name = parts[1];
            if prefix == "file" && FILE_FIELDS.contains(&name) {
                return name.to_string();
            }
            if prefix == "note" {
                if NOTE_FIELDS.contains(&name) {
                    return name.to_string();
                }
                return format!("json_extract(properties, '$.{}')", name);
            }
        }
        return field.to_string();
    }

    if FILE_FIELDS.contains(&field) {
        return field.to_string();
    }

    if NOTE_FIELDS.contains(&field) {
        return field.to_string();
    }

    format!("json_extract(properties, '$.{}')", field)
}

pub fn compile(node: &AstNode) -> String {
    match node {
        AstNode::Binary { left, op, right } => {
            let left_sql = compile(left);
            let right_sql = compile(right);

            let sql_op = match op.as_str() {
                "AND" => "AND",
                "OR" => "OR",
                "==" => "=",
                "!=" => "!=",
                ">" => ">",
                "<" => "<",
                ">=" => ">=",
                "<=" => "<=",
                "=~" => "LIKE",
                _ => "=",
            };

            if op == "=~" {
                format!("{} LIKE {}", left_sql, right_sql)
            } else {
                format!("{} {} {}", left_sql, sql_op, right_sql)
            }
        }
        AstNode::Field(name) => {
            let resolved = resolve_field(name);
            resolved
        }
        AstNode::StringLiteral(val) => {
            format!("'{}'", val.replace('\'', "''"))
        }
        AstNode::NumberLiteral(val) => val.clone(),
        AstNode::FunctionCall { name, args } => {
            if name == "has" && args.len() == 2 {
                let field = compile(&args[0]);
                let value = compile(&args[1]);
                let clean_value = value.trim_matches('\'');
                return format!("{} LIKE '%{}%'", field, clean_value);
            }
            "1=1".to_string()
        }
        AstNode::Grouping(expr) => {
            format!("({})", compile(expr))
        }
    }
}

pub fn build_sql(query: &str, fields: &str) -> Result<String, String> {
    let parsed = super::parser::parse(query);
    let where_clause = compile(&parsed);

    let select_fields: String = if fields == "*" {
        "path, folder, name, ext, size, ctime, mtime, content, tags, links, backlinks, embeds, properties".to_string()
    } else {
        let resolved: Vec<String> = fields.split(',').map(|f| resolve_field(f.trim())).collect();
        resolved.join(", ")
    };

    Ok(format!(
        "SELECT {} FROM documents WHERE {}",
        select_fields, where_clause
    ))
}

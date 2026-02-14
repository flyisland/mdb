use duckdb::{params, Connection};
use serde::{Deserialize, Serialize};
use std::path::Path;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Document {
    pub path: String,
    pub folder: String,
    pub name: String,
    pub ext: String,
    pub size: u64,
    pub ctime: i64,
    pub mtime: i64,
    pub content: String,
    pub tags: Vec<String>,
    pub links: Vec<String>,
    pub backlinks: Vec<String>,
    pub embeds: Vec<String>,
    pub properties: serde_json::Value,
}

pub struct Database {
    conn: Connection,
}

impl Database {
    pub fn new(path: &Path) -> Result<Self, Box<dyn std::error::Error>> {
        let conn = Connection::open(path)?;
        let db = Database { conn };
        db.init_schema()?;
        Ok(db)
    }

    fn init_schema(&self) -> Result<(), Box<dyn std::error::Error>> {
        self.conn.execute(
            "CREATE TABLE IF NOT EXISTS documents (
                path TEXT PRIMARY KEY,
                folder TEXT NOT NULL,
                name TEXT NOT NULL,
                ext TEXT NOT NULL,
                size INTEGER NOT NULL,
                ctime BIGINT NOT NULL,
                mtime BIGINT NOT NULL,
                content TEXT,
                tags TEXT,
                links TEXT,
                backlinks TEXT,
                embeds TEXT,
                properties TEXT
            )",
            [],
        )?;

        self.conn.execute(
            "CREATE INDEX IF NOT EXISTS idx_mtime ON documents(mtime)",
            [],
        )?;
        self.conn.execute(
            "CREATE INDEX IF NOT EXISTS idx_folder ON documents(folder)",
            [],
        )?;
        self.conn
            .execute("CREATE INDEX IF NOT EXISTS idx_name ON documents(name)", [])?;

        Ok(())
    }

    pub fn upsert_document(&self, doc: &Document) -> Result<(), Box<dyn std::error::Error>> {
        self.conn.execute(
            "INSERT OR REPLACE INTO documents 
             (path, folder, name, ext, size, ctime, mtime, content, tags, links, backlinks, embeds, properties)
             VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)",
            params![
                &doc.path,
                &doc.folder,
                &doc.name,
                &doc.ext,
                doc.size as i64,
                doc.ctime,
                doc.mtime,
                &doc.content,
                serde_json::to_string(&doc.tags)?,
                serde_json::to_string(&doc.links)?,
                serde_json::to_string(&doc.backlinks)?,
                serde_json::to_string(&doc.embeds)?,
                serde_json::to_string(&doc.properties)?,
            ],
        )?;
        Ok(())
    }

    pub fn get_mtime(&self, path: &str) -> Result<Option<i64>, Box<dyn std::error::Error>> {
        let mut stmt = self
            .conn
            .prepare("SELECT mtime FROM documents WHERE path = ?")?;
        let mut rows = stmt.query(params![path])?;

        if let Some(row) = rows.next()? {
            let mtime: i64 = row.get(0)?;
            Ok(Some(mtime))
        } else {
            Ok(None)
        }
    }

    pub fn get_all_links(
        &self,
    ) -> Result<std::collections::HashMap<String, Vec<String>>, Box<dyn std::error::Error>> {
        let mut stmt = self.conn.prepare("SELECT path, links FROM documents")?;
        let mut rows = stmt.query([])?;

        let mut link_map = std::collections::HashMap::new();
        while let Some(row) = rows.next()? {
            let path: String = row.get(0)?;
            let links_json: String = row.get(1)?;
            let links: Vec<String> = serde_json::from_str(&links_json).unwrap_or_default();
            link_map.insert(path, links);
        }

        Ok(link_map)
    }

    pub fn query(
        &self,
        sql: &str,
        _fields: &str,
        limit: usize,
    ) -> Result<Vec<Vec<String>>, Box<dyn std::error::Error>> {
        let sql = format!("{} LIMIT {}", sql, limit);

        let mut results = Vec::new();

        let con = self
            .conn
            .try_clone()
            .map_err(|e| format!("Clone error: {}", e))?;

        let mut stmt = con.prepare(&sql)?;
        let mut rows = stmt.query([])?;

        while let Some(row) = rows.next()? {
            let mut result_row = Vec::new();
            for i in 0..13 {
                let val: duckdb::types::Value = row.get(i)?;
                let s = match val {
                    duckdb::types::Value::Text(t) => t,
                    duckdb::types::Value::Int(i) => i.to_string(),
                    duckdb::types::Value::BigInt(i) => i.to_string(),
                    duckdb::types::Value::Double(d) => d.to_string(),
                    duckdb::types::Value::Float(f) => f.to_string(),
                    duckdb::types::Value::Boolean(b) => b.to_string(),
                    _ => String::new(),
                };
                result_row.push(s);
            }
            results.push(result_row);
        }

        Ok(results)
    }
}

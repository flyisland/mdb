use crate::db::{Database, Document};
use crate::extractor::Extractor;
use std::fs;
use std::path::Path;
use std::time::UNIX_EPOCH;
use walkdir::WalkDir;

pub fn index_directory(
    dir: &Path,
    db: &Database,
    force: bool,
    verbose: bool,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut count = 0;
    let mut all_docs: Vec<Document> = Vec::new();

    for entry in WalkDir::new(dir)
        .follow_links(true)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        let path = entry.path();
        if path.is_file() && path.extension().map_or(false, |ext| ext == "md") {
            let path_str = path.to_string_lossy().to_string();

            if !force {
                if let Some(db_mtime) = db.get_mtime(&path_str)? {
                    let file_mtime = fs::metadata(path)?
                        .modified()?
                        .duration_since(UNIX_EPOCH)?
                        .as_secs() as i64;
                    if file_mtime <= db_mtime {
                        continue;
                    }
                }
            }

            let metadata = fs::metadata(path)?;
            let file_name = path.file_name().unwrap().to_string_lossy().to_string();
            let parent = path.parent().unwrap().to_string_lossy().to_string();

            let content = fs::read_to_string(path)?;
            let extracted = Extractor::extract(&content);

            let size = metadata.len();
            let ctime = metadata.created()?.duration_since(UNIX_EPOCH)?.as_secs() as i64;
            let mtime = metadata.modified()?.duration_since(UNIX_EPOCH)?.as_secs() as i64;

            let doc = Document {
                path: path_str,
                folder: parent,
                name: file_name.trim_end_matches(".md").to_string(),
                ext: "md".to_string(),
                size,
                ctime,
                mtime,
                content: extracted.content,
                tags: extracted.tags,
                links: extracted.links,
                backlinks: vec![],
                embeds: extracted.embeds,
                properties: extracted.frontmatter,
            };

            db.upsert_document(&doc)?;
            if verbose {
                println!("Indexed: {}", doc.path);
            }
            all_docs.push(doc.clone());
            count += 1;
        }
    }

    let link_map = db.get_all_links()?;
    let mut backlinks: std::collections::HashMap<String, Vec<String>> =
        std::collections::HashMap::new();

    for (path, links) in &link_map {
        for link in links {
            let link_name = link
                .trim_end_matches(|c: char| c == '|' || c == '#')
                .to_string();
            backlinks.entry(link_name).or_default().push(path.clone());
        }
    }

    for doc in &all_docs {
        if let Some(back_links) = backlinks.get(&doc.name) {
            let mut updated_doc = doc.clone();
            updated_doc.backlinks = back_links.clone();
            db.upsert_document(&updated_doc)?;
        }
    }

    println!("Indexed {} files", count);
    Ok(())
}

use regex::Regex;
use serde_json::Value;

static WIKILINK_REGEX: &str = r"\[\[([^\]]+)\]\]";
static EMBED_REGEX: &str = r"!\[\[([^\]]+)\]\]";
static TAG_REGEX: &str = r"#[\w\-/]+";

pub struct Extractor;

impl Extractor {
    pub fn extract(content: &str) -> ExtractedContent {
        let (frontmatter, content) = Self::parse_frontmatter(content);
        let tags = Self::extract_tags(&content);
        let links = Self::extract_wikilinks(&content);
        let embeds = Self::extract_embeds(&content);

        ExtractedContent {
            content,
            frontmatter,
            tags,
            links,
            embeds,
        }
    }

    fn parse_frontmatter(content: &str) -> (Value, String) {
        if content.starts_with("---") {
            if let Some(end_idx) = content[3..].find("---") {
                let yaml_content = &content[3..end_idx + 3];
                let remaining = &content[end_idx + 6..];

                if let Ok(props) = serde_yaml::from_str::<Value>(yaml_content) {
                    return (props, remaining.trim().to_string());
                }
            }
        }
        (Value::Null, content.to_string())
    }

    fn extract_tags(content: &str) -> Vec<String> {
        let re = Regex::new(TAG_REGEX).unwrap();
        re.find_iter(content)
            .map(|m| m.as_str().trim_start_matches('#').to_string())
            .collect()
    }

    fn extract_wikilinks(content: &str) -> Vec<String> {
        let re = Regex::new(WIKILINK_REGEX).unwrap();
        re.captures_iter(content)
            .filter_map(|cap| cap.get(1).map(|m| m.as_str().to_string()))
            .collect()
    }

    fn extract_embeds(content: &str) -> Vec<String> {
        let re = Regex::new(EMBED_REGEX).unwrap();
        re.captures_iter(content)
            .filter_map(|cap| cap.get(1).map(|m| m.as_str().to_string()))
            .collect()
    }
}

pub struct ExtractedContent {
    pub content: String,
    pub frontmatter: Value,
    pub tags: Vec<String>,
    pub links: Vec<String>,
    pub embeds: Vec<String>,
}

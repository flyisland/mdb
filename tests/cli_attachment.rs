mod common;

use common::TestVault;
use serde_json::Value;

const START: &str = "<!-- markbase:source-attachments:start -->";
const END: &str = "<!-- markbase:source-attachments:end -->";

fn source_note() -> String {
    format!(
        "---\ntype: source\n---\n\n# Source\n\n## 原始输入\n\nOriginal text.\n\n## 证据附件\n\n{}\n{}\n",
        START, END
    )
}

fn json(output: &std::process::Output) -> Value {
    serde_json::from_slice(&output.stdout).unwrap()
}

#[test]
fn test_source_attach_copies_file_writes_evidence_and_returns_json() {
    let vault = TestVault::new();
    vault.create_note_in_subdir("sources", "evidence", &source_note());
    let input = vault.create_file("outside/report.txt", "proof\n");

    let output = vault.run_cli(&[
        "source",
        "attach",
        "evidence",
        &input.to_string_lossy(),
        "--description",
        "Test proof",
    ]);
    assert!(
        output.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let value = json(&output);
    assert_eq!(value["status"], "copied");
    assert_eq!(value["attachment"]["mime_type"], "text/plain");
    assert_eq!(
        value["attachment"]["sha256"],
        "f6ed42a9d765eeb230a069bbc3d5dc346b2669594bb0b83cc6d14d5d967b8961"
    );
    let archived = vault
        .path
        .join(value["attachment"]["path"].as_str().unwrap());
    assert_eq!(std::fs::read_to_string(&archived).unwrap(), "proof\n");
    let source = std::fs::read_to_string(vault.path.join("sources/evidence.md")).unwrap();
    assert!(source.contains("Original text."));
    assert!(source.contains("Test proof"));
    assert!(source.contains("markbase:source-attachment"));
    let verified = vault.run_cli(&["source", "verify-attachments", "evidence"]);
    assert!(verified.status.success());
    assert_eq!(json(&verified)["ok"], true);
}

#[test]
fn test_source_attachment_links_percent_encode_paths_and_rerender_safely() {
    let vault = TestVault::new();
    vault.create_note_in_subdir("来源 文件夹", "source note", &source_note());
    let input = vault.create_file("输入 文件/2026-06 中文 #?.png", "image bytes");

    let attached = vault.run_cli(&[
        "source",
        "attach",
        "source note",
        &input.to_string_lossy(),
        "--description",
        "Unicode evidence",
    ]);
    assert!(
        attached.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&attached.stderr)
    );
    let attachment = json(&attached)["attachment"].clone();
    assert_eq!(
        attachment["path"],
        "来源 文件夹/attachments/source note/2026-06 中文 #?.png"
    );
    assert!(
        attachment["original_path"]
            .as_str()
            .unwrap()
            .ends_with("输入 文件/2026-06 中文 #?.png")
    );

    let source_path = vault.path.join("来源 文件夹/source note.md");
    let encoded_target = "attachments/source%20note/2026-06%20%E4%B8%AD%E6%96%87%20%23%3F.png";
    let unescaped_target = "attachments/source note/2026-06 中文 #?.png";
    let source = std::fs::read_to_string(&source_path).unwrap();
    assert!(source.contains(encoded_target));
    assert!(!source.contains(&format!("]({})", unescaped_target)));

    // Simulate the rows emitted by an older Markbase version while retaining
    // the authoritative JSON metadata, then migrate them through the CLI.
    std::fs::write(
        &source_path,
        source.replacen(encoded_target, unescaped_target, 1),
    )
    .unwrap();
    let rerendered = vault.run_cli(&["source", "rerender-attachments", "source note"]);
    assert!(
        rerendered.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&rerendered.stderr)
    );
    assert_eq!(json(&rerendered)["attachments"][0], attachment);
    let migrated = std::fs::read_to_string(&source_path).unwrap();
    assert!(migrated.contains(encoded_target));
    assert_eq!(
        json(&vault.run_cli(&["source", "verify-attachments", "source note"]))["ok"],
        true
    );
}

#[test]
fn test_source_attachment_links_keep_unreserved_paths_usable() {
    let vault = TestVault::new();
    vault.create_note_in_subdir("sources", "evidence", &source_note());
    let input = vault.create_file("outside/report.txt", "proof");

    let output = vault.run_cli(&[
        "source",
        "attach",
        "evidence",
        &input.to_string_lossy(),
        "--description",
        "Test proof",
    ]);
    assert!(output.status.success());
    let source = std::fs::read_to_string(vault.path.join("sources/evidence.md")).unwrap();
    assert!(source.contains("[report.txt](attachments/evidence/report.txt)"));
}

#[test]
fn test_source_attach_is_idempotent_for_same_content() {
    let vault = TestVault::new();
    vault.create_note_in_subdir("sources", "evidence", &source_note());
    let first = vault.create_file("one/report.txt", "same");
    let second = vault.create_file("two/report.txt", "same");
    assert!(
        vault
            .run_cli(&[
                "source",
                "attach",
                "evidence",
                &first.to_string_lossy(),
                "--description",
                "First"
            ])
            .status
            .success()
    );
    let output = vault.run_cli(&[
        "source",
        "attach",
        "evidence",
        &second.to_string_lossy(),
        "--description",
        "Second",
    ]);
    assert!(output.status.success());
    assert_eq!(json(&output)["status"], "existing");
    let listed = vault.run_cli(&["source", "attachments", "evidence"]);
    assert_eq!(json(&listed).as_array().unwrap().len(), 1);
}

#[test]
fn test_source_attach_disambiguates_same_filename_with_different_content() {
    let vault = TestVault::new();
    vault.create_note_in_subdir("sources", "evidence", &source_note());
    let first = vault.create_file("one/report.txt", "first");
    let second = vault.create_file("two/report.txt", "second");
    assert!(
        vault
            .run_cli(&[
                "source",
                "attach",
                "evidence",
                &first.to_string_lossy(),
                "--description",
                "First"
            ])
            .status
            .success()
    );
    let output = vault.run_cli(&[
        "source",
        "attach",
        "evidence",
        &second.to_string_lossy(),
        "--description",
        "Second",
    ]);
    assert!(output.status.success());
    assert_eq!(
        json(&output)["attachment"]["path"].as_str().unwrap(),
        "sources/attachments/evidence/report_02.txt"
    );
    assert_eq!(
        std::fs::read_to_string(vault.path.join("sources/attachments/evidence/report.txt"))
            .unwrap(),
        "first"
    );
}

#[test]
fn test_source_verify_attachments_detects_missing_and_tampered_files() {
    let vault = TestVault::new();
    vault.create_note_in_subdir("sources", "evidence", &source_note());
    let input = vault.create_file("outside/report.txt", "proof");
    let attach = vault.run_cli(&[
        "source",
        "attach",
        "evidence",
        &input.to_string_lossy(),
        "--description",
        "Proof",
    ]);
    let path = json(&attach)["attachment"]["path"]
        .as_str()
        .unwrap()
        .to_string();
    std::fs::write(vault.path.join(&path), "tampered").unwrap();
    let output = vault.run_cli(&["source", "verify-attachments", "evidence"]);
    assert!(!output.status.success());
    assert!(
        json(&output)["issues"]
            .as_array()
            .unwrap()
            .iter()
            .any(|issue| issue["code"] == "sha256_mismatch")
    );
    std::fs::remove_file(vault.path.join(&path)).unwrap();
    let output = vault.run_cli(&["source", "verify-attachments", "evidence"]);
    assert!(!output.status.success());
    assert!(
        json(&output)["issues"]
            .as_array()
            .unwrap()
            .iter()
            .any(|issue| issue["code"] == "missing")
    );
}

#[test]
fn test_source_attach_rejects_non_source_and_missing_input() {
    let vault = TestVault::new();
    vault.create_note_in_subdir("sources", "ordinary", "---\ntype: note\n---\n\n<!-- markbase:source-attachments:start -->\n<!-- markbase:source-attachments:end -->\n");
    vault.create_note_in_subdir("sources", "source", &source_note());
    let missing = vault.path.join("missing.txt");
    assert!(
        !vault
            .run_cli(&[
                "source",
                "attach",
                "ordinary",
                &missing.to_string_lossy(),
                "--description",
                "Nope"
            ])
            .status
            .success()
    );
    assert!(
        !vault
            .run_cli(&[
                "source",
                "attach",
                "source",
                &missing.to_string_lossy(),
                "--description",
                "Nope"
            ])
            .status
            .success()
    );
    assert!(!vault.path.join("sources/attachments/source").exists());
}

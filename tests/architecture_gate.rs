use std::fs;
use std::path::Path;

fn collect_rs_files(dir: &Path, out: &mut Vec<std::path::PathBuf>) {
    if let Ok(entries) = fs::read_dir(dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                collect_rs_files(&path, out);
            } else if path.extension().and_then(|s| s.to_str()) == Some("rs") {
                out.push(path);
            }
        }
    }
}

#[test]
fn all_src_files_enforce_atomic_200_loc_ceiling() {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let src_dir = manifest_dir.join("src");
    let mut files = Vec::new();
    collect_rs_files(&src_dir, &mut files);
    files.sort();

    assert!(
        files.len() >= 60,
        "Expected atomic modular architecture (>= 60 .rs files in src/), found {}",
        files.len()
    );

    let mut violations = Vec::new();
    for path in &files {
        let content = fs::read_to_string(path).expect("Failed to read source file");
        let line_count = content.lines().count();
        if line_count > 200 {
            let rel = path.strip_prefix(manifest_dir).unwrap_or(path);
            violations.push(format!(
                "{} ({} lines > 200 max)",
                rel.display(),
                line_count
            ));
        }
    }

    assert!(
        violations.is_empty(),
        "Atomic Clean Architecture violation — files in src/ exceeded 200 LOC:\n{}",
        violations.join("\n")
    );
}

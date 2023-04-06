use std::{
    error::Error,
    fs,
    path::{Path, PathBuf},
    process::Command,
};

use walkdir::WalkDir;

pub async fn build_docs(crate_name: &str) -> Result<(), Box<dyn Error>> {
    fs::create_dir_all("./embedding-docs")?;
    fs::create_dir_all("./embedding-crates")?;

    let path_to_embedding_docs = format!("./embedding-docs/{}", crate_name);
    let embedding_folder_path = Path::new(&path_to_embedding_docs);

    if embedding_folder_path.is_dir() {
        return Ok(());
    }

    let cargo_new_result = Command::new("cargo")
        .arg("new")
        .arg(format!("./embedding-crates/{}-docs", crate_name))
        .output()?;
    if !cargo_new_result.status.success() {
        return Err(format!(
            "Failed to create new cargo project: {}",
            String::from_utf8_lossy(&cargo_new_result.stderr)
        )
        .into());
    }

    let cargo_add_result = Command::new("cargo")
        .current_dir(format!("./embedding-crates/{}-docs", crate_name))
        .arg("add")
        .arg(crate_name)
        .output()?;
    if !cargo_add_result.status.success() {
        return Err(format!(
            "Failed to add crate to cargo project: {}",
            String::from_utf8_lossy(&cargo_add_result.stderr)
        )
        .into());
    }

    let cargo_doc_result = Command::new("cargo")
        .current_dir(format!("./embedding-crates/{}-docs", crate_name))
        .arg("doc")
        .arg("--no-deps")
        .arg(format!("--package={}", crate_name))
        .output()?;
    if !cargo_doc_result.status.success() {
        return Err(format!(
            "Failed to generate docs: {}",
            String::from_utf8_lossy(&cargo_doc_result.stderr)
        )
        .into());
    }

    let path_to_docs = format!("./embedding-crates/{}-docs/target/doc", crate_name);
    let folder_path = Path::new(&path_to_docs);
    let folder_path_exists = folder_path.is_dir();
    if !folder_path_exists {
        return Err(format!(
            "Failed to find generated docs at path: {}",
            folder_path.display()
        )
        .into());
    }

    let move_folder_result = fs::rename(&path_to_docs, format!("./embedding-docs/{}", crate_name));
    if move_folder_result.is_err() {
        return Err(format!(
            "Failed to move docs to embedding-docs folder: {}",
            move_folder_result.err().unwrap()
        )
        .into());
    }

    let remove_cargo_project_result =
        fs::remove_dir_all(format!("./embedding-crates/{}-docs", crate_name));
    if remove_cargo_project_result.is_err() {
        return Err(format!(
            "Failed to remove cargo project: {}",
            remove_cargo_project_result.err().unwrap()
        )
        .into());
    }

    let docs_root_dir = PathBuf::from(format!("./embedding-docs/{}/", crate_name));

    for entry in WalkDir::new(&docs_root_dir) {
        let entry = entry.unwrap();
        let path = entry.path();

        if !path.is_dir() && path.extension().unwrap_or_default() != "html" {
            fs::remove_file(path).expect("Failed to remove file");
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[actix_web::test]
    async fn test_build_docs() {
        assert!(build_docs("ratchet_core").await.is_ok());
        assert_eq!(Path::new("./embedding-docs/ratchet_core").is_dir(), true);
        assert_eq!(
            Path::new("./embedding-crates/ratchet_core-docs").is_dir(),
            false
        );
        let html_files_only = WalkDir::new("./embedding-docs/ratchet_core")
            .into_iter()
            .filter_map(|entry| entry.ok())
            .all(|entry| {
                entry.path().is_dir() || entry.path().extension().map_or(false, |ext| ext == "html")
            });

        assert_eq!(html_files_only, true);
    }
}

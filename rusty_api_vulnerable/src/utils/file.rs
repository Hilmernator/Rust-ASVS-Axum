use std::fs;
use std::io::{self, Write};
use std::path::PathBuf;

pub fn read_drop(alias: &str, path: &str) -> io::Result<String> {
    let full_path = PathBuf::from("uploads")
        .join(alias)
        .join(path);
    let contents = fs::read_to_string(full_path)?;

    Ok(contents)
}

pub fn upload_drop(alias: &str, path: &str, file_bytes: &[u8]) -> io::Result<String> {
    let file_path = PathBuf::from("uploads")
        .join(alias)
        .join(path);

    if let Some(parent) = file_path.parent() {
        fs::create_dir_all(parent)?;
    }


    let mut file = fs::File::create(file_path)?;
    file.write_all(file_bytes)?;

    Ok("file uploaded".to_string())
}

pub fn list_drop(alias: &str, path: &str) -> io::Result<String> {
    let full_path = PathBuf::from("uploads")
        .join(alias)
        .join(path);
    let mut results = vec![];

    for entry in fs::read_dir(full_path)? {
        let entry = entry?;
        let filename = entry.file_name();
        results.push(filename.to_string_lossy().to_string());

    }
    Ok(results.join("\n"))
    
}
pub fn root_list_drop(alias: &str) -> io::Result<String> {
    let path = PathBuf::from("uploads")
        .join(alias);

    let mut results = vec![];

    for entry in fs::read_dir(path)? {
        let entry = entry?;
        let filename = entry.file_name();
        results.push(filename.to_string_lossy().to_string());
    }
    Ok(results.join("\n"))
}

pub fn export_drop(alias: &str, path: &str) -> io::Result<Vec<u8>> {
    let full_path = PathBuf::from("uploads")
        .join(alias)
        .join(path);

    let content = fs::read(full_path)?;

    Ok(content)

}
pub fn create_directory(alias: &str, path: &str) -> io::Result<String> {
    let full_path = PathBuf::from("uploads")
        .join(alias)
        .join(path);
    fs::create_dir_all(&full_path)?;

    Ok("Directory created".to_string())
}

pub fn delete_directory(alias: &str, path: &str) -> io::Result<String> {
    let full_path = PathBuf::from("uploads")
        .join(alias)
        .join(path);
    fs::remove_dir_all(&full_path)?;
    Ok(format!("Directory deleted {}", full_path.display()))
}
use std::fs;
use std::io::{self, Write};
use std::path::PathBuf;
use regex::Regex;

use std::path::Path;

pub fn secure_join_path(base: &Path, alias: &str , sub_path: &str) -> Option<PathBuf> {
    println!("[secure_join_path] ▶️ alias='{}', sub_path='{}'", alias, sub_path);

    let valid_input = Regex::new(r"^[a-zA-Z0-9_\-/\.]+$").unwrap();

    let alias_ok = valid_input.is_match(alias);
    let sub_path_ok = sub_path.is_empty() || valid_input.is_match(sub_path);
    println!("[secure_join_path] ✅ Regex match: alias_ok={}, sub_path_ok={}", alias_ok, sub_path_ok);

    if !alias_ok || !sub_path_ok {
        println!("⛔ Regex validation failed");
        return None;
    }

    let forbidden = ["..", "%2e", "%2f", "%5c"];
    for f in forbidden {
        if alias.contains(f) {
            println!("⛔ alias contains forbidden pattern: '{}'", f);
            return None;
        }
        if sub_path.contains(f) {
            println!("⛔ sub_path contains forbidden pattern: '{}'", f);
            return None;
        }
    }

    let full_path = base.join(alias).join(sub_path);
    println!("[secure_join_path] 🛠 Joined path: {:?}", full_path);

    // För canonicalization: bara kontrollera att den FÖRÄLDERN ligger innanför base
    let parent = full_path.parent()?;
    let canon_parent = parent.canonicalize().ok()?;
    let base_canon = base.canonicalize().ok()?;
    if canon_parent.starts_with(&base_canon) {
        println!("✅ Parent path is secure");
        Some(full_path)
    } else {
        println!("⛔ Canonicalized parent escapes base dir");
        None
    }
}
 

pub fn read_drop(alias: &str, path: &str) -> io::Result<String> {
    let full_path = secure_join_path(Path::new("uploads"), alias, path)
        .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidInput, "Invalid Path"))?;
    let contents = fs::read_to_string(full_path)?;

    Ok(contents)
}

pub fn upload_drop(alias: &str, path: &str, file_bytes: &[u8]) -> io::Result<String> {
    println!("[upload_drop] alias: '{}', path: '{}'", alias, path);

    let full_path = secure_join_path(Path::new("uploads"), alias, path)
        .ok_or_else(|| {
            println!("❌ secure_join_path failed!");
            io::Error::new(io::ErrorKind::InvalidInput, "Invalid Path")
        })?;

    println!("[upload_drop] full path after canonicalization: {:?}", full_path);

    if let Some(parent) = full_path.parent() {
        println!("Creating parent dirs: {:?}", parent);
        fs::create_dir_all(parent)?;
    }

    let mut file = fs::File::create(&full_path)?;
    file.write_all(file_bytes)?;

    println!("[upload_drop] ✅ Write succeeded");
    Ok("file uploaded".to_string())
}



pub fn list_drop(alias: &str, path: &str) -> io::Result<String> {
    let full_path = secure_join_path(Path::new("uploads"), alias, path)
        .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidInput, "Invalid Path"))?;
    let mut results = vec![];

    for entry in fs::read_dir(full_path)? {
        let entry = entry?;
        let filename = entry.file_name();
        results.push(filename.to_string_lossy().to_string());

    }
    Ok(results.join("\n"))
    
}
pub fn root_list_drop(alias: &str) -> io::Result<String> {
    let full_path = secure_join_path(Path::new("uploads"), alias, "")
        .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidInput, "Invalid Path"))?;

    let mut results = vec![];

    for entry in fs::read_dir(full_path)? {
        let entry = entry?;
        let filename = entry.file_name();
        results.push(filename.to_string_lossy().to_string());
    }
    Ok(results.join("\n"))
}

pub fn export_drop(alias: &str, path: &str) -> io::Result<Vec<u8>> {
    let full_path = secure_join_path(Path::new("uploads"), alias, path)
        .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidInput, "Invalid Path"))?;

    let content = fs::read(full_path)?;

    Ok(content)

}
pub fn create_directory(alias: &str, path: &str) -> io::Result<String> {
    let full_path = secure_join_path(Path::new("uploads"), alias, path)
        .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidInput, "Invalid Path"))?;

    fs::create_dir_all(&full_path)?;

    Ok("Directory created".to_string())
}

pub fn delete_directory(alias: &str, path: &str) -> io::Result<String> {
    let full_path = secure_join_path(Path::new("uploads"), alias, path)
        .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidInput, "Invalid Path"))?;

    fs::remove_dir_all(&full_path)?;
    Ok(format!("Directory deleted {}", full_path.display()))
}
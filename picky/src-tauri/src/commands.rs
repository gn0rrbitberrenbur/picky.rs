use std::collections::HashMap;
use std::fs;
use std::path::Path;

use base64::{engine::general_purpose, Engine as _};

use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct ImagePair {
    pub preview: String,
    pub raw: String,
}

#[tauri::command]
pub fn contains_images(path: String) -> Result<bool, String> {
    let dir = Path::new(&path);

    if !dir.exists() || !dir.is_dir() {
        return Err("Invalid folder".into());
    }

    let raw_extensions = [
        "cr2", "cr3", "nef", "arw", "dng", "orf",
        "rw2", "raf", "sr2", "pef",
    ];

    let mut previews: HashMap<String, ()> = HashMap::new();
    let mut raws: HashMap<String, ()> = HashMap::new();

    for entry in fs::read_dir(dir).map_err(|e| e.to_string())? {
        let entry = entry.map_err(|e| e.to_string())?;
        let path = entry.path();

        if !path.is_file() {
            continue;
        }

        let ext = path
            .extension()
            .and_then(|e| e.to_str())
            .map(|e| e.to_lowercase());

        let stem = path
            .file_stem()
            .and_then(|s| s.to_str())
            .map(|s| s.to_string());

        match (ext, stem) {
            (Some(ext), Some(stem)) if ["jpg", "jpeg", "png"].contains(&ext.as_str()) => {
                previews.insert(stem, ());
            }
            (Some(ext), Some(stem)) if raw_extensions.contains(&ext.as_str()) => {
                raws.insert(stem, ());
            }
            _ => {}
        }
    }

    if previews.is_empty() {
        return Ok(false);
    }

    for stem in previews.keys() {
        if !raws.contains_key(stem) {
            return Ok(false);
        }
    }

    Ok(true)
}


#[tauri::command]
pub fn map_images(path: String) -> Result<Vec<ImagePair>, String> {
    let dir = Path::new(&path);

    if !dir.exists() || !dir.is_dir() {
        return Err("Invalid folder".into());
    }

    let mut previews: HashMap<String, String> = HashMap::new();
    let mut raws: HashMap<String, String> = HashMap::new();

    let raw_extensions = [
        "cr2", "cr3", "nef", "arw", "dng", "orf",
        "rw2", "raf", "sr2", "pef",
    ];

    for entry in fs::read_dir(dir).map_err(|e| e.to_string())? {
        let entry = entry.map_err(|e| e.to_string())?;
        let path = entry.path();

        if !path.is_file() {
            continue;
        }

        let ext = path
            .extension()
            .and_then(|e| e.to_str())
            .map(|e| e.to_lowercase());

        let stem = path
            .file_stem()
            .and_then(|s| s.to_str())
            .map(|s| s.to_string());

        match (ext, stem) {
            (Some(ext), Some(stem)) if ["jpg", "jpeg", "png"].contains(&ext.as_str()) => {
                previews.insert(stem, path.to_string_lossy().to_string());
            }
            (Some(ext), Some(stem)) if raw_extensions.contains(&ext.as_str()) => {
                raws.insert(stem, path.to_string_lossy().to_string());
            }
            _ => {}
        }
    }

    let mut result = Vec::new();

    for (stem, preview) in previews {
        if let Some(raw) = raws.get(&stem) {
            result.push(ImagePair {
                preview,
                raw: raw.clone(),
            });
        }
    }

    Ok(result)
}

#[tauri::command]
pub fn image_to_base64_data_url(arg: String) -> Result<String, String> {

    let bytes = fs::read(&arg).map_err(|e| e.to_string())?;
    let mime = if arg.ends_with(".png") { "image/png" } else { "image/jpeg" };
    let base64 = general_purpose::STANDARD.encode(bytes);

    Ok(format!("data:{};base64,{}", mime, base64))
}

#[tauri::command]
pub fn copy_image_to_wanna_edit(image_path: &str) -> Result<(), String> {
    let image_path = Path::new(image_path);

    let file_name = image_path
        .file_name()
        .ok_or_else(|| "Invalid filepath".to_string())?;

    let target_dir = image_path
        .parent()
        .ok_or_else(|| "No parent-directory found".to_string())?
        .join("wanna_edit");

    fs::create_dir_all(&target_dir)
        .map_err(|e| e.to_string())?;

    let target_path = target_dir.join(file_name);

    fs::copy(image_path, &target_path)
        .map_err(|e| e.to_string())?;

    Ok(())
}
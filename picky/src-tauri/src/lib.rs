mod commands;

use commands::{contains_images, 
               map_images, 
               image_to_base64_data_url, 
               copy_image_to_wanna_edit
            };

use std::fs;
use tauri::Manager;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![contains_images, 
                                                 map_images,
                                                 image_to_base64_data_url, 
                                                 copy_image_to_wanna_edit
                                                 ]
                                                )
        .setup(|app| {
            // Delete app cache on startup
            // to fix whitescreen issue
            //
            if cfg!(target_os = "linux") {
                let cache_dir = app.path().cache_dir()?;
                let package_info = app.package_info();
                let app_name = package_info.name.as_str();
                let app_cache = cache_dir.join(app_name);
                if app_cache.exists() {
                    let _ = fs::remove_dir_all(&app_cache);
                }
            }
        
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
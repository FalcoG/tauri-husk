#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]


use std::env;

#[cfg(target_os = "windows")]
extern crate winreg;
#[cfg(target_os = "windows")]
use winreg::enums::*;
#[cfg(target_os = "windows")]
use winreg::RegKey;
#[cfg(target_os = "windows")]
use std::error::Error;
use tauri::{ PhysicalPosition, PhysicalSize, Position, Size };

#[cfg(target_os = "windows")]
async fn registry_read() -> Result<String, Box<dyn Error>> {
    let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
    let hkcu = RegKey::predef(HKEY_CURRENT_USER);
    let cur_ver = hklm.open_subkey("SOFTWARE\\Microsoft\\Windows\\CurrentVersion")?;
    let pf: String = cur_ver.get_value("ProgramFilesDir")?;
    let dp: String = cur_ver.get_value("DevicePath")?;

    let taskband = hkcu.open_subkey("Software\\Microsoft\\Windows\\CurrentVersion\\Explorer\\Taskband")?;
    // let taskband_favorites = taskband.get_value("Favorites")?.unwrap();
    // let taskband_favorites: &[u8] = taskband.get_value("Favorites")?;
    // let taskband_favorites: winreg::enums::RegType::REG_BINARY = taskband.get_value("Favorites")?;
    let taskbar_version: u32 = taskband.get_value("FavoritesVersion")?;
    //
    println!("Favorites version {}", taskbar_version);
    // println!("Favorites {}", String::from(taskband_favorites));
    // println!("Favorites {}", String::from_utf8(taskband_favorites).unwrap());
    println!("ProgramFiles = {}\nDevicePath = {}", pf, dp);

    Ok(format!("hello, world!"))
}

#[cfg(target_os = "windows")]
#[tauri::command]
async fn registry() -> String {
    let registry_result = registry_read().await;

    let registry_return = match registry_result {
        Ok(file) => file,
        Err(error) => panic!("Problem opening the file: {:?}", error.to_string()),
    };

    return registry_return
}

#[cfg(not(target_os = "windows"))]
#[tauri::command]
async fn registry() -> String {
    format!("UNSUPPORTED")
}

#[tauri::command]
fn example() -> &'static str {
    env::consts::OS
    // format!("OS: {}, test", env::consts::OS)
}

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

fn main() {
    tauri::Builder::default()
        .setup(|app| {
            let handle = app.handle();
            std::thread::spawn(move || {
                let window = tauri::WindowBuilder::new(
                    &handle, "Dock", tauri::WindowUrl::App("/dock".into())
                )
                    .inner_size(800 as f64, 100 as f64)
                    .always_on_top(true)
                    .decorations(false)
                    .transparent(true)
                    .build()
                    .unwrap();

                let monitor = window.current_monitor();
                let size = match monitor {
                    Ok(monitor) => {
                        let monitor2 = monitor.unwrap();
                        let monitor_size = monitor2.size();
                        let dock_height = 200;

                        window.set_size(
                            Size::Physical(PhysicalSize {
                                width: monitor_size.width / 2 as u32,
                                height: dock_height as u32
                            })
                        ).unwrap();

                        window.set_position(
                            Position::Physical(PhysicalPosition {
                                x: monitor_size.width as i32 / 4,
                                y: (monitor_size.height - dock_height) as i32
                            })
                        ).unwrap();
                        format!("monitor? {}", monitor_size.width)
                    }
                    Err(error) => {
                        format!("Error monitor retrieval {}", error)
                    }
                };

                // let newSize = window.inner_size().await;
                // window.set_size(newSize);

                // let custom_size = tauri::Size().from(100 as f64, 800 as f64);
                // window.set_size(size, 100 as f64)
                // window.inner_size()
                // window.inner_size(size, 100 as f64);
                println!("SIZE! {}", size)
            });
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![greet, example, registry])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}


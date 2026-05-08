mod commands;
mod core;
mod database;
mod shared;
mod tracing_init;

use crate::commands::commands::get_grouped_data;
use crate::commands::autostart::{get_autostart_status, set_autostart};
use crate::core::collector::run_collector;
use crate::tracing_init::init_tracing;
use std::error::Error;
use tauri::{
    menu::{Menu, MenuItem},
    tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent},
    Manager, WindowEvent,
};

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let _guard = init_tracing();

    tauri::Builder::default()
        .plugin(tauri_plugin_autostart::Builder::new().build())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            greet, get_grouped_data, set_autostart, get_autostart_status
        ])
        .setup(|app| {

            use tauri_plugin_autostart::MacosLauncher;
            match app.handle().plugin(tauri_plugin_autostart::init(
                MacosLauncher::LaunchAgent,
                    Some(vec![]),
                )) {
                Ok(_) => tracing::info!("Autostart plugin initialized successfully"),
                Err(e) => tracing::error!("Failed to initialize autostart plugin: {}", e),
            }

            // Start the collector in a background task
            let handle = app.handle().clone();
            tauri::async_runtime::spawn(async move {
                run_collector(handle).await;
            });

            // Build the system tray menu and event handlers
            build_system_tray(app)?;

            Ok(())
        })
        .on_window_event(|window, event| match event {
            WindowEvent::CloseRequested { api, .. } => {
                //? TODO dialog
                api.prevent_close();
                let _ = window.hide();
            }
            WindowEvent::Resized(size) => {
                if size.width == 0 && size.height == 0 {
                    let _ = window.hide();
                }
            }
            _ => {}
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

fn build_system_tray(app: &mut tauri::App) -> Result<(), Box<dyn Error + 'static>> {
    let show_item = MenuItem::with_id(app, "show", "Show", true, None::<&str>)?;
    let quit_item = MenuItem::with_id(app, "quit", "Quit", true, None::<&str>)?;
    let menu = Menu::with_items(app, &[&show_item, &quit_item])?;

    TrayIconBuilder::new()
        .icon(app.default_window_icon().unwrap().clone())
        .menu(&menu)
        .tooltip("Activity Tracker")
        .on_menu_event(|app, event| match event.id.as_ref() {
            "show" => {
                if let Some(window) = app.get_webview_window("main") {
                    let _ = window.show();
                    let _ = window.set_focus();
                }
            }
            "quit" => {
                app.exit(0);
            }
            _ => {}
        })
        .on_tray_icon_event(|tray, event| {
            if let TrayIconEvent::Click {
                button: MouseButton::Left,
                button_state: MouseButtonState::Up,
                ..
            } = event
            {
                let app = tray.app_handle();
                if let Some(window) = app.get_webview_window("main") {
                    let _ = window.show();
                    let _ = window.set_focus();
                }
            }
        })
        .build(app)?;
    Ok(())
}

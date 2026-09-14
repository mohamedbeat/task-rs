pub mod gpu;
pub mod icon;
pub mod usage;

mod processes;
use std::{collections::HashMap, sync::Mutex, time::Duration};
use sysinfo::{self, Disks};
use tauri::{
    menu::{Menu, MenuItem},
    tray::{TrayIconBuilder, TrayIconEvent},
    Emitter, Manager, WindowEvent,
};
use tauri_plugin_autostart::MacosLauncher;

// Background loop — refreshes and pushes updates via events
pub fn start_monitoring(app: tauri::AppHandle) {
    std::thread::spawn(move || {
        let mut sys = sysinfo::System::new_all();

        loop {
            {
                sys.refresh_cpu_usage();
                sys.refresh_memory();
                let state = app.state::<processes::SysState>();
                let mut icon_cache = state.icon_cache.lock().unwrap();
                sys.refresh_processes(sysinfo::ProcessesToUpdate::All, true);
                let list = processes::get_process_list(&sys, &mut icon_cache);
                // sys (the MutexGuard) drops here at end of block, before emit
                let _ = app.emit("processes-update", &list);
            }
            std::thread::sleep(Duration::from_secs(1));
        }
    });
}

pub fn start_monitoring_stats(app: tauri::AppHandle) {
    std::thread::spawn(move || {
        let mut sys = sysinfo::System::new_all();

        let mut disks = Disks::new_with_refreshed_list();
        loop {
            {
                sys.refresh_cpu_usage();
                sys.refresh_memory();
                disks.refresh(true);
                let stats = usage::get_system_stats(&sys, &disks);
                // sys (the MutexGuard) drops here at end of block, before emit
                let _ = app.emit("stats", &stats);
            }
            std::thread::sleep(Duration::from_secs(3));
        }
    });
}
// Wrap System in a Mutex so it can be shared safely between
// the background thread and any commands called from the frontend
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_autostart::init(
            MacosLauncher::LaunchAgent,
            None,
        ))
        .plugin(tauri_plugin_global_shortcut::Builder::new().build())
        .plugin(tauri_plugin_clipboard_manager::init())
        .plugin(tauri_plugin_opener::init())
        .manage(processes::SysState {
            sys: Mutex::new(sysinfo::System::new_all()),
            icon_cache: Mutex::new(HashMap::new()),
        })
        .setup(|app| {
            // Build tray menu
            let show_item = MenuItem::with_id(app, "show", "Show", true, None::<&str>)?;
            let quit_item = MenuItem::with_id(app, "quit", "Quit", true, None::<&str>)?;
            let menu = Menu::with_items(app, &[&show_item, &quit_item])?;

            let _tray = TrayIconBuilder::new()
                .icon(app.default_window_icon().unwrap().clone())
                .menu(&menu)
                .show_menu_on_left_click(false)
                .on_menu_event(|app, event| match event.id.as_ref() {
                    "show" => {
                        if let Some(window) = app.get_webview_window("main") {
                            let _ = window.show();
                            let _ = window.unminimize();
                            let _ = window.set_focus();
                        }
                    }
                    "quit" => {
                        app.exit(0);
                    }
                    _ => {}
                })
                .on_tray_icon_event(|tray, event| {
                    // left-click on the icon itself (not the menu) also shows the window
                    if let TrayIconEvent::Click { button, .. } = event {
                        if button == tauri::tray::MouseButton::Left {
                            let app = tray.app_handle();
                            if let Some(window) = app.get_webview_window("main") {
                                let _ = window.show();
                                let _ = window.unminimize();
                                let _ = window.set_focus();
                            }
                        }
                    }
                })
                .build(app)?;

            let handle = app.handle().clone();
            start_monitoring(handle.clone());
            start_monitoring_stats(handle);
            Ok(())
        })
        // Intercept the window close button — hide instead of actually closing
        .on_window_event(|window, event| {
            if let WindowEvent::CloseRequested { api, .. } = event {
                window.hide().unwrap();
                api.prevent_close();
            }
        })
        .invoke_handler(tauri::generate_handler![
            processes::get_processes,
            processes::kill_process
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

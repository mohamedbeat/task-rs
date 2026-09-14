pub mod gpu;
pub mod icon;
pub mod usage;

mod processes;
use std::{collections::HashMap, sync::Mutex, time::Duration};
use sysinfo::{self, Disks};
use tauri::{Emitter, Manager};

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
        .plugin(tauri_plugin_global_shortcut::Builder::new().build())
        .plugin(tauri_plugin_clipboard_manager::init())
        .manage(processes::SysState {
            sys: Mutex::new(sysinfo::System::new_all()),
            icon_cache: Mutex::new(HashMap::new()),
        })
        .setup(|app| {
            let handle = app.handle().clone();
            start_monitoring(handle.clone());
            start_monitoring_stats(handle);
            Ok(())
        })
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            processes::get_processes,
            processes::kill_process
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

use serde::{Deserialize, Serialize};
use std::{collections::HashMap, sync::Mutex, time::Duration};

use crate::icon;

pub struct SysState {
    pub sys: Mutex<sysinfo::System>,
    pub icon_cache: Mutex<HashMap<String, Option<String>>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProcessInfo {
    pub pid: u32,
    pub name: String,
    pub cpu_usage: f32,
    pub cpu_usage_h: String,
    pub memory: u64,
    pub memory_h: String,
    pub disk_usage: u64,
    pub disk_usage_h: String,
    pub exe_path: Option<String>,
    pub icon: Option<String>,
}

pub fn get_process_list(
    sys: &sysinfo::System,
    icon_cache: &mut HashMap<String, Option<String>>,
) -> Vec<ProcessInfo> {
    sys.processes()
        .iter()
        .map(|(id, p)| {
            let cpu = p.cpu_usage();
            let mem = p.memory();
            let exe_path = p.exe().map(|p| p.display().to_string());

            let icon = exe_path
                .as_ref()
                .and_then(|path| get_icon_cached(icon_cache, path));

            let p = ProcessInfo {
                pid: id.as_u32(),
                name: p.name().to_string_lossy().to_string(),
                cpu_usage: p.cpu_usage(),
                cpu_usage_h: humanize_cpu(p.cpu_usage()),
                memory: p.memory(),
                memory_h: humanize_bytes(p.memory()),
                disk_usage: p.disk_usage().total_read_bytes,
                disk_usage_h: humanize_bytes(p.disk_usage().total_read_bytes),
                exe_path: p.exe().map(|p| p.display().to_string()),
                icon: icon,
            };
            p
        })
        .collect()
}

pub fn get_icon_cached(
    cache: &mut HashMap<String, Option<String>>,
    exe_path: &str,
) -> Option<String> {
    if let Some(cached) = cache.get(exe_path) {
        return cached.clone();
    }
    let icon = icon::get_exe_icon_base64(exe_path);
    cache.insert(exe_path.to_string(), icon.clone());
    icon
}

// On-demand command — frontend can call this directly too,
// e.g. for an immediate refresh on button click, not just the interval
#[tauri::command]
pub fn get_processes(state: tauri::State<SysState>) -> Vec<ProcessInfo> {
    let mut sys = state.sys.lock().unwrap();
    let mut icon_cache = state.icon_cache.lock().unwrap();
    sys.refresh_processes(sysinfo::ProcessesToUpdate::All, true);
    get_process_list(&sys, &mut icon_cache)
}

#[tauri::command]
pub fn kill_process(state: tauri::State<SysState>, pid: u32) -> bool {
    let sys = state.sys.lock().unwrap();
    match sys.process(sysinfo::Pid::from_u32(pid)) {
        Some(process) => process.kill(),
        None => false,
    }
}

pub fn humanize_bytes(bytes: u64) -> String {
    const UNITS: [&str; 5] = ["B", "KB", "MB", "GB", "TB"];
    let mut size = bytes as f64;
    let mut unit_index = 0;

    while size >= 1024.0 && unit_index < UNITS.len() - 1 {
        size /= 1024.0;
        unit_index += 1;
    }

    if unit_index == 0 {
        format!("{} {}", size as u64, UNITS[unit_index])
    } else {
        format!("{:.1} {}", size, UNITS[unit_index])
    }
}
pub fn humanize_cpu(cpu_usage: f32) -> String {
    format!("{:.1}%", cpu_usage)
}

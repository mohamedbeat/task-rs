use serde::{Deserialize, Serialize};
use sysinfo::{Disks, System};

use crate::gpu::{dxgi, nvml_gpu, pdh};

#[derive(Serialize, Clone)]
pub struct SystemStats {
    // Memory
    pub memory_used_gb: f64,
    pub memory_total_gb: f64,
    pub memory_percent: f32,

    // CPU
    pub cpu_percent: f32,
    pub cpu_per_core: Vec<f32>,
    pub cpu_name: String,

    // Disk
    pub disks: Vec<DiskInfo>,

    // GPU
    pub gpu: Vec<GpuInfo>,
}

#[derive(Serialize, Clone)]
pub struct GpuInfo {
    name: String,
    usage_percent: f32,
    memory_used_gb: f64,
    memory_total_gb: f64,
}

#[derive(Serialize, Clone)]
pub struct DiskInfo {
    pub name: String,
    pub used_gb: f64,
    pub total_gb: f64,
    pub percent: f32,
}

fn bytes_to_gb(bytes: u64) -> f64 {
    bytes as f64 / (1024.0 * 1024.0 * 1024.0)
}

pub fn get_system_stats(sys: &System, disks: &Disks) -> SystemStats {
    let mem_total = sys.total_memory();
    let mem_used = sys.used_memory();

    let disk_list: Vec<DiskInfo> = disks
        .iter()
        .map(|disk| {
            let total = disk.total_space();
            let available = disk.available_space();
            let used = total.saturating_sub(available);
            DiskInfo {
                name: disk.name().to_string_lossy().to_string(),
                used_gb: bytes_to_gb(used),
                total_gb: bytes_to_gb(total),
                percent: if total > 0 {
                    (used as f32 / total as f32) * 100.0
                } else {
                    0.0
                },
            }
        })
        .collect();

    SystemStats {
        memory_used_gb: bytes_to_gb(mem_used),
        memory_total_gb: bytes_to_gb(mem_total),
        memory_percent: if mem_total > 0 {
            (mem_used as f32 / mem_total as f32) * 100.0
        } else {
            0.0
        },
        cpu_name: sys
            .cpus()
            .first()
            .map(|c| c.brand().trim().to_string())
            .unwrap_or_else(|| "Unknown Cpu".to_string()),
        cpu_percent: sys.global_cpu_usage(),
        cpu_per_core: sys.cpus().iter().map(|c| c.cpu_usage()).collect(),
        disks: disk_list,
        gpu: get_gpu_stats(), // stub for now, see below
    }
}

pub fn get_gpu_stats() -> Vec<GpuInfo> {
    let adapters = dxgi::enumerate_adapters();
    let pdh_stats = pdh::query_pdh_gpu_stats(); // ~200ms cost due to double-sample

    // let wmi_stats = wmi_gpu::query_wmi_gpu_stats();

    let mut nvidia_index = 0usize;

    adapters
        .into_iter()
        .map(|adapter| {
            let mut usage = pdh_stats
                .as_ref()
                .and_then(|w| w.usage_by_luid.get(&adapter.luid).copied())
                .unwrap_or(0.0);

            let mut used_bytes = pdh_stats
                .as_ref()
                .and_then(|w| w.mem_used_by_luid.get(&adapter.luid).copied())
                .unwrap_or(0);

            const VENDOR_NVIDIA: u32 = 0x10DE;
            if adapter.vendor_id == VENDOR_NVIDIA {
                if let Some((nv_usage, nv_used, _nv_total)) =
                    nvml_gpu::nvidia_override(nvidia_index)
                {
                    usage = nv_usage;
                    used_bytes = nv_used;
                }
                nvidia_index += 1;
            }

            GpuInfo {
                name: adapter.name,
                usage_percent: usage,
                memory_used_gb: bytes_to_gb(used_bytes),
                memory_total_gb: bytes_to_gb(adapter.total_vram_bytes),
            }
        })
        .collect()
}

use serde::Deserialize;
use std::collections::HashMap;
use wmi::WMIConnection;

#[derive(Deserialize, Debug)]
struct GpuEngineRow {
    #[serde(rename = "Name")]
    name: String,
    #[serde(rename = "UtilizationPercentage")]
    utilization_percentage: u64,
}

#[derive(Deserialize, Debug)]
struct GpuMemoryRow {
    #[serde(rename = "Name")]
    name: String,
    #[serde(rename = "DedicatedUsage")]
    dedicated_usage: u64,
}

fn parse_luid_from_name(name: &str) -> Option<i64> {
    // e.g. "...luid_0x00000000_0x0000AB12_phys_0..."
    let idx = name.find("luid_")?;
    let rest = &name[idx + 5..];
    let mut parts = rest.splitn(3, '_');
    let high = parts.next()?;
    let low = parts.next()?;

    let high = i64::from_str_radix(high.trim_start_matches("0x"), 16).ok()?;
    let low = i64::from_str_radix(low.trim_start_matches("0x"), 16).ok()?;

    Some((high << 32) | (low & 0xFFFFFFFF))
}

pub struct WmiGpuStats {
    pub usage_by_luid: HashMap<i64, f32>,
    pub mem_used_by_luid: HashMap<i64, u64>,
}

pub fn query_wmi_gpu_stats() -> Option<WmiGpuStats> {
    let wmi = WMIConnection::new().ok()?;

    let engines: Vec<GpuEngineRow> = wmi
        .raw_query(
            "SELECT Name, UtilizationPercentage FROM Win32_PerfFormattedData_Counters_GPUEngine",
        )
        .ok()?;

    let mems: Vec<GpuMemoryRow> = wmi
        .raw_query(
            "SELECT Name, DedicatedUsage FROM Win32_PerfFormattedData_Counters_GPUAdapterMemory",
        )
        .ok()?;

    let mut usage_by_luid: HashMap<i64, f32> = HashMap::new();
    for row in engines {
        if let Some(luid) = parse_luid_from_name(&row.name) {
            let entry = usage_by_luid.entry(luid).or_insert(0.0);
            *entry = (*entry + row.utilization_percentage as f32).min(100.0);
        }
    }

    let mut mem_used_by_luid: HashMap<i64, u64> = HashMap::new();
    for row in mems {
        if let Some(luid) = parse_luid_from_name(&row.name) {
            mem_used_by_luid.insert(luid, row.dedicated_usage);
        }
    }

    Some(WmiGpuStats {
        usage_by_luid,
        mem_used_by_luid,
    })
}

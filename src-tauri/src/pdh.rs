// gpu/pdh.rs
use std::collections::HashMap;
use windows::core::PCWSTR;
use windows::Win32::System::Performance::*;

use crate::luid;

fn to_wide(s: &str) -> Vec<u16> {
    s.encode_utf16().chain(std::iter::once(0)).collect()
}

unsafe fn query_wildcard(path: &str) -> Option<Vec<(String, f64)>> {
    let mut query = PDH_HQUERY::default();
    if PdhOpenQueryW(PCWSTR::null(), 0, &mut query) != 0 {
        return None;
    }

    let wide_path = to_wide(path);
    let mut counter = PDH_HCOUNTER::default();
    if PdhAddEnglishCounterW(query, PCWSTR(wide_path.as_ptr()), 0, &mut counter) != 0 {
        let _ = PdhCloseQuery(query);
        return None;
    }

    // First sample primes the counter; utilization needs a second sample to compute a rate.
    let _ = PdhCollectQueryData(query);
    std::thread::sleep(std::time::Duration::from_millis(200));
    if PdhCollectQueryData(query) != 0 {
        let _ = PdhCloseQuery(query);
        return None;
    }

    let mut buffer_size: u32 = 0;
    let mut item_count: u32 = 0;

    // First call: just get the required buffer size (expected to "fail" with PDH_MORE_DATA)
    let _ = PdhGetFormattedCounterArrayW(
        counter,
        PDH_FMT_DOUBLE,
        &mut buffer_size,
        &mut item_count,
        None,
    );

    if buffer_size == 0 {
        let _ = PdhCloseQuery(query);
        return Some(Vec::new());
    }

    let mut buffer: Vec<u8> = vec![0u8; buffer_size as usize];
    let result = PdhGetFormattedCounterArrayW(
        counter,
        PDH_FMT_DOUBLE,
        &mut buffer_size,
        &mut item_count,
        Some(buffer.as_mut_ptr() as *mut PDH_FMT_COUNTERVALUE_ITEM_W),
    );

    let _ = PdhCloseQuery(query);

    if result != 0 {
        return None;
    }

    let items = std::slice::from_raw_parts(
        buffer.as_ptr() as *const PDH_FMT_COUNTERVALUE_ITEM_W,
        item_count as usize,
    );

    let mut out = Vec::with_capacity(item_count as usize);
    for item in items {
        let name = item.szName.to_string().unwrap_or_default();
        let value = item.FmtValue.Anonymous.doubleValue;
        out.push((name, value));
    }

    Some(out)
}

pub struct PdhGpuStats {
    pub usage_by_luid: HashMap<i64, f32>,
    pub mem_used_by_luid: HashMap<i64, u64>,
}

pub fn query_pdh_gpu_stats() -> Option<PdhGpuStats> {
    let util_rows = unsafe { query_wildcard(r"\GPU Engine(*)\Utilization Percentage")? };
    let mem_rows = unsafe { query_wildcard(r"\GPU Adapter Memory(*)\Dedicated Usage")? };

    let mut usage_by_luid: HashMap<i64, f32> = HashMap::new();
    for (name, value) in util_rows {
        if let Some(luid) = luid::parse_luid_from_name(&name) {
            let entry = usage_by_luid.entry(luid).or_insert(0.0);
            *entry = (*entry + value as f32).min(100.0);
        }
    }

    let mut mem_used_by_luid: HashMap<i64, u64> = HashMap::new();
    for (name, value) in mem_rows {
        if let Some(luid) = luid::parse_luid_from_name(&name) {
            mem_used_by_luid.insert(luid, value as u64);
        }
    }

    Some(PdhGpuStats {
        usage_by_luid,
        mem_used_by_luid,
    })
}

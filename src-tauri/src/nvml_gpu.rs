use nvml_wrapper::Nvml;

pub fn nvidia_override(index: usize) -> Option<(f32, u64, u64)> {
    let nvml = Nvml::init().ok()?;
    let device = nvml.device_by_index(index as u32).ok()?;
    let util = device.utilization_rates().ok()?;
    let mem = device.memory_info().ok()?;
    Some((util.gpu as f32, mem.used, mem.total))
}

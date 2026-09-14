use windows::Win32::Graphics::Dxgi::*;

pub struct DxgiAdapter {
    pub luid: i64,
    pub name: String,
    pub vendor_id: u32,
    pub total_vram_bytes: u64,
}

pub fn enumerate_adapters() -> Vec<DxgiAdapter> {
    let mut result = Vec::new();

    unsafe {
        let factory: IDXGIFactory1 = match CreateDXGIFactory1() {
            Ok(f) => f,
            Err(_) => return result,
        };

        let mut i = 0;
        loop {
            let adapter: IDXGIAdapter1 = match factory.EnumAdapters1(i) {
                Ok(a) => a,
                Err(_) => break, // DXGI_ERROR_NOT_FOUND
            };
            i += 1;

            let desc = match adapter.GetDesc1() {
                Ok(d) => d,
                Err(_) => continue,
            };

            // Skip Microsoft's software adapter ("Microsoft Basic Render Driver")
            const DXGI_ADAPTER_FLAG_SOFTWARE: u32 = 2;
            if desc.Flags & DXGI_ADAPTER_FLAG_SOFTWARE != 0 {
                continue;
            }

            let name = String::from_utf16_lossy(
                &desc.Description[..desc
                    .Description
                    .iter()
                    .position(|&c| c == 0)
                    .unwrap_or(desc.Description.len())],
            );

            // let luid = ((desc.AdapterLuid.HighPart as i64) << 32)
            //     | (desc.AdapterLuid.LowPart as i64 & 0xFFFFFFFF);
            let luid: i64 = (((desc.AdapterLuid.HighPart as u32 as u64) << 32)
                | (desc.AdapterLuid.LowPart as u64)) as i64;

            result.push(DxgiAdapter {
                luid,
                name,
                vendor_id: desc.VendorId,
                total_vram_bytes: desc.DedicatedVideoMemory as u64,
            });
        }
    }

    result
}

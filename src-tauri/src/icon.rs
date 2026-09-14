use base64::{engine::general_purpose::STANDARD, Engine};
use image::{ImageBuffer, Rgba};
use windows::core::PCWSTR;
use windows::Win32::Graphics::Gdi::{
    CreateCompatibleDC, DeleteDC, DeleteObject, GetDIBits, GetObjectW, SelectObject, BITMAP,
    BITMAPINFO, BITMAPINFOHEADER, BI_RGB, DIB_RGB_COLORS, HBITMAP,
};
use windows::Win32::UI::Shell::{SHGetFileInfoW, SHFILEINFOW, SHGFI_ICON, SHGFI_LARGEICON};
use windows::Win32::UI::WindowsAndMessaging::{DestroyIcon, GetIconInfo, HICON, ICONINFO};

fn get_icon_handle(exe_path: &str) -> Option<HICON> {
    let wide_path: Vec<u16> = exe_path.encode_utf16().chain(std::iter::once(0)).collect();

    let mut shfi = SHFILEINFOW::default();

    let result = unsafe {
        SHGetFileInfoW(
            PCWSTR(wide_path.as_ptr()),
            windows::Win32::Storage::FileSystem::FILE_FLAGS_AND_ATTRIBUTES(0),
            Some(&mut shfi),
            std::mem::size_of::<SHFILEINFOW>() as u32,
            SHGFI_ICON | SHGFI_LARGEICON,
        )
    };

    if result == 0 || shfi.hIcon.is_invalid() {
        None
    } else {
        Some(shfi.hIcon)
    }
}

fn icon_to_rgba(hicon: HICON) -> Option<(Vec<u8>, u32, u32)> {
    unsafe {
        let mut icon_info = ICONINFO::default();
        if let Err(e) = GetIconInfo(hicon, &mut icon_info) {
            println!("GetIconInfo failed: {:?}", e);
            return None;
        }
        let hbitmap = icon_info.hbmColor;
        let mut bitmap = BITMAP::default();
        let obj_result = GetObjectW(
            hbitmap.into(),
            std::mem::size_of::<BITMAP>() as i32,
            Some(&mut bitmap as *mut _ as *mut _),
        );

        let width = bitmap.bmWidth as u32;
        let height = bitmap.bmHeight as u32;
        if width == 0 || height == 0 {
            println!("Zero dimensions, bailing out");
            return None;
        }

        let hdc = CreateCompatibleDC(None);
        let old_obj = SelectObject(hdc, hbitmap.into());

        let mut bmi = BITMAPINFO {
            bmiHeader: BITMAPINFOHEADER {
                biSize: std::mem::size_of::<BITMAPINFOHEADER>() as u32,
                biWidth: width as i32,
                biHeight: -(height as i32), // negative = top-down DIB
                biPlanes: 1,
                biBitCount: 32,
                biCompression: BI_RGB.0,
                ..Default::default()
            },
            ..Default::default()
        };

        let mut buffer = vec![0u8; (width * height * 4) as usize];

        let dib_result = GetDIBits(
            hdc,
            hbitmap,
            0,
            height,
            Some(buffer.as_mut_ptr() as *mut _),
            &mut bmi,
            DIB_RGB_COLORS,
        );

        SelectObject(hdc, old_obj);
        let _ = DeleteDC(hdc);
        let _ = DeleteObject(hbitmap.into());
        let _ = DeleteObject(icon_info.hbmMask.into());

        // Windows gives BGRA, need to swap to RGBA
        for chunk in buffer.chunks_exact_mut(4) {
            chunk.swap(0, 2);
        }

        Some((buffer, width, height))
    }
}

fn rgba_to_png_base64(rgba: Vec<u8>, width: u32, height: u32) -> Option<String> {
    let img: ImageBuffer<Rgba<u8>, Vec<u8>> = ImageBuffer::from_raw(width, height, rgba)?;

    let mut png_bytes: Vec<u8> = Vec::new();
    img.write_to(
        &mut std::io::Cursor::new(&mut png_bytes),
        image::ImageFormat::Png,
    )
    .ok()?;

    Some(format!(
        "data:image/png;base64,{}",
        STANDARD.encode(&png_bytes)
    ))
}

pub fn get_exe_icon_base64(exe_path: &str) -> Option<String> {
    let hicon = match get_icon_handle(exe_path) {
        Some(h) => h,
        None => {
            println!("Failed to get icon handle for {}", exe_path);
            return None;
        }
    };
    let (rgba, w, h) = match icon_to_rgba(hicon) {
        Some(v) => v,
        None => {
            println!("Failed to convert icon to RGBA for {}", exe_path);
            unsafe {
                let _ = DestroyIcon(hicon);
            }
            return None;
        }
    };
    unsafe {
        let _ = DestroyIcon(hicon);
    }
    // // TEMP DEBUG: write to disk so we can inspect it directly
    // if let Some(img) = image::ImageBuffer::<image::Rgba<u8>, _>::from_raw(w, h, rgba.clone()) {
    //     let _ = img.save("C:\\temp\\debug_icon.png");
    //     println!("Saved debug icon: {}x{}", w, h);
    // }

    let result = rgba_to_png_base64(rgba, w, h);
    if result.is_none() {
        println!("Failed to encode PNG for {}", exe_path);
    }
    result
}

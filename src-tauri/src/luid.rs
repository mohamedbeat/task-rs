// gpu/luid.rs
pub fn parse_luid_from_name(name: &str) -> Option<i64> {
    let idx = name.find("luid_")?;
    let rest = &name[idx + 5..];
    let mut parts = rest.splitn(3, '_');
    let high_str = parts.next()?.trim_start_matches("0x");
    let low_str = parts.next()?.trim_start_matches("0x");

    let high = u32::from_str_radix(high_str, 16).ok()?;
    let low = u32::from_str_radix(low_str, 16).ok()?;

    Some((((high as u64) << 32) | (low as u64)) as i64)
}

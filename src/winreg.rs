//! Tiny HKCU registry helpers (autostart, toast app registration).

use windows_sys::Win32::System::Registry::*;

fn wide(s: &str) -> Vec<u16> {
    s.encode_utf16().chain([0]).collect()
}

pub fn set(path: &str, name: &str, value: &str) -> bool {
    let v = wide(value);
    unsafe {
        RegSetKeyValueW(
            HKEY_CURRENT_USER,
            wide(path).as_ptr(),
            wide(name).as_ptr(),
            REG_SZ,
            v.as_ptr() as _,
            (v.len() * 2) as u32,
        ) == 0
    }
}

pub fn delete(path: &str, name: &str) -> bool {
    unsafe { RegDeleteKeyValueW(HKEY_CURRENT_USER, wide(path).as_ptr(), wide(name).as_ptr()) == 0 }
}

pub fn get(path: &str, name: &str) -> Option<String> {
    let mut buf = [0u16; 512];
    let mut size = (buf.len() * 2) as u32;
    let ok = unsafe {
        RegGetValueW(
            HKEY_CURRENT_USER,
            wide(path).as_ptr(),
            wide(name).as_ptr(),
            RRF_RT_REG_SZ,
            std::ptr::null_mut(),
            buf.as_mut_ptr() as _,
            &mut size,
        ) == 0
    };
    ok.then(|| String::from_utf16_lossy(&buf[..(size as usize / 2).saturating_sub(1)]))
}

pub fn exists(path: &str, name: &str) -> bool {
    unsafe {
        RegGetValueW(
            HKEY_CURRENT_USER,
            wide(path).as_ptr(),
            wide(name).as_ptr(),
            RRF_RT_REG_SZ,
            std::ptr::null_mut(),
            std::ptr::null_mut(),
            std::ptr::null_mut(),
        ) == 0
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn set_exists_delete_roundtrip() {
        let p = r"Software\SonyControlTest";
        assert!(super::set(p, "x", "yes"));
        assert!(super::exists(p, "x"));
        assert_eq!(super::get(p, "x").as_deref(), Some("yes"));
        assert!(super::delete(p, "x"));
        assert!(!super::exists(p, "x"));
    }
}

//! List audio outputs/microphones and set the Windows default device.

use serde::Serialize;
use std::ffi::c_void;
use windows::core::{interface, IUnknown, IUnknown_Vtbl, GUID, HRESULT, PCWSTR};
use windows::Win32::Devices::FunctionDiscovery::PKEY_Device_FriendlyName;
use windows::Win32::Media::Audio::*;
use windows::Win32::System::Com::*;

#[derive(Serialize, Clone)]
pub struct Dev {
    pub id: String,
    pub name: String,
    pub default: bool,
}

#[derive(Serialize)]
pub struct Devices {
    pub outputs: Vec<Dev>,
    pub inputs: Vec<Dev>,
}

// Undocumented but stable since Vista; what every "sound switcher" app uses.
#[interface("f8679f50-850a-41cf-9c72-430f290290c8")]
unsafe trait IPolicyConfig: IUnknown {
    fn GetMixFormat(&self, a: PCWSTR, b: *mut *mut c_void) -> HRESULT;
    fn GetDeviceFormat(&self, a: PCWSTR, b: i32, c: *mut *mut c_void) -> HRESULT;
    fn ResetDeviceFormat(&self, a: PCWSTR) -> HRESULT;
    fn SetDeviceFormat(&self, a: PCWSTR, b: *mut c_void, c: *mut c_void) -> HRESULT;
    fn GetProcessingPeriod(&self, a: PCWSTR, b: i32, c: *mut i64, d: *mut i64) -> HRESULT;
    fn SetProcessingPeriod(&self, a: PCWSTR, b: *mut i64) -> HRESULT;
    fn GetShareMode(&self, a: PCWSTR, b: *mut c_void) -> HRESULT;
    fn SetShareMode(&self, a: PCWSTR, b: *mut c_void) -> HRESULT;
    fn GetPropertyValue(&self, a: PCWSTR, b: *const c_void, c: *mut c_void) -> HRESULT;
    fn SetPropertyValue(&self, a: PCWSTR, b: *const c_void, c: *const c_void) -> HRESULT;
    fn SetDefaultEndpoint(&self, id: PCWSTR, role: ERole) -> HRESULT;
    fn SetEndpointVisibility(&self, a: PCWSTR, b: i32) -> HRESULT;
}
const CLSID_POLICY_CONFIG: GUID = GUID::from_u128(0x870af99c_171d_4f9e_af0d_e63df40c2bc9);

fn com() {
    // Already-initialized threads return an error we can ignore.
    unsafe {
        let _ = CoInitializeEx(None, COINIT_MULTITHREADED);
    }
}

fn enumerate(en: &IMMDeviceEnumerator, flow: EDataFlow) -> windows::core::Result<Vec<Dev>> {
    unsafe {
        let def_id = en
            .GetDefaultAudioEndpoint(flow, eConsole)
            .and_then(|d| d.GetId())
            .map(|p| {
                let s = p.to_string().unwrap_or_default();
                CoTaskMemFree(Some(p.0 as _));
                s
            })
            .unwrap_or_default();
        let col = en.EnumAudioEndpoints(flow, DEVICE_STATE_ACTIVE)?;
        let mut out = Vec::new();
        for i in 0..col.GetCount()? {
            let d = col.Item(i)?;
            let p = d.GetId()?;
            let id = p.to_string().unwrap_or_default();
            CoTaskMemFree(Some(p.0 as _));
            let name = d
                .OpenPropertyStore(STGM_READ)
                .and_then(|st| st.GetValue(&PKEY_Device_FriendlyName))
                .map(|v| v.to_string())
                .unwrap_or_else(|_| id.clone());
            out.push(Dev { default: id == def_id, id, name });
        }
        Ok(out)
    }
}

pub fn list() -> Result<Devices, String> {
    com();
    unsafe {
        let en: IMMDeviceEnumerator =
            CoCreateInstance(&MMDeviceEnumerator, None, CLSCTX_ALL).map_err(|e| e.to_string())?;
        Ok(Devices {
            outputs: enumerate(&en, eRender).map_err(|e| e.to_string())?,
            inputs: enumerate(&en, eCapture).map_err(|e| e.to_string())?,
        })
    }
}

#[cfg(test)]
mod tests {
    /// Needs real audio hardware: `cargo test -- --ignored`
    #[test]
    #[ignore]
    fn list_and_reapply_default() {
        let d = super::list().unwrap();
        let cur = d.outputs.iter().find(|x| x.default).expect("no default output");
        super::set_default(&cur.id).unwrap();
        let after = super::list().unwrap();
        assert!(after.outputs.iter().any(|x| x.default && x.id == cur.id));
        for x in d.outputs.iter().chain(&d.inputs) {
            eprintln!("{} {}", if x.default { "*" } else { " " }, x.name);
        }
    }
}

/// Makes the device default for all roles (games, music, calls).
pub fn set_default(id: &str) -> Result<(), String> {
    com();
    let wide: Vec<u16> = id.encode_utf16().chain([0]).collect();
    unsafe {
        let pc: IPolicyConfig =
            CoCreateInstance(&CLSID_POLICY_CONFIG, None, CLSCTX_ALL).map_err(|e| e.to_string())?;
        for role in [eConsole, eMultimedia, eCommunications] {
            pc.SetDefaultEndpoint(PCWSTR(wide.as_ptr()), role).ok().map_err(|e| e.to_string())?;
        }
    }
    Ok(())
}

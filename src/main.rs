#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

#[allow(non_snake_case)] // COM method names must match Windows
mod audio;
mod i18n;
mod sony;
mod winreg;

use i18n::{tr, S};

use std::sync::{Arc, Mutex};
use tauri::menu::{CheckMenuItem, Menu, MenuItem, PredefinedMenuItem};
use tauri::tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent};
use tauri::{AppHandle, Emitter, Manager, RunEvent, State, Wry};

type Hp = Arc<sony::Sony>;

const APP_ID: &str = "local.sonycontrol";
const RUN_KEY: &str = r"Software\Microsoft\Windows\CurrentVersion\Run";
const RUN_NAME: &str = "SonyControl";
const SETTINGS_KEY: &str = r"Software\SonyControl";

#[tauri::command]
fn lang_get() -> String {
    i18n::get().to_string()
}

/// Remembers the UI language and re-labels the tray menu in it.
#[tauri::command]
fn lang_set(app: AppHandle, code: String) {
    let code = i18n::set(&code);
    winreg::set(SETTINGS_KEY, "Lang", code);
    if let Some(t) = app.try_state::<TrayItems>() {
        let _ = t.open.set_text(tr(S::Open));
        let _ = t.quit.set_text(tr(S::Quit));
        for (m, s) in t.modes.iter().zip([S::Nc, S::Amb, S::Off]) {
            let _ = m.set_text(tr(s));
        }
    }
    if let Some(hp) = app.try_state::<Hp>() {
        update_tray(&app, &hp.state());
    }
}

#[tauri::command]
async fn hp_state(hp: State<'_, Hp>) -> Result<sony::HpState, String> {
    Ok(hp.state())
}

#[tauri::command]
async fn hp_set_nc(hp: State<'_, Hp>, mode: String, level: u8, voice: bool) -> Result<bool, String> {
    Ok(hp.set_nc(&mode, level, voice))
}

#[tauri::command]
async fn hp_set_eq(hp: State<'_, Hp>, preset: u8, bands: Vec<i8>) -> Result<bool, String> {
    Ok(hp.set_eq(preset, &bands))
}

#[tauri::command]
async fn hp_set(hp: State<'_, Hp>, what: String, v: u8) -> Result<bool, String> {
    Ok(hp.set(&what, v))
}

#[tauri::command]
async fn hp_switch(hp: State<'_, Hp>, mac: String) -> Result<bool, String> {
    Ok(hp.switch_source(&mac))
}

#[tauri::command]
async fn hp_auto_asm(hp: State<'_, Hp>, on: bool, sensitivity: u8) -> Result<bool, String> {
    Ok(hp.set_auto_asm(on, sensitivity))
}

/// Bluetooth name of this PC is normally the computer name; used to label it in the device list.
#[tauri::command]
fn pc_name() -> String {
    std::env::var("COMPUTERNAME").unwrap_or_default()
}

#[tauri::command]
async fn audio_list() -> Result<audio::Devices, String> {
    audio::list()
}

#[tauri::command]
async fn audio_set(id: String) -> Result<(), String> {
    audio::set_default(&id)
}

#[tauri::command]
fn autostart_get() -> bool {
    winreg::exists(RUN_KEY, RUN_NAME)
}

/// Autostart launches straight into the tray (`--tray`): no window, no WebView.
#[tauri::command]
fn autostart_set(on: bool) -> bool {
    if !on {
        return winreg::delete(RUN_KEY, RUN_NAME);
    }
    let exe = std::env::current_exe().map(|p| p.display().to_string()).unwrap_or_default();
    winreg::set(RUN_KEY, RUN_NAME, &format!("\"{exe}\" --tray"))
}

/// The user's headphones photo (never in the repo), as a data URL. Looked up in a `photo`
/// folder next to the exe first (portable, survives with the app), then in the app data folder.
/// headphones-1.png (three-quarter view) is preferred, else headphones-0.png. Async = off the UI thread.
#[tauri::command]
async fn photo(app: AppHandle) -> Option<String> {
    let exe_dir = std::env::current_exe().ok().and_then(|p| p.parent().map(|d| d.join("photo")));
    let dirs = [exe_dir, app.path().app_data_dir().ok()];
    dirs.iter()
        .flatten()
        .flat_map(|d| ["headphones-1.png", "headphones-0.png"].map(|f| d.join(f)))
        .find_map(|p| std::fs::read(p).ok())
        .or_else(|| (!EMBEDDED_PHOTO.is_empty()).then(|| EMBEDDED_PHOTO.to_vec()))
        .map(|b| format!("data:image/png;base64,{}", base64(&b)))
}

/// Picture baked in at build time via SONY_PHOTO (empty in public builds), see build.rs.
const EMBEDDED_PHOTO: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/embedded_photo.png"));

fn base64(b: &[u8]) -> String {
    const T: &[u8; 64] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    let mut s = String::with_capacity(b.len().div_ceil(3) * 4);
    for c in b.chunks(3) {
        let n = (c[0] as u32) << 16 | (*c.get(1).unwrap_or(&0) as u32) << 8 | *c.get(2).unwrap_or(&0) as u32;
        for i in 0..4 {
            s.push(if i <= c.len() { T[(n >> (18 - 6 * i) & 63) as usize] as char } else { '=' });
        }
    }
    s
}

/// The window (and its WebView) is destroyed on close to free memory; recreate it on demand.
fn show(app: &AppHandle) {
    show_try(app, 6);
}

/// WebView2 can refuse to start for a moment (e.g. a just-closed instance still holds its
/// data folder), so creation is retried a few times instead of silently giving up.
fn show_try(app: &AppHandle, tries: u8) {
    let w = match app.get_webview_window("main") {
        Some(w) => w,
        None => match tauri::WebviewWindowBuilder::from_config(app, &app.config().app.windows[0])
            .and_then(|b| b.build())
        {
            Ok(w) => w,
            Err(e) => {
                eprintln!("window: {e}");
                if tries > 0 {
                    let app = app.clone();
                    std::thread::spawn(move || {
                        std::thread::sleep(std::time::Duration::from_millis(700));
                        let a = app.clone();
                        let _ = app.run_on_main_thread(move || show_try(&a, tries - 1));
                    });
                }
                return;
            }
        },
    };
    let _ = w.show();
    let _ = w.unminimize();
    let _ = w.set_focus();
}

struct TrayItems {
    batt: MenuItem<Wry>,
    open: MenuItem<Wry>,
    quit: MenuItem<Wry>,
    modes: [CheckMenuItem<Wry>; 3],
}

/// Keeps the tray menu/tooltip in sync with the headphones.
fn update_tray(app: &AppHandle, s: &sony::HpState) {
    let Some(t) = app.try_state::<TrayItems>() else { return };
    let batt = match (s.connected, s.battery) {
        (false, _) => tr(S::NotConnected).to_string(),
        (true, Some(b)) => format!("{} — {b}%{}", s.name, if s.charging { " ⚡" } else { "" }),
        (true, None) => s.name.clone(),
    };
    let _ = t.batt.set_text(&batt);
    if let Some(tray) = app.tray_by_id("main") {
        let _ = tray.set_tooltip(Some(&batt));
    }
    let mode = if !s.nc_on { 2 } else if s.nc_mode == 1 { 1 } else { 0 };
    for (i, m) in t.modes.iter().enumerate() {
        let _ = m.set_enabled(s.connected);
        let _ = m.set_checked(s.connected && i == mode);
    }
}

/// Toast at 20% and 10%, once each until the battery is charged again.
fn battery_alert(s: &sony::HpState) {
    static LAST: Mutex<u8> = Mutex::new(100);
    let (Some(b), true) = (s.battery, s.connected) else { return };
    let mut last = LAST.lock().unwrap();
    if s.charging || b > 25 {
        *last = 100;
        return;
    }
    for t in [20u8, 10] {
        if b <= t && *last > t {
            *last = t;
            let _ = tauri_winrt_notification::Toast::new(APP_ID)
                .title(&tr(S::BatteryTitle).replace("{name}", &s.name).replace("{b}", &b.to_string()))
                .text1(tr(if t == 10 { S::VeryLow } else { S::Low }))
                .show();
            break;
        }
    }
}

fn main() {
    // Lets Windows show our toasts under the app's own name (unpackaged app registration).
    winreg::set(&format!(r"Software\Classes\AppUserModelId\{APP_ID}"), "DisplayName", "Sony Control");
    i18n::set(&winreg::get(SETTINGS_KEY, "Lang").unwrap_or_else(i18n::system_locale));

    tauri::Builder::default()
        .plugin(tauri_plugin_single_instance::init(|app, _, _| show(app)))
        .setup(|app| {
            let handle = app.handle().clone();
            let hp = sony::Sony::new(move |s| {
                #[cfg(debug_assertions)]
                eprintln!("hp: {}", serde_json::to_string(s).unwrap_or_default());
                let _ = handle.emit("hp", s);
                update_tray(&handle, s);
                battery_alert(s);
            });

            let batt = MenuItem::with_id(app, "batt", tr(S::Searching), false, None::<&str>)?;
            let modes = [
                CheckMenuItem::with_id(app, "nc", tr(S::Nc), false, false, None::<&str>)?,
                CheckMenuItem::with_id(app, "amb", tr(S::Amb), false, false, None::<&str>)?,
                CheckMenuItem::with_id(app, "off", tr(S::Off), false, false, None::<&str>)?,
            ];
            let open = MenuItem::with_id(app, "show", tr(S::Open), true, None::<&str>)?;
            let quit = MenuItem::with_id(app, "quit", tr(S::Quit), true, None::<&str>)?;
            let menu = Menu::with_items(
                app,
                &[
                    &batt,
                    &PredefinedMenuItem::separator(app)?,
                    &modes[0],
                    &modes[1],
                    &modes[2],
                    &PredefinedMenuItem::separator(app)?,
                    &open,
                    &quit,
                ],
            )?;
            app.manage(TrayItems { batt, open, quit, modes });
            TrayIconBuilder::with_id("main")
                .icon(app.default_window_icon().unwrap().clone())
                .tooltip("Sony Control")
                .menu(&menu)
                .show_menu_on_left_click(false)
                .on_menu_event(|app, e| match e.id.as_ref() {
                    "show" => show(app),
                    "quit" => app.exit(0),
                    m @ ("nc" | "amb" | "off") => {
                        let hp = app.state::<Hp>().inner().clone();
                        let m = m.to_string();
                        // Off the UI thread: sending waits for the headphones' ACK.
                        std::thread::spawn(move || {
                            let s = hp.state();
                            hp.set_nc(&m, s.amb_level, s.voice);
                        });
                    }
                    _ => {}
                })
                .on_tray_icon_event(|tray, e| {
                    if let TrayIconEvent::Click { button: MouseButton::Left, button_state: MouseButtonState::Up, .. } = e {
                        show(tray.app_handle());
                    }
                })
                .build(app)?;

            hp.start();
            app.manage(hp);
            if !std::env::args().any(|a| a == "--tray") {
                show(app.handle());
            }
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            hp_state, hp_set_nc, hp_set_eq, hp_set, hp_switch, hp_auto_asm, pc_name, audio_list, audio_set,
            autostart_get, autostart_set, photo, lang_get, lang_set
        ])
        .build(tauri::generate_context!())
        .expect("failed to start")
        .run(|_, e| {
            // Closing the last window keeps the app in the tray; only "Quit" (exit code set) quits.
            if let RunEvent::ExitRequested { api, code: None, .. } = e {
                api.prevent_exit();
            }
        });
}

#[cfg(test)]
mod tests {
    /// Shows a real Windows toast: `cargo test -- --ignored toast`
    #[test]
    #[ignore]
    fn toast_shows() {
        crate::winreg::set(&format!(r"Software\Classes\AppUserModelId\{}", super::APP_ID), "DisplayName", "Sony Control");
        tauri_winrt_notification::Toast::new(super::APP_ID)
            .title("WH-1000XM6: 20% заряду")
            .text1("Заряд закінчується.")
            .show()
            .expect("toast");
    }

    #[test]
    fn base64_matches_known_values() {
        assert_eq!(super::base64(b""), "");
        assert_eq!(super::base64(b"f"), "Zg==");
        assert_eq!(super::base64(b"fo"), "Zm8=");
        assert_eq!(super::base64(b"foo"), "Zm9v");
        assert_eq!(super::base64(b"foobar"), "Zm9vYmFy");
    }
}

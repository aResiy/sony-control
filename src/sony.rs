//! Sony headphones control over Bluetooth RFCOMM ("MDR" protocol v2).
//! Byte layouts ported from mos9527/SonyHeadphonesClient (MIT).

use serde::Serialize;
use std::mem::{size_of, zeroed};
use std::sync::{mpsc, Arc, Mutex};
use std::thread;
use std::time::Duration;
use windows_sys::core::GUID;
use windows_sys::Win32::Devices::Bluetooth::*;
use windows_sys::Win32::Networking::WinSock::*;

const START: u8 = 0x3E;
const END: u8 = 0x3C;
const ESC: u8 = 0x3D;
const T_ACK: u8 = 0x01;
const T_MDR: u8 = 0x0C;
const T_MDR2: u8 = 0x0E;
const SERVICE_V2: GUID = GUID::from_u128(0x956C7B26_D49A_4BA8_B03F_B17D393CB6E2);

/// Frame: '>' escape(type, seq, len BE32, data, checksum) '<'
pub fn pack(ty: u8, seq: u8, data: &[u8]) -> Vec<u8> {
    let mut raw = vec![ty, seq];
    raw.extend((data.len() as u32).to_be_bytes());
    raw.extend_from_slice(data);
    raw.push(raw.iter().fold(0u8, |a, b| a.wrapping_add(*b)));
    let mut out = vec![START];
    for b in raw {
        match b {
            START | END | ESC => out.extend([ESC, b - 0x10]),
            _ => out.push(b),
        }
    }
    out.push(END);
    out
}

/// Inverse of `pack`, given the bytes between the markers.
pub fn unpack(body: &[u8]) -> Option<(u8, u8, Vec<u8>)> {
    let mut raw = Vec::with_capacity(body.len());
    let mut it = body.iter();
    while let Some(&b) = it.next() {
        if b == ESC {
            let n = *it.next()?;
            if !(0x2C..=0x2E).contains(&n) {
                return None;
            }
            raw.push(n + 0x10);
        } else {
            raw.push(b);
        }
    }
    if raw.len() < 7 {
        return None;
    }
    let (msg, sum) = raw.split_at(raw.len() - 1);
    if msg.iter().fold(0u8, |a, b| a.wrapping_add(*b)) != sum[0] {
        return None;
    }
    let len = u32::from_be_bytes(msg[2..6].try_into().ok()?) as usize;
    let data = &msg[6..];
    (data.len() == len).then(|| (msg[0], msg[1], data.to_vec()))
}

#[derive(Clone, Serialize, Default)]
pub struct HpState {
    pub connected: bool,
    pub name: String,
    pub status: String,
    pub battery: Option<u8>,
    pub charging: bool,
    /// NC/ASM message variant the device answered with (0x17 or 0x19), 0 = unknown.
    pub nc_type: u8,
    pub nc_on: bool,
    /// 0 = noise cancelling, 1 = ambient
    pub nc_mode: u8,
    pub voice: bool,
    pub amb_level: u8,
    pub auto_asm: bool,
    pub sensitivity: u8,
    pub eq_preset: Option<u8>,
    /// Centered band values (-eq_range..=eq_range). 6 bands = Clear Bass + 5.
    pub eq_bands: Vec<i8>,
    pub eq_range: i8,
    // Player
    pub playing: bool,
    pub title: String,
    pub artist: String,
    pub volume: Option<u8>,
    // Settings; None = not reported by the device (hidden in the UI)
    pub speak_to_chat: Option<bool>,
    pub auto_pause: Option<bool>,
    pub dsee: Option<bool>,
    /// 0 = sound quality, 1 = stable connection
    pub conn_mode: Option<u8>,
    /// (variant 0x04|0x05, element)
    pub auto_off: Option<(u8, u8)>,
    pub general: Vec<GeneralSetting>,
    pub peers: Vec<Peer>,
    /// Background music mode: (variant 0x03|0x09, on, room 0 small / 1 middle / 2 large)
    pub bgm: Option<(u8, bool, u8)>,
    pub cinema: Option<bool>,
    /// What the NC/AMB button cycles through (Function enum: 0x01..0x04)
    pub nc_button: Option<u8>,
    /// Voice guidance volume (signed small steps)
    pub prompt_volume: Option<i8>,
    /// Sound pressure at the ears, dB
    pub spl: Option<u8>,
}

#[derive(Clone, Serialize)]
pub struct GeneralSetting {
    pub id: u8,
    pub name: String,
    pub on: bool,
}

#[derive(Clone, Serialize)]
pub struct Peer {
    pub mac: String,
    pub name: String,
    pub connected: bool,
    pub playing: bool,
}

/// Reads a u8-length-prefixed string at `*p`, advancing it.
fn pstr(d: &[u8], p: &mut usize) -> Option<String> {
    let len = *d.get(*p)? as usize;
    let s = String::from_utf8_lossy(d.get(*p + 1..*p + 1 + len)?).into_owned();
    *p += 1 + len;
    Some(s)
}

/// PERI_RET/NTFY_PARAM paired device list. `cod`: entries carry a 3-byte class of device.
fn parse_peers(d: &[u8], cod: bool) -> Option<Vec<Peer>> {
    let mut p = 3;
    let mut raw = Vec::new();
    for _ in 0..*d.get(2)? {
        let mac = String::from_utf8_lossy(d.get(p..p + 17)?).into_owned();
        let status = *d.get(p + 17)?;
        p += 18 + if cod { 3 } else { 0 };
        raw.push((mac, status, pstr(d, &mut p)?));
    }
    let playing_id = d.get(p).copied().unwrap_or(0);
    Some(
        raw.into_iter()
            .map(|(mac, st, name)| Peer { mac, name, connected: st != 0, playing: st != 0 && st == playing_id })
            .collect(),
    )
}

struct Conn {
    sock: SOCKET,
    seq: u8,
    acks: mpsc::Receiver<u8>,
    /// Consecutive commands without an ACK.
    misses: u8,
}

pub struct Sony {
    state: Mutex<HpState>,
    conn: Mutex<Option<Conn>>,
    wlock: Mutex<()>,
    on_change: Box<dyn Fn(&HpState) + Send + Sync>,
}

impl Sony {
    pub fn new(on_change: impl Fn(&HpState) + Send + Sync + 'static) -> Arc<Self> {
        Arc::new(Self {
            state: Mutex::new(HpState { status: "searching".into(), ..Default::default() }),
            conn: Mutex::new(None),
            wlock: Mutex::new(()),
            on_change: Box::new(on_change),
        })
    }

    pub fn state(&self) -> HpState {
        self.state.lock().unwrap().clone()
    }

    fn update(&self, f: impl FnOnce(&mut HpState)) {
        let s = {
            let mut g = self.state.lock().unwrap();
            f(&mut g);
            g.clone()
        };
        (self.on_change)(&s);
    }

    fn is_connected(&self) -> bool {
        self.conn.lock().unwrap().is_some()
    }

    /// Background loop: connect when headphones appear, refresh battery every minute.
    pub fn start(self: &Arc<Self>) {
        let me = self.clone();
        thread::spawn(move || {
            let mut tick = 0u32;
            loop {
                if !me.is_connected() {
                    if let Err(e) = me.connect() {
                        me.update(|s| s.status = e);
                    }
                } else {
                    if tick % 12 == 0 {
                        me.send(&[0x22, 0x00]);
                        me.send(&[0x22, 0x08]);
                    }
                    if tick % 3 == 0 {
                        me.sound_pressure();
                    }
                }
                tick = tick.wrapping_add(1);
                thread::sleep(Duration::from_secs(5));
            }
        });
    }

    fn connect(self: &Arc<Self>) -> Result<(), String> {
        let (name, addr) = find_device()?;
        let sock = rfcomm_connect(addr)?;
        let (tx, rx) = mpsc::channel();
        *self.conn.lock().unwrap() = Some(Conn { sock, seq: 0, acks: rx, misses: 0 });
        self.update(|s| {
            s.connected = true;
            s.name = name;
            s.status = "connected".into();
        });
        let me = self.clone();
        thread::spawn(move || me.read_loop(sock, tx));
        self.refresh();
        Ok(())
    }

    /// Handshake, then ask for the full state. Unsupported queries just go unanswered.
    fn refresh(&self) {
        for q in [
            &[0x00, 0x00][..], // protocol info
            &[0x66, 0x19],     // NC/ASM (with noise adaptation)
            &[0x66, 0x17],     // NC/ASM
            &[0x22, 0x00],     // battery
            &[0x22, 0x08],     // battery with threshold
            &[0x56, 0x00],     // EQ
            &[0xA6, 0x01],     // track name
            &[0xA6, 0x20],     // volume
            &[0xA2, 0x01],     // play/pause
            &[0xF6, 0x0C],     // speak-to-chat
            &[0xF6, 0x01],     // pause when taken off
            &[0xE6, 0x01],     // DSEE
            &[0xE6, 0x00],     // connection mode
            &[0x26, 0x05],     // auto power off (wearing detection)
            &[0x26, 0x04],     // auto power off
            &[0xE6, 0x09],     // background music mode (with error code)
            &[0xE6, 0x03],     // background music mode
            &[0xE6, 0x04],     // cinema
            &[0x66, 0x30],     // NC/AMB button function
        ] {
            self.send(q);
        }
        for id in 0xD1..=0xD4 {
            self.send(&[0xD0, id, 0x01]); // general setting name (English)
            self.send(&[0xD6, id]);       // general setting value
        }
        self.send_t(T_MDR2, &[0x36, 0x02]); // paired devices (multipoint)
        self.send_t(T_MDR2, &[0x36, 0x00]);
        self.send_t(T_MDR2, &[0x46, 0x20]); // voice guidance volume
        self.sound_pressure();
    }

    fn sound_pressure(&self) {
        self.send_t(T_MDR2, &[0x5A, 0x00]);
        self.send_t(T_MDR2, &[0x5A, 0x02]);
    }

    fn raw_send(&self, sock: SOCKET, buf: &[u8]) -> bool {
        let _g = self.wlock.lock().unwrap();
        unsafe { send(sock, buf.as_ptr(), buf.len() as i32, 0) == buf.len() as i32 }
    }

    pub fn send(&self, payload: &[u8]) -> bool {
        self.send_t(T_MDR, payload)
    }

    /// Sends one command (table 1 = T_MDR, table 2 = T_MDR2) and waits for the ACK.
    fn send_t(&self, ty: u8, payload: &[u8]) -> bool {
        let mut g = self.conn.lock().unwrap();
        let Some(c) = g.as_mut() else { return false };
        while c.acks.try_recv().is_ok() {}
        if !self.raw_send(c.sock, &pack(ty, c.seq, payload)) {
            return false;
        }
        match c.acks.recv_timeout(Duration::from_millis(1500)) {
            Ok(_) => {
                c.seq ^= 1;
                c.misses = 0;
                true
            }
            Err(_) => {
                // Headphones went away without the socket noticing: drop the link so
                // read_loop ends and the supervisor reconnects.
                c.misses += 1;
                if c.misses >= 3 {
                    unsafe { shutdown(c.sock, SD_BOTH) };
                }
                false
            }
        }
    }

    fn read_loop(&self, sock: SOCKET, acks: mpsc::Sender<u8>) {
        let mut buf = Vec::new();
        let mut tmp = [0u8; 2048];
        loop {
            let n = unsafe { recv(sock, tmp.as_mut_ptr(), tmp.len() as i32, 0) };
            if n <= 0 {
                break;
            }
            buf.extend_from_slice(&tmp[..n as usize]);
            while let Some(s) = buf.iter().position(|&b| b == START) {
                let Some(e) = buf[s..].iter().position(|&b| b == END).map(|e| e + s) else {
                    buf.drain(..s);
                    break;
                };
                let frame: Vec<u8> = buf.drain(..=e).skip(s).collect();
                let Some((ty, seq, data)) = unpack(&frame[1..frame.len() - 1]) else { continue };
                match ty {
                    T_ACK => {
                        let _ = acks.send(seq);
                    }
                    T_MDR | T_MDR2 => {
                        self.raw_send(sock, &pack(T_ACK, 1u8.wrapping_sub(seq), &[]));
                        if ty == T_MDR {
                            self.handle(&data);
                        } else {
                            self.handle2(&data);
                        }
                    }
                    _ => {}
                }
            }
            if !buf.contains(&START) {
                buf.clear(); // bytes outside any frame are noise; don't let them pile up
            }
        }
        unsafe { closesocket(sock) };
        *self.conn.lock().unwrap() = None;
        self.update(|s| {
            *s = HpState { status: "disconnected".into(), ..Default::default() }
        });
    }

    fn handle(&self, d: &[u8]) {
        match d {
            // NCASM_RET_PARAM / NCASM_NTFY_PARAM
            // XM6 answers both variants; the 0x17 reply is a stub once 0x19 is known.
            [0x67 | 0x69, 0x17, ..] if self.state().nc_type == 0x19 => {}
            [0x67 | 0x69, ty @ (0x17 | 0x19), _, on, mode, voice, level, rest @ ..] => self.update(|s| {
                s.nc_type = *ty;
                s.nc_on = *on == 1;
                s.nc_mode = *mode;
                s.voice = *voice == 1;
                s.amb_level = *level;
                if let [auto, sens, ..] = rest {
                    s.auto_asm = *auto == 1;
                    s.sensitivity = *sens;
                }
            }),
            // POWER_RET_STATUS / POWER_NTFY_STATUS (battery, battery with threshold)
            [0x23 | 0x25, 0x00 | 0x08, level, charging, ..] => self.update(|s| {
                s.battery = Some(*level);
                s.charging = *charging == 1;
            }),
            // EQEBB_RET_PARAM / EQEBB_NTFY_PARAM
            [0x57 | 0x59, 0x00, preset, n, bands @ ..] if bands.len() >= *n as usize => {
                let range: i8 = if *n == 10 { 6 } else { 10 };
                self.update(|s| {
                    s.eq_preset = Some(*preset);
                    s.eq_range = range;
                    s.eq_bands = bands[..*n as usize].iter().map(|b| *b as i8 - range).collect();
                })
            }
            // PLAY_RET/NTFY_PARAM: track names (title, album, artist, ...)
            [0xA7 | 0xA9, 0x01, ..] => {
                let mut p = 2;
                let mut names = Vec::new();
                while names.len() < 4 && p < d.len() {
                    p += 1; // status byte
                    match pstr(d, &mut p) {
                        Some(s) => names.push(s),
                        None => break,
                    }
                }
                self.update(|s| {
                    s.title = names.first().cloned().unwrap_or_default();
                    s.artist = names.get(2).cloned().unwrap_or_default();
                })
            }
            [0xA7 | 0xA9, 0x20, vol, ..] => self.update(|s| s.volume = Some(*vol)),
            // PLAY_RET/NTFY_STATUS: [type, enable, status(1 = playing)]
            [0xA3 | 0xA5, 0x01, _, status, ..] => self.update(|s| s.playing = *status == 1),
            // SYSTEM_RET/NTFY_PARAM (OnOff: 0 = ON)
            [0xF7 | 0xF9, 0x0C, v, ..] => self.update(|s| s.speak_to_chat = Some(*v == 0)),
            [0xF7 | 0xF9, 0x01, v, ..] => self.update(|s| s.auto_pause = Some(*v == 0)),
            // AUDIO_RET/NTFY_PARAM
            [0xE7 | 0xE9, 0x01, v, ..] => self.update(|s| s.dsee = Some(*v == 1)),
            [0xE7 | 0xE9, 0x00, v, ..] => self.update(|s| s.conn_mode = Some(*v)),
            // Listening mode: background music [type, on(0 = ON), room], cinema [type, on]
            [0xE7 | 0xE9, ty @ (0x03 | 0x09), on, room, ..] => self.update(|s| s.bgm = Some((*ty, *on == 0, *room))),
            [0xE7 | 0xE9, 0x04, on, ..] => self.update(|s| s.cinema = Some(*on == 0)),
            // NC/AMB button function
            [0x67 | 0x69, 0x30, f, ..] => self.update(|s| s.nc_button = Some(*f)),
            // POWER_RET/NTFY_PARAM auto power off
            [0x27 | 0x29, ty @ (0x04 | 0x05), cur, ..] => self.update(|s| s.auto_off = Some((*ty, *cur))),
            // GENERAL_SETTING_RET_CAPABILITY: [id, type(0 = bool), format, name]
            [0xD1, id, 0x00, _, ..] => {
                let mut p = 4;
                if let Some(name) = pstr(d, &mut p) {
                    let id = *id;
                    self.update(|s| match s.general.iter_mut().find(|g| g.id == id) {
                        Some(g) => g.name = name,
                        None => s.general.push(GeneralSetting { id, name, on: false }),
                    })
                }
            }
            // GENERAL_SETTING_RET/NTFY_PARAM: [id, type, value(0 = ON)]
            [0xD7 | 0xD9, id, 0x00, v, ..] => {
                let id = *id;
                self.update(|s| match s.general.iter_mut().find(|g| g.id == id) {
                    Some(g) => g.on = *v == 0,
                    None => s.general.push(GeneralSetting { id, name: String::new(), on: *v == 0 }),
                })
            }
            _ => {}
        }
    }

    /// Table 2 messages (multipoint).
    fn handle2(&self, d: &[u8]) {
        match d {
            [0x37 | 0x39, t @ (0x00 | 0x02), ..] => {
                if let Some(peers) = parse_peers(d, *t == 0x02) {
                    self.update(|s| s.peers = peers)
                }
            }
            // VOICE_GUIDANCE_RET_PARAM volume
            [0x47, 0x20, v, ..] => self.update(|s| s.prompt_volume = Some(*v as i8)),
            // SAFE_LISTENING_RET_EXTENDED_PARAM: [type, dB level, error cause]
            // 255 = not measured (e.g. paused): keep the last real value.
            [0x5B, 0x00 | 0x02, level, ..] if (1..=150).contains(level) => self.update(|s| s.spl = Some(*level)),
            // Source switched: [type, result, mac(17)]
            [0x3D, 0x01, _, mac @ ..] if mac.len() >= 17 => {
                let mac = String::from_utf8_lossy(&mac[..17]).into_owned();
                self.update(|s| s.peers.iter_mut().for_each(|p| p.playing = p.mac == mac))
            }
            _ => {}
        }
    }

    /// Simple settings. `v` meaning depends on `what`.
    pub fn set(&self, what: &str, v: u8) -> bool {
        let onoff = if v != 0 { 0x00 } else { 0x01 }; // device encodes ON = 0
        let ok = match what {
            "volume" => self.send(&[0xA8, 0x20, v]),
            // 0x07 play, 0x01 pause, 0x02 next, 0x03 previous
            "play" => self.send(&[0xA4, 0x01, 0x00, v]),
            "speak_to_chat" => self.send(&[0xF8, 0x0C, onoff, 0x01]),
            "auto_pause" => self.send(&[0xF8, 0x01, onoff]),
            "dsee" => self.send(&[0xE8, 0x01, (v != 0) as u8]),
            "conn_mode" => self.send(&[0xE8, 0x00, v]),
            "auto_off" => {
                let ty = self.state().auto_off.map_or(0x04, |a| a.0);
                self.send(&[0x28, ty, v, 0x00])
            }
            "power_off" => self.send(&[0x24, 0x03, 0x01]),
            // 0 standard, 1 background music, 2 cinema: switch the other mode off first.
            "listen" => self.set_listening(v),
            // room for background music: 0 small, 1 middle, 2 large
            "bgm_room" => {
                let ty = self.state().bgm.map_or(0x09, |b| b.0);
                self.send(&[0xE8, ty, 0x00, v])
            }
            "nc_button" => self.send(&[0x68, 0x30, v]),
            // v is the signed volume as a byte (two's complement)
            "prompt_volume" => self.send_t(T_MDR2, &[0x48, 0x20, v, 0x00]),
            id if id.starts_with("gs") => {
                let id = id[2..].parse::<u8>().unwrap_or(0);
                self.send(&[0xD8, id, 0x00, onoff])
            }
            _ => false,
        };
        // Read back so the UI shows what the headphones actually applied.
        match what {
            "volume" => self.send(&[0xA6, 0x20]),
            "speak_to_chat" => self.send(&[0xF6, 0x0C]),
            "auto_pause" => self.send(&[0xF6, 0x01]),
            "dsee" => self.send(&[0xE6, 0x01]),
            "conn_mode" => self.send(&[0xE6, 0x00]),
            "auto_off" => self.send(&[0x26, self.state().auto_off.map_or(0x04, |a| a.0)]),
            "listen" | "bgm_room" => {
                let ty = self.state().bgm.map_or(0x09, |b| b.0);
                self.send(&[0xE6, ty]) && self.send(&[0xE6, 0x04]) && self.send(&[0x52, 0x00])
            }
            "nc_button" => self.send(&[0x66, 0x30]),
            "prompt_volume" => self.send_t(T_MDR2, &[0x46, 0x20]),
            id if id.starts_with("gs") => self.send(&[0xD6, id[2..].parse::<u8>().unwrap_or(0)]),
            _ => true,
        };
        ok
    }

    /// Listening mode 0 standard / 1 background music / 2 cinema. The modes exclude each
    /// other, so the one being left is switched off before the new one goes on.
    fn set_listening(&self, mode: u8) -> bool {
        let s = self.state();
        let (bgm_ty, room) = s.bgm.map_or((0x09, 0), |b| (b.0, b.2));
        let bgm = |on: bool| self.send(&[0xE8, bgm_ty, if on { 0x00 } else { 0x01 }, room]);
        let cinema = |on: bool| self.send(&[0xE8, 0x04, if on { 0x00 } else { 0x01 }]);
        match mode {
            1 => cinema(false) && bgm(true),
            2 => bgm(false) && cinema(true),
            _ => bgm(false) && cinema(false),
        }
    }

    /// Multipoint: make `mac` (as reported in `peers`) the playing device.
    pub fn switch_source(&self, mac: &str) -> bool {
        let mut p = vec![0x3C, 0x01];
        p.extend(mac.bytes().take(17));
        self.send_t(T_MDR2, &p)
    }

    /// Auto ambient (noise adaptation) and its sensitivity: 0 standard, 1 high, 2 low.
    pub fn set_auto_asm(&self, on: bool, sensitivity: u8) -> bool {
        self.update(|s| {
            s.auto_asm = on;
            s.sensitivity = sensitivity;
        });
        let s = self.state();
        let mode = if !s.nc_on { "off" } else if s.nc_mode == 1 { "amb" } else { "nc" };
        self.set_nc(mode, s.amb_level, s.voice)
    }

    /// mode: "off" | "nc" | "amb"; level: ambient level 0..=20.
    pub fn set_nc(&self, mode: &str, level: u8, voice: bool) -> bool {
        let s = self.state();
        let ty = if s.nc_type == 0 { 0x19 } else { s.nc_type };
        let (on, m) = match mode {
            "off" => (0, s.nc_mode),
            "nc" => (1, 0),
            _ => (1, 1),
        };
        let mut p = vec![0x68, ty, 0x01, on, m, voice as u8, level];
        if ty == 0x19 {
            p.extend([s.auto_asm as u8, s.sensitivity]);
        }
        let ok = self.send(&p);
        if ok {
            self.update(|s| {
                s.nc_on = on == 1;
                s.nc_mode = m;
                s.voice = voice;
                s.amb_level = level;
            });
        }
        ok
    }

    /// Empty `bands` = switch preset only.
    pub fn set_eq(&self, preset: u8, bands: &[i8]) -> bool {
        let range = self.state().eq_range.max(1);
        let mut p = vec![0x58, 0x00, preset, bands.len() as u8];
        p.extend(bands.iter().map(|b| (b.clamp(&-range, &range) + range) as u8));
        let ok = self.send(&p);
        self.send(&[0x56, 0x00]);
        ok
    }
}

fn is_sony(name: &str) -> bool {
    name.starts_with("WH-") || name.starts_with("WF-") || name.contains("LinkBuds") || name.contains("ULT WEAR")
}

/// First connected Sony headphones: (name, Bluetooth address).
fn find_device() -> Result<(String, u64), String> {
    unsafe {
        let mut params: BLUETOOTH_DEVICE_SEARCH_PARAMS = zeroed();
        params.dwSize = size_of::<BLUETOOTH_DEVICE_SEARCH_PARAMS>() as u32;
        params.fReturnConnected = 1;
        let mut info: BLUETOOTH_DEVICE_INFO = zeroed();
        info.dwSize = size_of::<BLUETOOTH_DEVICE_INFO>() as u32;
        let h = BluetoothFindFirstDevice(&params, &mut info);
        if h as usize == 0 {
            return Err("no_bt".into());
        }
        let mut found = None;
        loop {
            let len = info.szName.iter().position(|&c| c == 0).unwrap_or(info.szName.len());
            let name = String::from_utf16_lossy(&info.szName[..len]);
            if info.fConnected != 0 && is_sony(&name) {
                found = Some((name, info.Address.Anonymous.ullLong));
                break;
            }
            if BluetoothFindNextDevice(h, &mut info) == 0 {
                break;
            }
        }
        BluetoothFindDeviceClose(h);
        found.ok_or_else(|| "no_sony".into())
    }
}

fn rfcomm_connect(addr: u64) -> Result<SOCKET, String> {
    unsafe {
        let mut wsa: WSADATA = zeroed();
        WSAStartup(0x202, &mut wsa);
        let s = socket(AF_BTH as _, SOCK_STREAM as _, BTHPROTO_RFCOMM as _);
        if s == INVALID_SOCKET {
            return Err(format!("socket:{}", WSAGetLastError()));
        }
        let mut sa: SOCKADDR_BTH = zeroed();
        sa.addressFamily = AF_BTH as _;
        sa.btAddr = addr;
        sa.serviceClassId = SERVICE_V2;
        let r = connect(s, &sa as *const _ as *const SOCKADDR, size_of::<SOCKADDR_BTH>() as i32);
        if r != 0 {
            let e = WSAGetLastError();
            closesocket(s);
            return Err(format!("connect_failed:{e}"));
        }
        Ok(s)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Needs the real headphones connected and no other app holding them:
    /// `cargo test -- --ignored live_settings_roundtrip`. Restores every setting it touches.
    #[test]
    #[ignore]
    fn live_settings_roundtrip() {
        let hp = Sony::new(|_| {});
        hp.connect().expect("connect");
        thread::sleep(Duration::from_secs(2));
        let s0 = hp.state();
        for (what, get) in [
            ("dsee", (|s: &HpState| s.dsee) as fn(&HpState) -> Option<bool>),
            ("speak_to_chat", |s: &HpState| s.speak_to_chat),
        ] {
            let before = get(&s0).unwrap_or_else(|| panic!("{what} not reported"));
            assert!(hp.set(what, (!before) as u8));
            thread::sleep(Duration::from_millis(500));
            assert_eq!(get(&hp.state()), Some(!before), "{what} did not flip");
            assert!(hp.set(what, before as u8));
            thread::sleep(Duration::from_millis(500));
            assert_eq!(get(&hp.state()), Some(before), "{what} not restored");
        }
        let vol = s0.volume.expect("volume");
        assert!(hp.set("volume", vol));
        thread::sleep(Duration::from_millis(500));
        assert_eq!(hp.state().volume, Some(vol));
        eprintln!("peers: {:?}", hp.state().peers.iter().map(|p| &p.name).collect::<Vec<_>>());
    }

    /// Sets auto power off to SONY_AUTO_OFF (default 17 = never; XM6 only accepts 16 "when taken
    /// off" and 17 "never") and reads it back:
    /// `cargo test -- --ignored live_auto_off`
    #[test]
    #[ignore]
    fn live_auto_off() {
        let v: u8 = std::env::var("SONY_AUTO_OFF").ok().and_then(|s| s.parse().ok()).unwrap_or(17);
        let hp = Sony::new(|_| {});
        hp.connect().expect("connect");
        thread::sleep(Duration::from_secs(2));
        eprintln!("before: {:?}", hp.state().auto_off);
        assert!(hp.set("auto_off", v));
        thread::sleep(Duration::from_millis(600));
        eprintln!("after: {:?}", hp.state().auto_off);
        assert_eq!(hp.state().auto_off.map(|a| a.1), Some(v));
    }

    /// `cargo test -- --ignored live_listening_modes` with the headphones connected.
    #[test]
    #[ignore]
    fn live_listening_modes() {
        let hp = Sony::new(|_| {});
        hp.connect().expect("connect");
        thread::sleep(Duration::from_secs(2));
        let s0 = hp.state();
        let mode = |s: &HpState| (s.bgm.map(|b| b.1), s.cinema);
        let start = mode(&s0);
        for (m, want) in [(2, (Some(false), Some(true))), (1, (Some(true), Some(false))), (0, (Some(false), Some(false)))] {
            assert!(hp.set("listen", m));
            thread::sleep(Duration::from_millis(600));
            assert_eq!(mode(&hp.state()), want, "listening mode {m}");
        }
        let restore = match start { (Some(true), _) => 1, (_, Some(true)) => 2, _ => 0 };
        assert!(hp.set("listen", restore));
        let pv = s0.prompt_volume.expect("prompt volume");
        assert!(hp.set("prompt_volume", (pv + 1) as u8));
        thread::sleep(Duration::from_millis(500));
        assert_eq!(hp.state().prompt_volume, Some(pv + 1));
        assert!(hp.set("prompt_volume", pv as u8));
    }

    #[test]
    fn frame_roundtrip_with_escapes() {
        let data = [0x3C, 0x3D, 0x3E, 0x00, 0x66];
        let f = pack(T_MDR, 1, &data);
        assert_eq!((f[0], *f.last().unwrap()), (START, END));
        assert!(!f[1..f.len() - 1].iter().any(|b| *b == START || *b == END));
        assert_eq!(unpack(&f[1..f.len() - 1]), Some((T_MDR, 1, data.to_vec())));
        // Known frame from the reference implementation: ACK seq 1
        assert_eq!(pack(T_ACK, 1, &[]), vec![0x3E, 0x01, 0x01, 0, 0, 0, 0, 0x02, 0x3C]);
    }
}

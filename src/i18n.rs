//! Tray menu and notification strings (the window UI translates itself in ui/i18n.js).

use std::sync::Mutex;

pub const SUPPORTED: [&str; 12] = ["en", "uk", "es", "pt", "de", "fr", "it", "pl", "ru", "zh", "ja", "ko"];

/// open, quit, noise cancelling, ambient, turn off NC, not connected, searching,
/// battery low, battery very low, "{name}: {b}% battery"
type Strings = [&'static str; 10];

const TEXT: [(&str, Strings); 12] = [
    ("en", ["Open", "Quit", "Noise Cancelling", "Ambient Sound", "Noise cancelling off", "Headphones not connected", "Looking for headphones…",
            "Battery is running low.", "Time to charge, they'll turn off soon.", "{name}: {b}% battery"]),
    ("uk", ["Відкрити", "Вийти", "Шумодав", "Ambient", "Вимкнути шумодав", "Навушники не підключені", "Пошук навушників…",
            "Заряд закінчується.", "Час заряджати, скоро вимкнуться.", "{name}: {b}% заряду"]),
    ("es", ["Abrir", "Salir", "Cancelación de ruido", "Sonido ambiente", "Desactivar cancelación", "Auriculares no conectados", "Buscando auriculares…",
            "Queda poca batería.", "Hora de cargar, se apagarán pronto.", "{name}: {b}% de batería"]),
    ("pt", ["Abrir", "Sair", "Cancelamento de ruído", "Som ambiente", "Desligar cancelamento", "Fones não conectados", "Procurando fones…",
            "A bateria está acabando.", "Hora de carregar, vão desligar em breve.", "{name}: {b}% de bateria"]),
    ("de", ["Öffnen", "Beenden", "Noise Cancelling", "Umgebungsgeräusche", "Geräuschunterdrückung aus", "Kopfhörer nicht verbunden", "Suche Kopfhörer…",
            "Akku wird schwach.", "Zeit zum Laden, sie schalten sich bald aus.", "{name}: {b}% Akku"]),
    ("fr", ["Ouvrir", "Quitter", "Réduction de bruit", "Son ambiant", "Désactiver la réduction", "Casque non connecté", "Recherche du casque…",
            "La batterie est faible.", "Il faut recharger, il va bientôt s'éteindre.", "{name} : {b} % de batterie"]),
    ("it", ["Apri", "Esci", "Cancellazione rumore", "Suono ambiente", "Disattiva cancellazione", "Cuffie non collegate", "Ricerca cuffie…",
            "Batteria quasi scarica.", "È ora di ricaricare, si spegneranno presto.", "{name}: {b}% di batteria"]),
    ("pl", ["Otwórz", "Zakończ", "Redukcja szumów", "Dźwięki otoczenia", "Wyłącz redukcję szumów", "Słuchawki nie są połączone", "Szukam słuchawek…",
            "Bateria się kończy.", "Czas naładować, wkrótce się wyłączą.", "{name}: {b}% baterii"]),
    ("ru", ["Открыть", "Выйти", "Шумоподавление", "Окружающий звук", "Выключить шумоподавление", "Наушники не подключены", "Поиск наушников…",
            "Заряд заканчивается.", "Пора заряжать, скоро выключатся.", "{name}: {b}% заряда"]),
    ("zh", ["打开", "退出", "降噪", "环境声", "关闭降噪", "耳机未连接", "正在查找耳机…",
            "电量不足。", "请充电，耳机即将关机。", "{name}：电量 {b}%"]),
    ("ja", ["開く", "終了", "ノイズキャンセリング", "外音取り込み", "ノイズキャンセリングをオフ", "ヘッドホン未接続", "ヘッドホンを検索中…",
            "バッテリー残量が少なくなっています。", "充電してください。まもなく電源が切れます。", "{name}：バッテリー {b}%"]),
    ("ko", ["열기", "종료", "노이즈 캔슬링", "주변 소리", "노이즈 캔슬링 끄기", "헤드폰이 연결되지 않음", "헤드폰 찾는 중…",
            "배터리가 부족합니다.", "충전하세요. 곧 꺼집니다.", "{name}: 배터리 {b}%"]),
];

static LANG: Mutex<&'static str> = Mutex::new("en");

pub enum S { Open, Quit, Nc, Amb, Off, NotConnected, Searching, Low, VeryLow, BatteryTitle }

pub fn tr(s: S) -> &'static str {
    let lang = *LANG.lock().unwrap();
    let row = TEXT.iter().find(|(l, _)| *l == lang).unwrap_or(&TEXT[0]);
    row.1[s as usize]
}

pub fn get() -> &'static str {
    *LANG.lock().unwrap()
}

/// Sets the language if supported ("pt-BR" → "pt"); returns the code now in use.
pub fn set(locale: &str) -> &'static str {
    let code = locale.split(['-', '_']).next().unwrap_or("").to_ascii_lowercase();
    if let Some(c) = SUPPORTED.iter().find(|c| **c == code) {
        *LANG.lock().unwrap() = c;
    }
    get()
}

/// Windows UI locale, e.g. "uk-UA".
pub fn system_locale() -> String {
    let mut buf = [0u16; 85];
    let n = unsafe { windows_sys::Win32::Globalization::GetUserDefaultLocaleName(buf.as_mut_ptr(), buf.len() as i32) };
    String::from_utf16_lossy(&buf[..(n.max(1) - 1) as usize])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn picks_supported_language_and_falls_back() {
        assert_eq!(set("pt-BR"), "pt");
        assert_eq!(tr(S::Quit), "Sair");
        assert_eq!(set("xx-YY"), "pt"); // unsupported: unchanged
        assert_eq!(set("en"), "en");
        assert_eq!(tr(S::Open), "Open");
    }
}

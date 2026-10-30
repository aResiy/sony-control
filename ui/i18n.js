// UI translations. Every language lists its strings in the same order as KEYS.
// Missing strings fall back to English, then to the key itself.
const KEYS = [
  "tab_sound", "tab_devices", "tab_settings", "profiles", "save_current", "cancel", "prof_hint", "prof_game", "prof_music", "prof_call",
  "nc_title", "mode_nc", "mode_nc_d", "mode_amb", "mode_amb_d", "mode_off", "mode_off_d", "mode_none_d",
  "amb_level", "amb_desc", "voice", "voice_sub", "voice_short", "auto_amb", "auto_amb_sub", "sens_low", "sens_std", "sens_high",
  "eq", "preset_off", "preset_custom",
  "peers_title", "peers_hint", "out_title", "mic_title", "mic_warn", "active", "playing", "connected", "not_connected", "this_pc",
  "smart", "stc_sub", "autopause", "autopause_sub", "gs_touch", "gs_touch_sub", "gs_multi", "gs_multi_sub", "gs_side", "gs_side_sub",
  "audio_conn", "dsee_sub", "bt_priority", "conn_quality", "conn_stable",
  "app", "language", "autostart", "autostart_sub", "theme",
  "power", "auto_off", "auto_off_sub", "ao_removed", "ao_5", "ao_15", "ao_30", "ao_60", "ao_180", "ao_never", "power_off", "power_off_confirm",
  "headphones", "nothing_playing", "prev", "next", "playpause",
  "st_searching", "st_connected", "st_disconnected", "st_no_bt", "st_no_sony", "st_connect_failed", "st_socket",
];

const LANGS = {
  en: ["English", [
    "Sound", "Devices", "Settings", "Profiles", "Save current", "Cancel", "Pick a profile to save the current settings to.", "Game", "Music", "Call",
    "Noise cancelling", "Noise Cancelling", "Silence around you", "Ambient Sound", "Hear what's around you", "Off", "No sound processing", "Headphones not connected",
    "Ambient sound level", "Level {n} of 20", "Focus on voice", "Hear people, not noise", "voice focus", "Adaptive Ambient", "Adjusts to the noise around you", "Low", "Standard", "High",
    "Equalizer", "Off", "Custom",
    "Headphones are playing from", "Click a connected device to switch the audio to it.", "PC sound output", "PC microphone", "The headphones' Bluetooth mic lowers sound quality. Better pick another one.", "active", "playing", "connected", "not connected", "(this PC)",
    "Smart features", "Pauses music when you start talking", "Pause when taken off", "Music stops when you take the headphones off", "Touch panel", "Touch controls on the right cup", "Two devices at once", "Multipoint: phone and PC together", "Hear your own voice", "In calls, so you don't talk too loud",
    "Sound & connection", "Upscales compressed music (Spotify, YouTube)", "Bluetooth priority", "Sound quality (LDAC)", "Stable connection",
    "App", "Language", "Start with Windows", "Straight to the tray, no window (≈6 MB RAM)", "Theme",
    "Power", "Auto power off", "When the headphones aren't in use", "When taken off", "5 min", "15 min", "30 min", "1 hour", "3 hours", "Never", "Turn off headphones", "Click again to turn off",
    "Headphones", "Nothing playing", "Previous", "Next", "Play / pause",
    "Looking for headphones…", "Connected", "Disconnected. Looking for headphones…", "No Bluetooth headphones connected", "No Sony headphones connected", "Couldn't connect to the headphones (code {code})", "Bluetooth error {code}",
  ]],
  uk: ["Українська", [
    "Звук", "Пристрої", "Налаштування", "Профілі", "Зберегти поточне", "Скасувати", "Вибери профіль, у який зберегти поточні налаштування.", "Гра", "Музика", "Дзвінок",
    "Шумоподавлення", "Шумодав", "Тиша навколо", "Ambient", "Чутно, що відбувається навколо", "Вимкнено", "Без обробки звуку", "Навушники не підключені",
    "Рівень звуків навколо", "Рівень {n} з 20", "Фокус на голосі", "Чутно людей, а не шум", "фокус на голосі", "Авто-Ambient", "Сам підлаштовується під шум навколо", "Низька", "Стандарт", "Висока",
    "Еквалайзер", "Вимк", "Свій",
    "Навушники грають з", "Натисни на підключений пристрій, щоб перемкнути звук на нього.", "Вихід звуку ПК", "Мікрофон ПК", "Мікрофон навушників по Bluetooth погіршує звук. Краще вибери інший.", "активний", "грає", "підключено", "не підключено", "(цей ПК)",
    "Розумні функції", "Пауза, коли ти починаєш говорити", "Пауза, коли знімаєш", "Музика зупиняється, коли зняв навушники", "Сенсорна панель", "Керування дотиком до правої чашки", "Два пристрої одночасно", "Multipoint: телефон і ПК разом", "Чути свій голос", "У дзвінках, щоб не говорити голосно",
    "Звук і з'єднання", "Покращує стиснуту музику (Spotify, YouTube)", "Пріоритет Bluetooth", "Якість звуку (LDAC)", "Стабільність",
    "Програма", "Мова", "Запускати з Windows", "Одразу в трей, без вікна (≈6 МБ пам'яті)", "Тема",
    "Живлення", "Автовимкнення", "Коли навушники не використовуються", "Коли зняв", "5 хв", "15 хв", "30 хв", "1 год", "3 год", "Ніколи", "Вимкнути навушники", "Натисни ще раз, щоб вимкнути",
    "Навушники", "Нічого не грає", "Попередній", "Наступний", "Пауза / грати",
    "Пошук навушників…", "Підключено", "Відключено. Шукаю навушники…", "Навушники не підключені по Bluetooth", "Навушники Sony не підключені", "Не вдалося підключитися до навушників (код {code})", "Помилка Bluetooth {code}",
  ]],
  es: ["Español", [
    "Sonido", "Dispositivos", "Ajustes", "Perfiles", "Guardar actual", "Cancelar", "Elige un perfil para guardar la configuración actual.", "Juego", "Música", "Llamada",
    "Cancelación de ruido", "Cancelación de ruido", "Silencio a tu alrededor", "Sonido ambiente", "Escucha lo que pasa a tu alrededor", "Desactivado", "Sin procesamiento de sonido", "Auriculares no conectados",
    "Nivel de sonido ambiente", "Nivel {n} de 20", "Enfoque en la voz", "Escucha a las personas, no el ruido", "enfoque en la voz", "Ambiente adaptativo", "Se ajusta al ruido a tu alrededor", "Baja", "Estándar", "Alta",
    "Ecualizador", "Desact.", "Personal",
    "Los auriculares suenan desde", "Haz clic en un dispositivo conectado para cambiar el audio a él.", "Salida de sonido del PC", "Micrófono del PC", "El micrófono Bluetooth de los auriculares empeora el sonido. Mejor elige otro.", "activo", "reproduciendo", "conectado", "no conectado", "(este PC)",
    "Funciones inteligentes", "Pausa cuando empiezas a hablar", "Pausa al quitártelos", "La música se detiene al quitarte los auriculares", "Panel táctil", "Control táctil en el auricular derecho", "Dos dispositivos a la vez", "Multipoint: teléfono y PC juntos", "Oír tu propia voz", "En llamadas, para no hablar demasiado alto",
    "Sonido y conexión", "Mejora la música comprimida (Spotify, YouTube)", "Prioridad Bluetooth", "Calidad de sonido (LDAC)", "Conexión estable",
    "Aplicación", "Idioma", "Iniciar con Windows", "Directo a la bandeja, sin ventana (≈6 MB de RAM)", "Tema",
    "Energía", "Apagado automático", "Cuando no se usan los auriculares", "Al quitártelos", "5 min", "15 min", "30 min", "1 hora", "3 horas", "Nunca", "Apagar auriculares", "Haz clic otra vez para apagar",
    "Auriculares", "No se reproduce nada", "Anterior", "Siguiente", "Reproducir / pausa",
    "Buscando auriculares…", "Conectado", "Desconectado. Buscando auriculares…", "No hay auriculares Bluetooth conectados", "No hay auriculares Sony conectados", "No se pudo conectar a los auriculares (código {code})", "Error de Bluetooth {code}",
  ]],
  pt: ["Português", [
    "Som", "Dispositivos", "Configurações", "Perfis", "Salvar atual", "Cancelar", "Escolha um perfil para salvar as configurações atuais.", "Jogo", "Música", "Chamada",
    "Cancelamento de ruído", "Cancelamento de ruído", "Silêncio ao seu redor", "Som ambiente", "Ouça o que acontece ao redor", "Desligado", "Sem processamento de som", "Fones não conectados",
    "Nível do som ambiente", "Nível {n} de 20", "Foco na voz", "Ouça pessoas, não ruído", "foco na voz", "Ambiente adaptativo", "Ajusta-se ao ruído ao redor", "Baixa", "Padrão", "Alta",
    "Equalizador", "Desl.", "Personalizado",
    "Os fones estão tocando de", "Clique em um dispositivo conectado para mudar o áudio para ele.", "Saída de som do PC", "Microfone do PC", "O microfone Bluetooth dos fones piora o som. Melhor escolher outro.", "ativo", "tocando", "conectado", "não conectado", "(este PC)",
    "Funções inteligentes", "Pausa quando você começa a falar", "Pausar ao tirar", "A música para quando você tira os fones", "Painel touch", "Controle por toque no fone direito", "Dois dispositivos ao mesmo tempo", "Multipoint: celular e PC juntos", "Ouvir a própria voz", "Em chamadas, para não falar alto demais",
    "Som e conexão", "Melhora música comprimida (Spotify, YouTube)", "Prioridade do Bluetooth", "Qualidade de som (LDAC)", "Conexão estável",
    "Aplicativo", "Idioma", "Iniciar com o Windows", "Direto para a bandeja, sem janela (≈6 MB de RAM)", "Tema",
    "Energia", "Desligamento automático", "Quando os fones não estão em uso", "Ao tirar", "5 min", "15 min", "30 min", "1 hora", "3 horas", "Nunca", "Desligar fones", "Clique de novo para desligar",
    "Fones de ouvido", "Nada tocando", "Anterior", "Próxima", "Tocar / pausar",
    "Procurando fones…", "Conectado", "Desconectado. Procurando fones…", "Nenhum fone Bluetooth conectado", "Nenhum fone Sony conectado", "Não foi possível conectar aos fones (código {code})", "Erro de Bluetooth {code}",
  ]],
  de: ["Deutsch", [
    "Klang", "Geräte", "Einstellungen", "Profile", "Aktuelle speichern", "Abbrechen", "Wähle ein Profil, in dem die aktuellen Einstellungen gespeichert werden.", "Spiel", "Musik", "Anruf",
    "Geräuschunterdrückung", "Noise Cancelling", "Stille um dich herum", "Umgebungsgeräusche", "Hör, was um dich herum passiert", "Aus", "Keine Klangverarbeitung", "Kopfhörer nicht verbunden",
    "Pegel der Umgebungsgeräusche", "Stufe {n} von 20", "Fokus auf Stimmen", "Menschen hören, nicht Lärm", "Stimmenfokus", "Adaptiver Umgebungsmodus", "Passt sich dem Lärm um dich an", "Niedrig", "Standard", "Hoch",
    "Equalizer", "Aus", "Eigene",
    "Kopfhörer spielen von", "Klicke auf ein verbundenes Gerät, um den Ton dorthin zu wechseln.", "PC-Tonausgabe", "PC-Mikrofon", "Das Bluetooth-Mikrofon der Kopfhörer verschlechtert den Klang. Wähle lieber ein anderes.", "aktiv", "spielt", "verbunden", "nicht verbunden", "(dieser PC)",
    "Smarte Funktionen", "Pausiert, wenn du zu sprechen beginnst", "Pause beim Abnehmen", "Musik stoppt, wenn du die Kopfhörer abnimmst", "Touch-Bedienfeld", "Touch-Steuerung an der rechten Muschel", "Zwei Geräte gleichzeitig", "Multipoint: Handy und PC zusammen", "Eigene Stimme hören", "Bei Anrufen, damit du nicht zu laut sprichst",
    "Klang & Verbindung", "Verbessert komprimierte Musik (Spotify, YouTube)", "Bluetooth-Priorität", "Klangqualität (LDAC)", "Stabile Verbindung",
    "App", "Sprache", "Mit Windows starten", "Direkt in den Infobereich, ohne Fenster (≈6 MB RAM)", "Design",
    "Energie", "Automatisch ausschalten", "Wenn die Kopfhörer nicht benutzt werden", "Beim Abnehmen", "5 Min.", "15 Min.", "30 Min.", "1 Std.", "3 Std.", "Nie", "Kopfhörer ausschalten", "Zum Ausschalten erneut klicken",
    "Kopfhörer", "Keine Wiedergabe", "Zurück", "Weiter", "Wiedergabe / Pause",
    "Suche Kopfhörer…", "Verbunden", "Getrennt. Suche Kopfhörer…", "Keine Bluetooth-Kopfhörer verbunden", "Keine Sony-Kopfhörer verbunden", "Verbindung zu den Kopfhörern fehlgeschlagen (Code {code})", "Bluetooth-Fehler {code}",
  ]],
  fr: ["Français", [
    "Son", "Appareils", "Paramètres", "Profils", "Enregistrer l'actuel", "Annuler", "Choisis un profil où enregistrer les réglages actuels.", "Jeu", "Musique", "Appel",
    "Réduction de bruit", "Réduction de bruit", "Le silence autour de toi", "Son ambiant", "Entends ce qui se passe autour", "Désactivé", "Aucun traitement du son", "Casque non connecté",
    "Niveau du son ambiant", "Niveau {n} sur 20", "Focus sur la voix", "Entendre les gens, pas le bruit", "focus voix", "Ambiant adaptatif", "S'adapte au bruit autour de toi", "Faible", "Standard", "Élevée",
    "Égaliseur", "Désact.", "Perso",
    "Le casque joue depuis", "Clique sur un appareil connecté pour y basculer le son.", "Sortie audio du PC", "Micro du PC", "Le micro Bluetooth du casque dégrade le son. Mieux vaut en choisir un autre.", "actif", "en lecture", "connecté", "non connecté", "(ce PC)",
    "Fonctions intelligentes", "Pause quand tu commences à parler", "Pause au retrait", "La musique s'arrête quand tu retires le casque", "Panneau tactile", "Commandes tactiles sur l'écouteur droit", "Deux appareils à la fois", "Multipoint : téléphone et PC ensemble", "Entendre sa propre voix", "Pendant les appels, pour ne pas parler trop fort",
    "Son et connexion", "Améliore la musique compressée (Spotify, YouTube)", "Priorité Bluetooth", "Qualité sonore (LDAC)", "Connexion stable",
    "Application", "Langue", "Lancer avec Windows", "Directement dans la zone de notification, sans fenêtre (≈6 Mo de RAM)", "Thème",
    "Alimentation", "Arrêt automatique", "Quand le casque n'est pas utilisé", "Au retrait", "5 min", "15 min", "30 min", "1 h", "3 h", "Jamais", "Éteindre le casque", "Clique à nouveau pour éteindre",
    "Casque", "Rien en lecture", "Précédent", "Suivant", "Lecture / pause",
    "Recherche du casque…", "Connecté", "Déconnecté. Recherche du casque…", "Aucun casque Bluetooth connecté", "Aucun casque Sony connecté", "Impossible de se connecter au casque (code {code})", "Erreur Bluetooth {code}",
  ]],
  it: ["Italiano", [
    "Suono", "Dispositivi", "Impostazioni", "Profili", "Salva attuale", "Annulla", "Scegli un profilo in cui salvare le impostazioni attuali.", "Gioco", "Musica", "Chiamata",
    "Cancellazione del rumore", "Cancellazione rumore", "Silenzio intorno a te", "Suono ambiente", "Senti cosa succede intorno", "Disattivato", "Nessuna elaborazione del suono", "Cuffie non collegate",
    "Livello suono ambiente", "Livello {n} di 20", "Focus sulla voce", "Senti le persone, non il rumore", "focus voce", "Ambiente adattivo", "Si adatta al rumore intorno a te", "Bassa", "Standard", "Alta",
    "Equalizzatore", "Off", "Personale",
    "Le cuffie suonano da", "Fai clic su un dispositivo collegato per spostare l'audio lì.", "Uscita audio del PC", "Microfono del PC", "Il microfono Bluetooth delle cuffie peggiora l'audio. Meglio sceglierne un altro.", "attivo", "in riproduzione", "collegato", "non collegato", "(questo PC)",
    "Funzioni intelligenti", "Pausa quando inizi a parlare", "Pausa quando le togli", "La musica si ferma quando togli le cuffie", "Pannello touch", "Comandi touch sul padiglione destro", "Due dispositivi insieme", "Multipoint: telefono e PC insieme", "Senti la tua voce", "Nelle chiamate, per non parlare troppo forte",
    "Suono e connessione", "Migliora la musica compressa (Spotify, YouTube)", "Priorità Bluetooth", "Qualità audio (LDAC)", "Connessione stabile",
    "App", "Lingua", "Avvia con Windows", "Direttamente nell'area di notifica, senza finestra (≈6 MB di RAM)", "Tema",
    "Alimentazione", "Spegnimento automatico", "Quando le cuffie non sono in uso", "Quando le togli", "5 min", "15 min", "30 min", "1 ora", "3 ore", "Mai", "Spegni le cuffie", "Fai di nuovo clic per spegnere",
    "Cuffie", "Niente in riproduzione", "Precedente", "Successivo", "Riproduci / pausa",
    "Ricerca cuffie…", "Collegato", "Scollegato. Ricerca cuffie…", "Nessuna cuffia Bluetooth collegata", "Nessuna cuffia Sony collegata", "Impossibile collegarsi alle cuffie (codice {code})", "Errore Bluetooth {code}",
  ]],
  pl: ["Polski", [
    "Dźwięk", "Urządzenia", "Ustawienia", "Profile", "Zapisz bieżące", "Anuluj", "Wybierz profil, do którego zapisać bieżące ustawienia.", "Gra", "Muzyka", "Rozmowa",
    "Redukcja szumów", "Redukcja szumów", "Cisza wokół ciebie", "Dźwięki otoczenia", "Słyszysz, co dzieje się wokół", "Wyłączone", "Bez przetwarzania dźwięku", "Słuchawki nie są połączone",
    "Poziom dźwięków otoczenia", "Poziom {n} z 20", "Skupienie na głosie", "Słyszysz ludzi, nie hałas", "skupienie na głosie", "Adaptacyjne otoczenie", "Dopasowuje się do hałasu wokół", "Niska", "Standard", "Wysoka",
    "Korektor", "Wył.", "Własny",
    "Słuchawki grają z", "Kliknij połączone urządzenie, aby przełączyć na nie dźwięk.", "Wyjście dźwięku PC", "Mikrofon PC", "Mikrofon Bluetooth słuchawek pogarsza dźwięk. Lepiej wybierz inny.", "aktywne", "gra", "połączone", "niepołączone", "(ten PC)",
    "Inteligentne funkcje", "Pauza, gdy zaczynasz mówić", "Pauza po zdjęciu", "Muzyka zatrzymuje się po zdjęciu słuchawek", "Panel dotykowy", "Sterowanie dotykiem na prawej muszli", "Dwa urządzenia naraz", "Multipoint: telefon i PC razem", "Słysz własny głos", "W rozmowach, by nie mówić za głośno",
    "Dźwięk i połączenie", "Poprawia skompresowaną muzykę (Spotify, YouTube)", "Priorytet Bluetooth", "Jakość dźwięku (LDAC)", "Stabilne połączenie",
    "Aplikacja", "Język", "Uruchamiaj z Windows", "Od razu w zasobniku, bez okna (≈6 MB RAM)", "Motyw",
    "Zasilanie", "Automatyczne wyłączanie", "Gdy słuchawki nie są używane", "Po zdjęciu", "5 min", "15 min", "30 min", "1 godz.", "3 godz.", "Nigdy", "Wyłącz słuchawki", "Kliknij ponownie, aby wyłączyć",
    "Słuchawki", "Nic nie gra", "Poprzedni", "Następny", "Odtwarzaj / pauza",
    "Szukam słuchawek…", "Połączono", "Rozłączono. Szukam słuchawek…", "Brak połączonych słuchawek Bluetooth", "Brak połączonych słuchawek Sony", "Nie udało się połączyć ze słuchawkami (kod {code})", "Błąd Bluetooth {code}",
  ]],
  ru: ["Русский", [
    "Звук", "Устройства", "Настройки", "Профили", "Сохранить текущее", "Отмена", "Выбери профиль, в который сохранить текущие настройки.", "Игра", "Музыка", "Звонок",
    "Шумоподавление", "Шумоподавление", "Тишина вокруг", "Окружающий звук", "Слышно, что происходит вокруг", "Выключено", "Без обработки звука", "Наушники не подключены",
    "Уровень окружающего звука", "Уровень {n} из 20", "Фокус на голосе", "Слышно людей, а не шум", "фокус на голосе", "Адаптивный режим", "Сам подстраивается под шум вокруг", "Низкая", "Стандарт", "Высокая",
    "Эквалайзер", "Выкл", "Свой",
    "Наушники играют с", "Нажми на подключённое устройство, чтобы переключить звук на него.", "Вывод звука ПК", "Микрофон ПК", "Bluetooth-микрофон наушников ухудшает звук. Лучше выбери другой.", "активный", "играет", "подключено", "не подключено", "(этот ПК)",
    "Умные функции", "Пауза, когда ты начинаешь говорить", "Пауза при снятии", "Музыка останавливается, когда снимаешь наушники", "Сенсорная панель", "Управление касанием правой чашки", "Два устройства одновременно", "Multipoint: телефон и ПК вместе", "Слышать свой голос", "В звонках, чтобы не говорить громко",
    "Звук и соединение", "Улучшает сжатую музыку (Spotify, YouTube)", "Приоритет Bluetooth", "Качество звука (LDAC)", "Стабильность",
    "Программа", "Язык", "Запускать с Windows", "Сразу в трей, без окна (≈6 МБ памяти)", "Тема",
    "Питание", "Автовыключение", "Когда наушники не используются", "При снятии", "5 мин", "15 мин", "30 мин", "1 ч", "3 ч", "Никогда", "Выключить наушники", "Нажми ещё раз, чтобы выключить",
    "Наушники", "Ничего не играет", "Предыдущий", "Следующий", "Пауза / играть",
    "Поиск наушников…", "Подключено", "Отключено. Ищу наушники…", "Bluetooth-наушники не подключены", "Наушники Sony не подключены", "Не удалось подключиться к наушникам (код {code})", "Ошибка Bluetooth {code}",
  ]],
  zh: ["简体中文", [
    "声音", "设备", "设置", "情景模式", "保存当前", "取消", "选择一个情景模式来保存当前设置。", "游戏", "音乐", "通话",
    "降噪", "降噪", "周围一片安静", "环境声", "听见周围的动静", "关闭", "不处理声音", "耳机未连接",
    "环境声级别", "级别 {n}/20", "人声聚焦", "听见人声而非噪音", "人声聚焦", "自适应环境声", "根据周围噪音自动调节", "低", "标准", "高",
    "均衡器", "关闭", "自定义",
    "耳机正在播放的设备", "点击已连接的设备，将音频切换过去。", "电脑声音输出", "电脑麦克风", "耳机的蓝牙麦克风会降低音质，建议选择其他麦克风。", "使用中", "播放中", "已连接", "未连接", "（本机）",
    "智能功能", "开口说话时自动暂停", "摘下时暂停", "摘下耳机时音乐自动停止", "触控面板", "右耳罩触控操作", "同时连接两台设备", "多点连接：手机和电脑同时使用", "听见自己的声音", "通话时避免说话太大声",
    "声音与连接", "提升压缩音乐的音质（Spotify、YouTube）", "蓝牙优先", "音质优先 (LDAC)", "连接稳定优先",
    "应用", "语言", "开机自启", "直接进入托盘，不打开窗口（约 6 MB 内存）", "主题",
    "电源", "自动关机", "耳机闲置时", "摘下时", "5 分钟", "15 分钟", "30 分钟", "1 小时", "3 小时", "从不", "关闭耳机", "再点一次以关闭",
    "耳机", "未在播放", "上一首", "下一首", "播放 / 暂停",
    "正在查找耳机…", "已连接", "已断开，正在查找耳机…", "未连接蓝牙耳机", "未连接索尼耳机", "无法连接耳机（错误码 {code}）", "蓝牙错误 {code}",
  ]],
  ja: ["日本語", [
    "サウンド", "デバイス", "設定", "プロファイル", "現在の設定を保存", "キャンセル", "現在の設定を保存するプロファイルを選んでください。", "ゲーム", "音楽", "通話",
    "ノイズキャンセリング", "ノイズキャンセリング", "周りが静かに", "外音取り込み", "周囲の音が聞こえる", "オフ", "音声処理なし", "ヘッドホン未接続",
    "外音取り込みレベル", "レベル {n}/20", "ボイスフォーカス", "騒音ではなく人の声を聞く", "ボイスフォーカス", "アダプティブ外音取り込み", "周囲の騒音に合わせて自動調整", "低", "標準", "高",
    "イコライザー", "オフ", "カスタム",
    "再生中のデバイス", "接続中のデバイスをクリックすると音声を切り替えます。", "PC の音声出力", "PC のマイク", "ヘッドホンの Bluetooth マイクは音質を下げます。別のマイクを選んでください。", "使用中", "再生中", "接続済み", "未接続", "（この PC）",
    "スマート機能", "話し始めると一時停止", "外すと一時停止", "ヘッドホンを外すと音楽が止まる", "タッチパネル", "右ハウジングのタッチ操作", "2 台同時接続", "マルチポイント：スマホと PC を同時に", "自分の声を聞く", "通話で声が大きくなりすぎないように",
    "サウンドと接続", "圧縮音源を高音質化（Spotify、YouTube）", "Bluetooth 優先", "音質優先 (LDAC)", "接続優先",
    "アプリ", "言語", "Windows 起動時に開始", "ウィンドウなしでトレイに常駐（約 6 MB）", "テーマ",
    "電源", "自動電源オフ", "ヘッドホンを使っていないとき", "外したとき", "5 分", "15 分", "30 分", "1 時間", "3 時間", "しない", "ヘッドホンの電源を切る", "もう一度クリックで電源オフ",
    "ヘッドホン", "再生していません", "前へ", "次へ", "再生 / 一時停止",
    "ヘッドホンを検索中…", "接続済み", "切断されました。ヘッドホンを検索中…", "Bluetooth ヘッドホンが接続されていません", "ソニーのヘッドホンが接続されていません", "ヘッドホンに接続できませんでした（コード {code}）", "Bluetooth エラー {code}",
  ]],
  ko: ["한국어", [
    "사운드", "기기", "설정", "프로필", "현재 설정 저장", "취소", "현재 설정을 저장할 프로필을 선택하세요.", "게임", "음악", "통화",
    "노이즈 캔슬링", "노이즈 캔슬링", "주변을 조용하게", "주변 소리", "주변 상황을 들을 수 있어요", "끄기", "음향 처리 없음", "헤드폰이 연결되지 않음",
    "주변 소리 레벨", "레벨 {n}/20", "음성 포커스", "소음 대신 사람 목소리를", "음성 포커스", "적응형 주변 소리", "주변 소음에 맞춰 자동 조절", "낮음", "표준", "높음",
    "이퀄라이저", "끄기", "사용자",
    "헤드폰 재생 중인 기기", "연결된 기기를 클릭하면 오디오가 전환됩니다.", "PC 소리 출력", "PC 마이크", "헤드폰의 블루투스 마이크는 음질을 떨어뜨립니다. 다른 마이크를 선택하세요.", "사용 중", "재생 중", "연결됨", "연결 안 됨", "(이 PC)",
    "스마트 기능", "말하기 시작하면 일시정지", "벗으면 일시정지", "헤드폰을 벗으면 음악이 멈춥니다", "터치 패널", "오른쪽 이어컵 터치 조작", "두 기기 동시 연결", "멀티포인트: 휴대폰과 PC 함께", "내 목소리 듣기", "통화 중 너무 크게 말하지 않도록",
    "사운드 및 연결", "압축 음원 음질 향상 (Spotify, YouTube)", "블루투스 우선순위", "음질 우선 (LDAC)", "연결 안정성",
    "앱", "언어", "Windows 시작 시 실행", "창 없이 트레이로 바로 (약 6MB 메모리)", "테마",
    "전원", "자동 전원 끄기", "헤드폰을 사용하지 않을 때", "벗었을 때", "5분", "15분", "30분", "1시간", "3시간", "안 함", "헤드폰 끄기", "끄려면 한 번 더 클릭",
    "헤드폰", "재생 중인 항목 없음", "이전", "다음", "재생 / 일시정지",
    "헤드폰 찾는 중…", "연결됨", "연결 끊김. 헤드폰 찾는 중…", "연결된 블루투스 헤드폰 없음", "연결된 소니 헤드폰 없음", "헤드폰에 연결할 수 없습니다 (코드 {code})", "블루투스 오류 {code}",
  ]],
};

let LANG = "en";
const DICT = {};
for (const [code, [, list]] of Object.entries(LANGS)) {
  if (list.length !== KEYS.length) console.error(`i18n: ${code} has ${list.length} strings, expected ${KEYS.length}`);
  DICT[code] = Object.fromEntries(KEYS.map((k, i) => [k, list[i]]));
}

/** Translate `key`, filling `{name}` placeholders from `vars`. */
function t(key, vars) {
  let s = DICT[LANG]?.[key] ?? DICT.en[key] ?? key;
  if (vars) for (const [k, v] of Object.entries(vars)) s = s.replace(`{${k}}`, v);
  return s;
}

/** Best match for a locale like "pt-BR" among the supported languages, else English. */
function pickLang(locale) {
  const code = (locale || "").toLowerCase().split(/[-_]/)[0];
  return code in LANGS ? code : "en";
}

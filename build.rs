fn main() {
    // Optional personal headphones picture baked into the exe: set SONY_PHOTO to a PNG path
    // at build time. Without it an empty file is embedded and the app uses its drawn model.
    let out = std::path::Path::new(&std::env::var("OUT_DIR").unwrap()).join("embedded_photo.png");
    println!("cargo:rerun-if-env-changed=SONY_PHOTO");
    match std::env::var("SONY_PHOTO") {
        Ok(p) if !p.is_empty() => {
            println!("cargo:rerun-if-changed={p}");
            std::fs::copy(&p, &out).unwrap_or_else(|e| panic!("SONY_PHOTO {p}: {e}"));
        }
        _ => std::fs::write(&out, []).unwrap(),
    }
    tauri_build::build()
}

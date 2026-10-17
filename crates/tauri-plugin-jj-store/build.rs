fn main() {
    #[cfg(feature = "tauri")]
    tauri_plugin::Builder::new(&["heads", "open", "status", "read", "close"]).build();
}

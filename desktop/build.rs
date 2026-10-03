fn main() {
    #[cfg(feature = "native")]
    tauri_build::try_build(
        tauri_build::Attributes::new()
            .app_manifest(tauri_build::AppManifest::new().commands(&["set_glass_background"])),
    )
    .expect("desktop build and command permissions");
}

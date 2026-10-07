fn main() {
    #[cfg(feature = "native")]
    tauri_build::try_build(tauri_build::Attributes::new().app_manifest(
        tauri_build::AppManifest::new().commands(&[
            "set_glass_background",
            "agent_power",
            "terminal_targets",
            "terminal_list",
            "terminal_open",
            "terminal_attach",
            "terminal_close",
            "terminal_input",
            "terminal_resize",
            "terminal_scroll",
            "terminal_focus",
            "terminal_ack",
            "terminal_viewport_ready",
            "terminal_detach",
        ]),
    ))
    .expect("desktop build and command permissions");
}

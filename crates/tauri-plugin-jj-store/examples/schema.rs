fn main() {
    let schema = schemars::schema_for!(tauri_plugin_jj_store::frames::Contract);
    println!("{}", serde_json::to_string_pretty(&schema).unwrap());
}

fn main() {
    println!(
        "{}",
        serde_json::to_string_pretty(&jj_store_http::contract::openapi()).unwrap()
    );
}

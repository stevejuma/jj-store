//! Disposable browser acceptance host using the production static asset router.
//! Native API requests are supplied by the browser test's captured fixtures.
#[tokio::main(flavor = "multi_thread", worker_threads = 2)]
async fn main() {
    let assets = jj_store_http::assets::BrowserAssets::load(std::path::Path::new(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../web/dist"
    )))
    .expect("run pnpm web:build before browser acceptance");
    let listener = tokio::net::TcpListener::bind("127.0.0.1:7458")
        .await
        .unwrap();
    axum::serve(listener, assets.router()).await.unwrap();
}

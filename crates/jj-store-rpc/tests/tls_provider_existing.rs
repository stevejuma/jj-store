use std::sync::Arc;

// This process represents a host that selects its provider before using RPC.
#[tokio::test]
async fn https_initialization_retains_the_host_crypto_provider() {
    let mut provider = rustls::crypto::ring::default_provider();
    provider.cipher_suites.truncate(1);
    provider.install_default().expect("isolated host provider");
    let selected = rustls::crypto::CryptoProvider::get_default()
        .unwrap()
        .clone();
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
    let server = jj_store_rpc::server::NativeRpc::new(vec![], Default::default()).unwrap();
    // Invalid identity must return an ordinary configuration error, not panic
    // while selecting between the two compiled cryptographic backends.
    let result = server
        .serve(
            listener,
            Some(tonic::transport::Identity::from_pem("invalid", "invalid")),
            false,
            async {},
        )
        .await;
    assert!(result.is_err());
    assert!(Arc::ptr_eq(
        &selected,
        rustls::crypto::CryptoProvider::get_default().unwrap()
    ));
    assert_eq!(selected.cipher_suites.len(), 1);
}

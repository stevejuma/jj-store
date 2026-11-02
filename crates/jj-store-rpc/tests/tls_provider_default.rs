// A separate integration-test process keeps Rustls's one-time provider isolated.
#[tokio::test]
async fn https_initialization_supports_both_compiled_crypto_backends() {
    assert!(rustls::crypto::CryptoProvider::get_default().is_none());
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
    assert!(rustls::crypto::CryptoProvider::get_default().is_some());
}

//! Tonic uses Rustls's process provider; feature unification can enable two.
pub(crate) fn ensure_provider() {
    if rustls::crypto::CryptoProvider::get_default().is_none() {
        // A concurrent host or RPC initialization may win. Preserve its choice.
        let _ = rustls::crypto::ring::default_provider().install_default();
    }
}

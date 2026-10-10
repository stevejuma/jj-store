//! Dedicated Keychain bundles; OAuth credentials never enter the settings secret journal.
use super::*;
#[cfg(target_os = "macos")]
fn service(root: &Path) -> String {
    use sha2::{Digest, Sha256};
    format!(
        "org.lapismd.jj-store.chatgpt.{}",
        hex::encode(Sha256::digest(root.as_os_str().as_encoded_bytes()))
    )
}
pub(super) fn save(
    root: &Path,
    id: &str,
    tokens: &oauth::Tokens,
    expires: u64,
    generation: u64,
) -> Result<()> {
    if !tokens.valid() {
        return Err(Error::Invalid);
    }
    #[cfg(target_os = "macos")]
    {
        let bytes = zeroize::Zeroizing::new(
            serde_json::to_vec(&json!({"tokens":tokens,"expires":expires,"generation":generation}))
                .map_err(|_| Error::Invalid)?,
        );
        if bytes.len() > 65536 {
            return Err(Error::Limit);
        }
        security_framework::passwords::set_generic_password(&service(root), id, &bytes)
            .map_err(|_| Error::Unavailable)
    }
    #[cfg(not(target_os = "macos"))]
    {
        let _ = (root, id, expires, generation);
        Err(Error::Unavailable)
    }
}
pub(super) fn load(root: &Path, id: &str, generation: u64) -> Result<Cached> {
    #[cfg(target_os = "macos")]
    {
        let bytes = zeroize::Zeroizing::new({
            use security_framework::item::{ItemClass, ItemSearchOptions, SearchResult};
            let mut query = ItemSearchOptions::new();
            query
                .class(ItemClass::generic_password())
                .service(&service(root))
                .account(id)
                .load_data(true)
                .skip_authenticated_items(true);
            match query
                .search()
                .map_err(|_| Error::Unavailable)?
                .into_iter()
                .next()
            {
                Some(SearchResult::Data(bytes)) => bytes,
                _ => return Err(Error::Unavailable),
            }
        });
        if bytes.len() > 65536 {
            return Err(Error::Limit);
        }
        #[derive(Deserialize)]
        struct Stored {
            tokens: oauth::Tokens,
            expires: u64,
            generation: u64,
        }
        let stored: Stored = serde_json::from_slice(&bytes).map_err(|_| Error::Unavailable)?;
        if stored.generation != generation {
            return Err(Error::Stale);
        }
        if !stored.tokens.valid() {
            return Err(Error::Invalid);
        }
        Ok(Cached {
            tokens: stored.tokens,
            expires: stored.expires,
            generation,
        })
    }
    #[cfg(not(target_os = "macos"))]
    {
        let _ = (root, id, generation);
        Err(Error::Unavailable)
    }
}
pub(super) fn clear(root: &Path, id: &str) -> Result<()> {
    #[cfg(target_os = "macos")]
    {
        match security_framework::passwords::delete_generic_password(&service(root), id) {
            Ok(()) => Ok(()),
            Err(e) if e.code() == -25300 => Ok(()),
            Err(_) => Err(Error::Unavailable),
        }
    }
    #[cfg(not(target_os = "macos"))]
    {
        let _ = (root, id);
        Err(Error::Unavailable)
    }
}

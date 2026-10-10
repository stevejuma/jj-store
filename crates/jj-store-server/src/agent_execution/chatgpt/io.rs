//! Fixed production endpoints and vault; deterministic substitutes exist only in tests.
use super::*;
impl Supervisor {
    pub(super) fn oauth_url(&self, path: &str) -> String {
        #[cfg(test)]
        if let Some(io) = self.0.chatgpt.fixture.lock().unwrap().as_ref() {
            return format!("{}{path}", io.origin);
        }
        format!("{}{path}", oauth::ISSUER)
    }
    pub(super) fn resource_url(&self, path: &str) -> String {
        #[cfg(test)]
        if let Some(io) = self.0.chatgpt.fixture.lock().unwrap().as_ref() {
            return format!("{}/v1{path}", io.origin);
        }
        format!("{}{path}", oauth::RESOURCE)
    }
    pub(super) async fn open_chatgpt_browser(&self, url: url::Url) -> Result<()> {
        #[cfg(test)]
        if let Some(io) = self.0.chatgpt.fixture.lock().unwrap().as_ref() {
            io.opened.send(url).map_err(|_| Error::Unavailable)?;
            return Ok(());
        }
        let status = tokio::time::timeout(
            Duration::from_secs(10),
            tokio::process::Command::new("/usr/bin/open")
                .arg(url.as_str())
                .stdin(std::process::Stdio::null())
                .stdout(std::process::Stdio::null())
                .stderr(std::process::Stdio::null())
                .kill_on_drop(true)
                .status(),
        )
        .await
        .map_err(|_| Error::Unavailable)?
        .map_err(|_| Error::Unavailable)?;
        if status.success() {
            Ok(())
        } else {
            Err(Error::Unavailable)
        }
    }
    pub(super) fn save_chatgpt_tokens(
        &self,
        id: &str,
        tokens: &oauth::Tokens,
        expires: u64,
        generation: u64,
    ) -> Result<()> {
        #[cfg(test)]
        if let Some(io) = self.0.chatgpt.fixture.lock().unwrap().as_ref() {
            io.vault.lock().unwrap().insert(
                id.into(),
                Cached {
                    tokens: tokens.clone(),
                    expires,
                    generation,
                },
            );
            return Ok(());
        }
        vault::save(&self.0.resources.root, id, tokens, expires, generation)
    }
    pub(super) fn load_chatgpt_tokens(&self, id: &str, generation: u64) -> Result<Cached> {
        #[cfg(test)]
        if let Some(io) = self.0.chatgpt.fixture.lock().unwrap().as_ref() {
            let vault = io.vault.lock().unwrap();
            let cached = vault.get(id).ok_or(Error::Unavailable)?;
            if cached.generation != generation {
                return Err(Error::Stale);
            }
            return Ok(Cached {
                tokens: cached.tokens.clone(),
                expires: cached.expires,
                generation,
            });
        }
        vault::load(&self.0.resources.root, id, generation)
    }
    pub(super) fn clear_chatgpt_tokens(&self, id: &str) -> Result<()> {
        #[cfg(test)]
        if let Some(io) = self.0.chatgpt.fixture.lock().unwrap().as_ref() {
            io.vault.lock().unwrap().remove(id);
            return Ok(());
        }
        vault::clear(&self.0.resources.root, id)
    }
}

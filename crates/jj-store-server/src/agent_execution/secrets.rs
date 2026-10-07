//! OS credential custody. Only opaque references are journaled.
use super::*;
use settings::{SecretCommand, SecretStatus};

#[cfg(target_os = "macos")]
fn service(root: &Path) -> String {
    use sha2::{Digest, Sha256};
    format!(
        "org.lapismd.jj-store.agents.{}",
        hex::encode(Sha256::digest(root.as_os_str().as_encoded_bytes()))
    )
}
pub(super) fn write(root: &Path, command: SecretCommand) -> Result<SecretStatus> {
    if !label(&command.id, 64)
        || !command
            .id
            .bytes()
            .all(|b| b.is_ascii_alphanumeric() || b == b'-')
        || command
            .value
            .as_ref()
            .is_some_and(|v| v.len() < 8 || v.len() > 4096 || v.contains('\0'))
    {
        return Err(Error::Invalid);
    }
    #[cfg(target_os = "macos")]
    {
        let service = service(root);
        if let Some(value) = command.value {
            let value = zeroize::Zeroizing::new(value);
            security_framework::passwords::set_generic_password(
                &service,
                &command.id,
                value.as_bytes(),
            )
            .map_err(|_| Error::Unavailable)?;
        } else {
            security_framework::passwords::delete_generic_password(&service, &command.id)
                .map_err(|_| Error::Unavailable)?;
        }
        Ok(SecretStatus {
            keychain_supported: true,
        })
    }
    #[cfg(not(target_os = "macos"))]
    {
        let _ = root;
        Err(Error::Unavailable)
    }
}
pub(super) fn get(root: &Path, id: &str) -> Result<String> {
    #[cfg(target_os = "macos")]
    {
        let bytes = zeroize::Zeroizing::new(
            security_framework::passwords::get_generic_password(&service(root), id)
                .map_err(|_| Error::Unavailable)?,
        );
        String::from_utf8(bytes.to_vec()).map_err(|_| Error::Unavailable)
    }
    #[cfg(not(target_os = "macos"))]
    {
        let _ = (root, id);
        Err(Error::Unavailable)
    }
}

#[cfg(target_os = "macos")]
pub(super) fn compute_identity(root: &Path) -> Result<String> {
    let service = service(root);
    match security_framework::passwords::get_generic_password(&service, "compute-identity") {
        Ok(bytes) => {
            let value = String::from_utf8(bytes).map_err(|_| Error::Unavailable)?;
            if value.len() != 64 || hex::decode(&value).is_err() {
                return Err(Error::Unavailable);
            }
            Ok(value)
        }
        Err(error) if error.code() == -25300 => {
            let mut bytes = [0u8; 32];
            getrandom::fill(&mut bytes).map_err(|_| Error::Unavailable)?;
            let key = hex::encode(bytes);
            security_framework::passwords::set_generic_password(
                &service,
                "compute-identity",
                key.as_bytes(),
            )
            .map_err(|_| Error::Unavailable)?;
            Ok(key)
        }
        Err(_) => Err(Error::Unavailable),
    }
}

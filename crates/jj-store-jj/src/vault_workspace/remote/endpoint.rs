use super::*;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum Protocol {
    Https,
    Ssh,
}

pub(super) fn protocol(endpoint: &str) -> Result<Protocol> {
    if endpoint.len() > 4096 || endpoint.chars().any(char::is_control) {
        return Err(Error::Binding);
    }
    // SCP-style SSH paths have a deliberately small, non-executable grammar.
    if !endpoint.contains("://") {
        let (authority, path) = endpoint.split_once(':').ok_or(Error::Binding)?;
        let (user, host) = authority.split_once('@').ok_or(Error::Binding)?;
        if !identity(user)
            || !hostname(host)
            || path.is_empty()
            || path.starts_with('-')
            || !path
                .bytes()
                .all(|b| b.is_ascii_alphanumeric() || b"._-/".contains(&b))
            || path.split('/').any(|part| part == "..")
        {
            return Err(Error::Binding);
        }
        return Ok(Protocol::Ssh);
    }
    let url = Url::parse(endpoint).map_err(|_| Error::Binding)?;
    if url.host_str().is_none()
        || matches!(url.host(), Some(url::Host::Domain(host)) if !hostname(host))
        || url.password().is_some()
        || url.query().is_some()
        || url.fragment().is_some()
        || url.path().trim_matches('/').is_empty()
    {
        return Err(Error::Binding);
    }
    match url.scheme() {
        "https" if url.username().is_empty() => Ok(Protocol::Https),
        "ssh" if url.username().is_empty() || identity(url.username()) => Ok(Protocol::Ssh),
        _ => Err(Error::Binding),
    }
}
fn identity(value: &str) -> bool {
    !value.is_empty()
        && !value.starts_with('-')
        && value
            .bytes()
            .all(|b| b.is_ascii_alphanumeric() || b"-_.".contains(&b))
}
fn hostname(value: &str) -> bool {
    !value.is_empty()
        && !value.starts_with('-')
        && value
            .bytes()
            .all(|b| b.is_ascii_alphanumeric() || b"-._".contains(&b))
}

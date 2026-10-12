use super::*;
pub(super) fn hexadecimal(value: &str) -> bool {
    value
        .bytes()
        .all(|c| c.is_ascii_digit() || (b'a'..=b'f').contains(&c))
}
pub(super) fn profile(details: &mut ProfileDetails) -> Result<()> {
    details.display_name = details.display_name.trim().into();
    if details.display_name.len() > 256
        || details.description.len() > 16 * 1024
        || details.display_name.chars().any(char::is_control)
        || details.description.contains('\0')
    {
        return Err(Error::Invalid);
    }
    details.email = details
        .email
        .take()
        .map(|s| s.trim().to_lowercase())
        .filter(|s| !s.is_empty());
    if details.email.as_ref().is_some_and(|s| {
        s.len() > 254
            || s.chars().any(|c| c.is_control() || c.is_whitespace())
            || s.split('@').count() != 2
            || s.starts_with('@')
            || s.ends_with('@')
    }) {
        return Err(Error::Invalid);
    }
    details.avatar = details
        .avatar
        .take()
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty());
    if let Some(avatar) = &details.avatar {
        if let Some(digest) = avatar.strip_prefix("blob:") {
            if digest.len() != 64 || !hexadecimal(digest) {
                return Err(Error::Invalid);
            }
        } else if (avatar.starts_with("data:") && !avatar.starts_with("data:image/svg+xml,"))
            || !crate::presentation_avatar::valid(avatar)
        {
            return Err(Error::Invalid);
        }
    }
    Ok(())
}
pub(super) fn shortcode(value: &str) -> Result<String> {
    let value = value.trim().trim_matches(':').to_ascii_lowercase();
    if value.is_empty()
        || value.len() > 64
        || !value
            .bytes()
            .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || b"_-".contains(&c))
    {
        return Err(Error::Invalid);
    }
    Ok(value)
}

//! Bounded default templates are frozen into the reviewed submission plan.
use super::*;
use std::io::Read as _;
pub(super) fn body(
    root: &std::path::Path,
    provider: f::Provider,
    fallback: String,
) -> f::Result<String> {
    let paths: &[&str] = match provider {
        f::Provider::Github => &[
            ".github/PULL_REQUEST_TEMPLATE.md",
            ".github/pull_request_template.md",
            "PULL_REQUEST_TEMPLATE.md",
        ],
        f::Provider::Gitlab => &[".gitlab/merge_request_templates/Default.md"],
        f::Provider::Forgejo => &[
            ".forgejo/PULL_REQUEST_TEMPLATE.md",
            ".forgejo/pull_request_template.md",
            ".gitea/pull_request_template.md",
            ".github/PULL_REQUEST_TEMPLATE.md",
        ],
    };
    for relative in paths {
        let path = root.join(relative);
        let metadata = match std::fs::symlink_metadata(&path) {
            Ok(v) => v,
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => continue,
            Err(_) => return Err(f::Error::State),
        };
        if !metadata.file_type().is_file()
            || !path
                .canonicalize()
                .map_err(|_| f::Error::State)?
                .starts_with(root)
        {
            return Err(f::Error::Invalid);
        }
        let mut bytes = vec![];
        std::fs::File::open(path)
            .map_err(|_| f::Error::State)?
            .take(128 * 1024 + 1)
            .read_to_end(&mut bytes)
            .map_err(|_| f::Error::State)?;
        if bytes.len() > 128 * 1024 {
            return Err(f::Error::Limit);
        }
        return String::from_utf8(bytes).map_err(|_| f::Error::Invalid);
    }
    Ok(fallback)
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn default_template_is_bounded_and_does_not_follow_a_file_symlink() {
        let root = tempfile::tempdir().unwrap();
        let root = root.path().canonicalize().unwrap();
        std::fs::create_dir(root.join(".github")).unwrap();
        let path = root.join(".github/PULL_REQUEST_TEMPLATE.md");
        std::fs::write(&path, "Explain validation\n").unwrap();
        assert_eq!(
            body(&root, f::Provider::Github, "commits".into()).unwrap(),
            "Explain validation\n"
        );
        std::fs::write(&path, vec![b'x'; 128 * 1024 + 1]).unwrap();
        assert!(matches!(
            body(&root, f::Provider::Github, "".into()),
            Err(f::Error::Limit)
        ));
        #[cfg(unix)]
        {
            std::fs::remove_file(&path).unwrap();
            std::os::unix::fs::symlink(root.join("outside"), &path).unwrap();
            assert!(body(&root, f::Provider::Github, "".into()).is_err());
        }
    }
}

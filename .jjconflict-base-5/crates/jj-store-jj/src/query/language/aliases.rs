//! Standard JJ 0.45.1 aliases; no server or device user configuration is read.
use jj_lib::revset::RevsetAliasesMap;
pub(super) const ALIASES: &[(&str, &str)] = &[
    (
        "trunk()",
        r#"latest(
  remote_bookmarks(exact:"main", exact:"origin") |
  remote_bookmarks(exact:"master", exact:"origin") |
  remote_bookmarks(exact:"trunk", exact:"origin") |
  remote_bookmarks(exact:"main", exact:"upstream") |
  remote_bookmarks(exact:"master", exact:"upstream") |
  remote_bookmarks(exact:"trunk", exact:"upstream") |
  root()
)
"#,
    ),
    (
        "builtin_log()",
        r#"present(@) | ancestors(immutable_heads().., 2) | trunk()"#,
    ),
    (
        "builtin_immutable_heads()",
        r#"trunk() | tags() | untracked_remote_bookmarks() | untracked_remote_tags()
"#,
    ),
    ("immutable_heads()", r#"builtin_immutable_heads()"#),
    ("immutable()", r#"::(immutable_heads() | root())"#),
    ("mutable()", r#"~immutable()"#),
    ("visible()", r#"::visible_heads()"#),
    ("hidden()", r#"~visible()"#),
];
pub fn standard_aliases() -> RevsetAliasesMap {
    let mut aliases = RevsetAliasesMap::new();
    for (name, value) in ALIASES {
        aliases
            .insert(*name, *value, None)
            .expect("pinned standard alias");
    }
    aliases
}

# Identity and native codecs

The model overview introduces typed identities, portable references and
integrity framing. E-CRATE-READMES-001 preserves JST-ID-001/002/003 identities,
codec fields and canonical bytes; no stored representation changes.

The upstream SimpleBackend cannot be adopted wholesale: copies are unsupported
and submodule serialization panics. Complete these variants in the project
codec. Full native values, not generic Serde JSON, determine persistence.
Preserve invisible intermediate commits needed by evolution. The profile is
verified in M0 before user data is stored; new-repository compatibility is not a
promise of Git-backend import compatibility.


E-CLI-GIT-CRATES-001 keeps crates.io publication to the jj-store command and the crates it links. jj-history-service, jj-vault, tauri-plugin-jj-store, and jj-store-embedded-rustfs stay git-only.

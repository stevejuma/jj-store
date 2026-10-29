# Adapters and Tantivy

The adapter, Tantivy and derived-index crate overviews describe their actual
source/projection/runner and row/index entry points. E-CRATE-READMES-001 leaves
JST-ADAPT-001/003/008 selection, checkpoint and rebuildability contracts
unchanged.

Operation/commit metadata may span retained history; file indexing uses explicit
configured bookmark/workspace scopes. Conflicted scopes are not assigned an
arbitrary winner. Source visibility is checked against the selected JJ view.
Tantivy opstamps are not WAL positions. The active index is never a RustFS
mount. Nostr, Git export, and passive replication remain independent follow-on
adapters.

E-CLI-GIT-CRATES-001 keeps crates.io publication to the jj-store command and the
crates it links. jj-history-service, jj-vault, tauri-plugin-jj-store, and
jj-store-embedded-rustfs stay git-only.

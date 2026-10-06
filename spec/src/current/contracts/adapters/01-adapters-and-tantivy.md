# Adapters and Tantivy

Operation/commit metadata may span retained history; file indexing uses explicit
configured bookmark/workspace scopes. Conflicted scopes are not assigned an
arbitrary winner. Source visibility is checked against the selected JJ view.
Tantivy opstamps are not WAL positions. The active index is never a RustFS
mount. Nostr, Git export, and passive replication remain independent follow-on
adapters.

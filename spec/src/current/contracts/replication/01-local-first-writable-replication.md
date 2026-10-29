# Local-first writable replication

The replication overview introduces endpoint, checkpoint and bounded worker
composition. E-CRATE-READMES-001 preserves JST-REPL-001/002/003 local authority,
origin identity and receiving-coordinator publication semantics.

Every writable checkout uses a complete local `jj-store` and local object
authority. Ordinary native JJ, AgentStore and ReviewStore reads and writes end
at that local authority. A bounded background worker exchanges immutable objects
and accepted publications with one configured authenticated upstream. Remote
health changes replication status; it does not enter the foreground command
path.

E-CLI-GIT-CRATES-001 keeps crates.io publication to the jj-store command and the
crates it links. jj-history-service, jj-vault, tauri-plugin-jj-store, and
jj-store-embedded-rustfs stay git-only.

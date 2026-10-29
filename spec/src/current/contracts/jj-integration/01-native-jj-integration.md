# Native JJ integration

The JJ and RPC overviews identify the pinned adapter, focused companion feature
and NativeClient composition. E-CRATE-READMES-001 preserves JST-JJ-001/002/003
native factory, authorization and operation-pinned read contracts.

M32 owns a standalone Cargo consumer in the JJ integration's test subtree. The
stock-JJ profile constructs a real upstream Git workspace, opens only the
focused companion integration and preserves source operation heads while
collaboration metadata is published and collected. Plain Git and native-only
profiles qualify without upstream JJ in their active dependency graph.
Source-only interfaces retain JST-JJ-001/002/003/004/005 codec, native identity
and operation-pinned read contracts. Production upstream imports remain inside
the owning JJ/CLI adapters; application services are absent from this private
external-host fixture.

E-CLI-GIT-CRATES-001 keeps crates.io publication to the jj-store command and the
crates it links. jj-history-service, jj-vault, tauri-plugin-jj-store, and
jj-store-embedded-rustfs stay git-only.

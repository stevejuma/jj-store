# Interactive agent execution resources

JST-DESK-001 and JST-DESK-003 extend the paired resource set with an optional
regular `jj-acp` executable. Explicit `--acp-executable` during staging, or
trusted `JJ_STORE_ACP_EXECUTABLE`, selects that binary. It is copied alongside
the paired custom `jj`; the host passes both absolute resource paths to its
owned service. Missing ACP leaves execution unavailable without affecting
repository browsing. Installed adapter availability and authentication are
separate from bundled harness availability.

Agent execution shares the server owner lifetime. Closing the desktop parent
pipe closes warm sessions before stopping HTTP/RPC and RustFS. The host allows
forty seconds for the complete service shutdown. Interrupted workspaces and
exact publication intents remain private installation state, and no prompt is
replayed at startup. Full signed app/DMG and WKWebView qualification remain
governed separately by JST-DESK-005.

M25 adds desktop-owned bounded idle-sleep inhibition. The macOS assertion helper
follows authoritative local agent activity, expires after one hour without
activity, and releases on all agents stopping or application shutdown. Polling
alone never renews it. The browser capability marker is presentation-only and
grants no repository access.

Full M25 staging includes `jj-acp`, `jj-store-local-agent`,
`jj-store-agent-tools` and `jj-store-compute`, with source pins, third-party
notices and a checksum manifest verified before launch. Symlink resources,
missing helpers, mismatched source pins, and modified executable/native runtime
bytes refuse startup. `--native-runtime-dir` embeds the pinned inference
artifact directory; otherwise runtime preparation remains an explicit user
download. Weights are separate explicit downloads/imports. The same resource
directory can be staged for a headless server installation; no globally
installed ACP/helper executable is needed. macOS machine identity uses the
installation-scoped Keychain; headless servers provide
`JJ_STORE_COMPUTE_IDENTITY` through operator process configuration.

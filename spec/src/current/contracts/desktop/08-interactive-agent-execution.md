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

# Standalone native deployment (continued 2)

`ArtifactTargetV1` additively accepts `agent_session`, whose value contains the
stable `session_id` and exact `session_snapshot` AgentStore reference. Existing
V1 target encodings and anchor hashes remain byte-for-byte unchanged. The new
variant uses the existing canonical anchor envelope, dependency traversal and
ReviewStore commands; it changes no manifest, WAL, request or receipt format.
OpenAPI and generated TypeScript include the variant, with an actual admitted
HTTP response captured for conformance. Native staging and cold reads reject
missing snapshots, wrong kinds and mismatched stable session identities.
Filesystem-local agent review targets remain unsupported.

The collaboration presentation extension is additive to the version-1 HTTP
schemas: agent session summaries may expose nullable first and last captured
activity milliseconds derived from already selected evidence. Missing, unmatched
or beyond-bound evidence leaves both values absent. Cold Storybook, production
bundle and LocalTrusted reset evidence use the pinned Codex 0.156.1, Cursor
3.22.7 and OpenCode 1.18.30 fixtures without changing their native AgentStore
encodings.

M11 additively registers WorkStore collection, typed command and durable-status
HTTP routes plus matching native RPC V1 methods and custom
`jj project`/`jj task` commands. Generated WorkStore DTOs retain distinct stable
IDs and exact native integer versions. The manifest-3 upgrade preserves
format-1/2 decoding and their canonical bytes, retains the existing origin
ledger, and freezes unsupported M10 origin transfer. WorkStore cursors are
opaque process-registry tokens with a five-minute lifetime; restart and expiry
require explicit refresh. Captured responses exercise actual registered routes,
not fabricated contract examples.

The M11 cold browser gate may reuse an explicit separately built pinned RustFS
runtime, with fresh disposable server state and a new production web bundle.
Native runtime negotiation remains the compatibility check; reuse does not
change supported versions. `JJ_STORE_REAL_TEST` narrows acceptance to a selected
Playwright file while retaining cold startup and cleanup.

For a new disposable sample, complete fixture/search preparation precedes a
fresh server and storage restart. The measured restart must open all persisted
sample data and its existing projection within the unchanged resource gates.
Initial preparation is reported separately; this browser gate does not claim
search cache-loss rebuild performance.

Projects hub browse summaries are additive HTTP contract V1 responses. They do
not change manifest, WAL, native WorkStore objects or CLI/RPC command formats.
Collection and browse continuations share the same resource budget and cannot be
exchanged across query kinds. Browser production builds use Community's public
Vite integration and ES module workers for its existing WASM runtime.

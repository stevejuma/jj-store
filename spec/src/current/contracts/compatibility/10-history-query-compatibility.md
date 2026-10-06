# History query compatibility

The owned query-error union adds a structured `invalid_query` variant; typed
consumers of failed index tasks retain string handling and narrow the diagnostic
variant explicitly. Query metadata and filtered operation graph requests are
additive version-1 contracts; structured query diagnostics are optional in
existing error envelopes. The catalog and standard aliases describe pinned JJ
0.45.1, with explicit filesystem-profile exclusions for content and
cross-operation predicates. No object, manifest, WAL, receipt or companion
persistence format changes. Operation filters remain an owned metadata language;
they do not claim JJ commit revset semantics. Design Core custom-language
support is qualified against source in an ignored temporary dependency setup;
registry-only consumption awaits a release containing that generic extension.

Optional LocalTrusted `default_user_email` is an additive QueryInfo hint; it
neither persists identity nor changes explicit query semantics. Authenticated
serving omits it. Multiline URLs preserve query whitespace within the existing
byte bound while other control characters remain rejected.

Local-management contract samples cover the additive SyncBarrier and
LocalSyncWait browser endpoints, including pending and reached outcomes with
unchanged full issuer/epoch identities. Native persistence formats are
unchanged.

Storage management adds versioned HTTP overview, command and report contracts
and optional local_storage_management capability. The filesystem-only profile
advertises no native maintenance. The private device journal is version 1;
native manifest, checkpoint, WAL, receipt and object encodings are unchanged.

The cached installation source-page profile adds one local HTTP read route to
make 89 paths: 67 native and 22 local. Its selectors are ephemeral runtime
references and its cached pages retain no raw paths, native session keys or
physical proof encodings. All existing repository/native/source/spool/journal
formats remain unchanged; typed limits are 100 page rows within at most 4096
tracked sources. This additive read profile does not qualify explicit import,
automatic startup, other readers or real harness/storage acceptance.

The optional API snapshot publication source field is additive in the version-1
management response. Closed owned records and generated OpenAPI/TypeScript
contracts preserve exact uint64 positions and counts. Existing file-only rows
keep their encoding. Current AgentStore is the only supported storage profile;
this metadata response adds no persistence migration or compatibility reader.
Actual native receipt/body-free HTTP samples qualify controlled OpenCode input
on MemoryStore separately from real harness/RustFS and release qualification.

The private version-1 import journal additionally accepts qualified OpenCode API
bindings with zero byte offset and zero through 131072 logical messages. Empty
history uses the exact initial native message-prefix digest. The original Codex
encoding and snapshot-chain bytes remain unchanged. Other unqualified profiles
are refused; current AgentStore remains the only supported storage profile.

The M18 preview integration regenerates the combined route inventory: 103 paths,
73 outside the installation-local prefix and 30 under `/v1/local/`. This retains
current main's working-file, local review, Storage and Sync controls alongside
all six installation capture routes. Earlier counts describe their historical
slices. Native storage and capture formats remain unchanged by integration.

Capture coverage adds optional bounded configured directory metadata. Generated
HTTP schemas and TypeScript must match the actual admitted response. Omission is
reported as unavailable metadata; qualified new servers report startup-resolved
roots. Transcript source files and native session keys remain absent.

Local capture settings use strict additive version-1 typed HTTP configuration
contracts. Configuration format 2 and AgentStore, source, spool, policy and
import journal formats remain unchanged. The five opt-in product profiles
described below are editable. Existing configuration values outside
agent_capture remain preserved by revision-conditional atomic replacement.
Unsupported startup readers remain separately gated.

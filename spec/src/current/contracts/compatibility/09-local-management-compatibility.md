# Local-management compatibility

Device workspace links use a separate installation-private V1 registry. Their
IDs identify local bindings and are not portable repository identities. The
additive optional HTTP capability is absent on older or non-managing services;
clients must treat absence as unavailable. New routes retain protocol version 1
and do not alter native objects, operation identities, WAL or receipt formats.
Cold acceptance covers the pinned macOS ARM64 native profile and persisted links
across restart; filesystem-only Git serving and other platforms remain gated.

Native folder selection adds an optional `local_folder_picker` capability and a
V1 response with a nullable path. Absence retains manual entry on older services
and unsupported platforms. It changes no repository or workspace-registry
format. The initial OS adapter is macOS; Linux and Windows dialog adapters are
deferred.

Sync management adds an optional local capability and V1 browser report/command
schemas. Existing replication RPC DTO bytes and persisted checkpoints remain
unchanged; clients treat absent capability as unsupported.

Search management adds optional capability and strict V1 intent/report schemas.
Configuration format 2 accepts an optional enabled field on Search entries,
defaulting to true for existing configurations. Native identities and repository
publication formats are unchanged.

Local filesystem configuration is an additive format-2 storage variant. Local
browser DTOs are separate schemas with real Git SHA-1 identities; no native
codec, identifier width, endpoint or companion format is widened. Discovery adds
source profile and availability with native defaults for existing clients.

Local WorkStore browser responses use the companion profile, real metadata
commit and independent WorkStore version. Native WorkStore receipts and
snapshots keep their existing formats; both providers consume one shared browse
projection and preserve the same typed command semantics.

Local AgentStore browser snapshots and freshness events carry an explicit local
profile and committed companion selection. Portable associations expose their
source repository, actual Git IDs and capsule identity. Shared sanitized
activity and transcript fields preserve native presentation while native wire
schemas remain unchanged.

The additive local reconnect endpoint changes only device bindings. Registration
journals bind a proposed companion identity before initialization; all journal
records and result identities are checked on restart. Existing code and
companion object formats remain unchanged.

Physical working-file editing initially qualifies regular UTF-8 files on macOS
ARM64 using descriptor-relative no-follow traversal and atomic private-file
rename. It preserves executable mode and excludes hard-linked/symlink files and
metadata paths. Save changes no JJ IDs, bookmarks, operation heads or companion
refs; recorded-state commands and conversion remain separate gates.

Explicit native repository deletion is qualified separately on macOS ARM64 with
the existing pinned managed embedded RustFS executable. Isolated acceptance
proves epoch refusal, original-session/write fencing, restart restoration,
idempotent completion, catalog replay denial and actual namespace erasure while
preserving a neighbour repository. Socket fault tests prove admitted PUT drain,
foreign-list rejection and refusal to erase after an unknown PUT outcome. An
unknown write requires a cold owned storage restart before the original deletion
can continue. External S3, versioned buckets, independent backups, remote/shared
administration and other platforms remain outside this gate. Local removal
continues to unregister only and never deletes source folders.

Updated servers read legacy V2 discovery catalogs with the deleted-ID set
omitted. Once deletion is admitted into that set, older binaries reject the
unknown field; downgrade serving is not qualified. Native object and publication
formats are unchanged.

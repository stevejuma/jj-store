# Authorized conflict-content uploads

The bounded raw-body endpoint produces an immutable, content-addressed staging
record scoped to repository, principal, exact single-head snapshot, commit, path
and fingerprint. The server computes the native file ID and freezes record V1
with a domain-separated SHA-256 golden vector. No staged record or file
publishes history. Current write permission is required before body consumption,
remote writes and completion; native RPC cannot address staging-record keys.

UploadedFile resolution checks the complete record binding and native content,
then joins the existing atomic path preflight and native rewrite. It supports
binary and empty content, explicit executable mode and optional native copy-term
selection. Structural conflicts can be replaced with ordinary files explicitly;
symlink, submodule and directory selection remain native term actions. Other
conflicts and native descendant/reference/evolution behavior use the same
executor proved in the preceding slice.

Named native/HTTP tests cover a 600,000-byte binary upload, matching native
bytes, no staging publication, ordinary-read exclusion, mixed resolutions,
copy-history retention, other-principal/wrong-path/missing handles, corrupted
records/files, missing native data, lost acknowledgements, restart, revocation
during body consumption, busy/deadline recovery, truncation, body limits and
authorization before polling. Acknowledged mutation retries survive
checkpointing and missing staging records. The selected WAL receipt is the only
mutation acknowledgement.

Limits include separate upload admission, bounded private spools, per-file and
batch caps, exact declared lengths and request/session deadlines. Staging
records remain non-authoritative and cleanup is disabled. Receipt fields and
native codecs are unchanged; only the typed resolution union, new upload ID,
staging format and generated HTTP contracts are extended. No browser features or
visuals change.

Exact commands and before/after failures are retained in E-M5-007 of the
[single ledger](../../spec/src/milestones.md). Real RustFS/TLS qualification and
UI gates retain their separate status. These fixtures do not substitute for
those deployment gates.

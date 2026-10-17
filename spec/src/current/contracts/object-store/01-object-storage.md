# Object storage

E-INJECTABLE-STORES-001 confirms JST-OBJ-001 is unchanged by collapsing the
existing S3 unknown-outcome repository guard into one equivalent conditional.
Unknown writes retain the same uncertainty fence and original request identity.

Version tokens are opaque equality tokens. A timeout is unknown, never proof of
failure. Test concurrent create and CAS, stale tokens, independent endpoints,
restarts, lost responses, range requests, and partial uploads. The initial
supported topology has one writer per repository. Failed CAS conformance blocks
rollout; GET followed by unconditional PUT is not a substitute.

M26's installation directory uses this same bounded ObjectStore port for typed
SHA-256 envelopes and raster blobs. Its installation-scoped manifest is the sole
selector; staged records cannot become public through a guessed digest. Startup
audits selected record closure and media existence; admitted media reads verify
complete bounded bytes. The independent directory selector does not publish JJ
repository state or enable destructive cleanup or multi-writer serving.

## Explicit repository deletion

A confirmed LocalTrusted Admin repository deletion is a narrow exception to
JST-OBJ-004's core V1 prohibition. Ordinary ObjectStore consumers still have no
LIST or DELETE methods, and automatic/destructive GC remains disabled. The
installation's separate S3 administrative path accepts a typed repository ID,
never a caller-supplied key or prefix. It fences all ordinary reads and writes
for that namespace, drains admitted writes even after caller cancellation, then
erases bounded pages under exactly `repos/{repository}/`. It must observe an
empty page before reporting completion. Other repositories, installation
objects, local folders and independent backups are outside this action.

Admission and original request/epoch are durably journalled before fencing or
erasure. Restart restores every fence before repository hosts or workers start.
Partial or unknown erasure remains pending and the same original request resumes
idempotently; GET status never erases. A deleted identity cannot be registered
again. The journal retains only installation/request/epoch identifiers and
completion, never repository content. Versioned S3 buckets are refused before
admission because this profile does not erase historical object versions.

Only the managed embedded profile advertises erasure. Its serving-instance
client retains every unknown PUT outcome until the owned storage process has
restarted. Fencing drains admitted requests but does not treat a timeout as
proof that a remote write finished. Any unknown repository write pauses erasure,
retains the original deletion journal and requires a cold managed storage
restart before continuing. External S3 erasure needs a separately qualified
provider-side write fence and remains unavailable.

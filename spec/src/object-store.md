# Object storage

Version tokens are opaque equality tokens. A timeout is unknown, never proof of
failure. Test concurrent create and CAS, stale tokens, independent endpoints,
restarts, lost responses, range requests, and partial uploads. The initial
supported topology has one writer per repository. Failed CAS conformance blocks
rollout; GET followed by unconditional PUT is not a substitute.

## Requirements

| ID          | Requirement                                                                                                                                                         |
| ----------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| JST-OBJ-001 | Object storage MUST provide get, head, immutable create, conditional compare-exchange, range reads, and explicit Applied, PreconditionFailed, and Unknown outcomes. |
| JST-OBJ-002 | Acknowledged writes MUST satisfy tested cross-route read-after-write and conditional-write semantics for the exact pinned RustFS image and topology.                |
| JST-OBJ-003 | Immutable uploads MUST enforce size and checksum limits and verify existing native content on collisions; large blobs MUST stream with bounded memory.              |
| JST-OBJ-004 | Normal graph reads MUST NOT rely on bucket LIST; destructive delete MUST remain disabled for core v1.                                                               |

## Qualified primitive probe

The initial single-node candidate passed
[seven probes](../../docs/evidence/m0-rustfs-single-node.json). The probe
discards one acknowledgement deliberately; it does not claim to inject a failure
inside RustFS. Production publication, IAM enforcement, streamed upload limits,
and multi-node routes require their own later acceptance evidence.

Verified spools support bounded half-open range readers using local seek and
limited reads. Opening a range never exposes its private path. This primitive
does not establish remote integrity; native callers verify the complete object
before creating a range reader.

A [second cold run](../../docs/evidence/m0-rustfs-cold.json) confirms incomplete
request bodies never become readable objects. This evidence supplements the
first report without replacing its recorded outcomes.

## Streamed transport contract

The owned object contract exposes GET, HEAD, bounded half-open byte ranges and
conditional PUT. It has no LIST, unconditional overwrite or delete method.
Version tokens are opaque strings. Uploads are fully staged in private temporary
files with an enforced byte limit and SHA-256 before network submission; this
bounds memory and prevents length/checksum declarations from diverging from the
streamed bytes. Single PUT is capped at 5 GiB; larger objects are unsupported.

The S3 adapter signs conditional headers and payload digests with SigV4,
disables redirects and retries, and sends the checked SHA-256 checksum. Only an
explicit 412 response establishes PreconditionFailed. Transport failures,
unexpected server responses and successful responses lacking a version token are
Unknown. The caller resolves Unknown through reads and its durable publication
receipt. GET validates response length and range metadata and exposes a bounded
stream. Native object readers additionally verify envelope integrity and native
identity.

HTTP storage endpoints require explicit loopback development configuration;
shared endpoints use HTTPS. Storage credentials are runtime inputs and excluded
from diagnostics. The fault store supports failure before apply, loss of reply
after apply, corrupt reads and missing objects; it is not deployment evidence.

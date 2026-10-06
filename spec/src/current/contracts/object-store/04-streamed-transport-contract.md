# Streamed transport contract

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

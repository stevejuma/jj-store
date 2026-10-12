# Object storage

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

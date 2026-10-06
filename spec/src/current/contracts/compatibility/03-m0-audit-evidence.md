# M0 audit evidence

The pinned reference/license and field audit is recorded in
[upstream lock](../../../../../docs/upstream-lock.md) and
[codec matrix](../../../../../docs/codec-coverage.md). The single-node RustFS
probe uses a disposable volume/container with signed S3 requests, 20 rounds of
16 contenders for both create and CAS, and process-kill recovery. Harness
self-tests run in the normal test/spec lanes; real Docker conformance is an
explicit operator lane.

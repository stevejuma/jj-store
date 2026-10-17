# Capture sanitization boundary

The capture boundary uses released Kingfisher scanner 1.4.0, rules 1.2.1 and
core 1.0.3 in-process under Apache-2.0, as qualified by public distribution.
Scanner default features are disabled, so validation and network calls are not
compiled. Base64 decoding is also disabled. The current policy identity is
`kingfisher-scanner@1.4.0/builtin-medium/v1`. Historical records retain their
original `mongodb-kingfisher@2.7.0/builtin-medium/v1` identity and sanitized
bytes.

Each hook submits one bounded JSON object. The boundary canonicalizes the object
with recursively sorted keys independently of workspace dependency feature
unification, normalizes the canonical workspace path to `$WORKSPACE`, scans
those in-memory bytes with the built-in medium-confidence rules and replaces
merged finding byte ranges with `[REDACTED]`. It reparses the result before
constructing the bounded sanitized payload. Invalid JSON, oversized input or
output, scanner/cache errors and invalid finding ranges produce a fixed omitted
state without returning raw values or secret-bearing diagnostics. The raw hook
payload is never copied to a temporary file or durable state. A user-restricted
compiled-rule cache is warmed during real integration setup and reused by hook
processes.

Prefix-scheduling acceptance prepares that qualified rule cache before its
unchanged publication deadline. Cold scanner initialization and cached child
process latency retain their separate privacy acceptance; the multi-step prefix,
input size, frame bounds and publication assertions are unchanged.

The packaged hook applies this boundary before capture-state creation. It writes
only a validated sanitized payload or fixed `{"status":"omitted"}` object into
the durable spool. Publication recovery remains required before M9.2 capture
support can be claimed.

M33 extracts this exact native policy into the Agent Redaction Rust crate. The
JJ adapter retains bounded domain payload construction and host cache-path
admission. Original sanitized bytes, policies and publication formats are
unchanged; the portable TypeScript complete-block policy is separate.

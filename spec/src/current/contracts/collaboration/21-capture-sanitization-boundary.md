# Capture sanitization boundary

The capture boundary uses MongoDB Kingfisher 2.7.0 in-process at source revision
`63dccba231b6b01990c84319d25f92f49428a431` under Apache-2.0. Scanner default
features are disabled, so validation and network calls are not compiled. Base64
decoding is also disabled. The fixed policy identity is
`mongodb-kingfisher@2.7.0/builtin-medium/v1`.

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

The packaged hook applies this boundary before capture-state creation. It writes
only a validated sanitized payload or fixed `{"status":"omitted"}` object into
the durable spool. Publication recovery remains required before M9.2 capture
support can be claimed.

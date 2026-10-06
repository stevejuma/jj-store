# RPC administration

The native V1 service includes typed repository creation and token list/apply/
status methods. Creation uses the separate operator policy; all token methods
require current repository admin access. Hosted routing binds repository IDs
lazily against the configured storage namespace, without a mutable in-memory
repository catalog or bucket listing. Unknown and unauthorized repository
bindings both deny access. The native object/head surface remains independently
guarded and cannot mutate settings.

Hosted creation reserves the requested installation alias before touching
repository authority. After manifest publication succeeds or an identical retry
recovers it, RPC atomically registers the alias with the initial admin token ID
as a discovery candidate. Committed grant/revoke receipts update that candidate
index, while every repository operation and discovery result still requires
current full-credential authorization against native settings.

Administration requests and responses bound snapshot heads, token counts,
message sizes, deadlines and concurrent work. Credential-bearing protobuf
messages use redacted Debug output; decoded secrets are cleared after parsing.
TLS and explicit loopback transport rules are identical to ordinary native RPC.

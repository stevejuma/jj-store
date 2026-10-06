# Approved device credentials

The invoking device may explicitly select absolute executable `GIT_ASKPASS`,
`SSH_ASKPASS` and `GIT_SSH` paths and an absolute bounded `GIT_SSL_CAINFO` trust
file. The bridge validates these paths, retains SSH agent access and fixes the
SSH argument profile. Shell-valued `GIT_SSH_COMMAND`, Git trace/config injection
and TLS-verification overrides are not forwarded. An explicit
`JJ_STORE_GIT_CREDENTIAL_HELPER` selects `none`, `osxkeychain`, `manager` or
`libsecret`; arbitrary credential-helper strings are refused. The macOS default
uses its OS keychain. Native/server bearer and storage credentials are removed
from transport and helper environments. Device approvals are never serialized
into immutable Git facts. Authentication and trust failures accept no fetch.

Dry-run prints the frozen source operation, destination approval, profile and
per-ref known-OID/known-absent lease, desired Git OID/deletion and complete
native target. These are planning facts, with no native intent or external
write.

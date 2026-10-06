# Conservative anchor placement

`AnchorResolverV1` is a bounded read-only service over an original immutable
anchor and one explicit target. It returns `exact`, `mapped_unchanged`,
`candidate_modified`, `deleted`, `ambiguous`, `unresolved` or `unavailable` with
categorical evidence and at most the caller's bounded candidate count. A
root-pinned `ReviewStoreSnapshot` can load a selected anchor and invoke the same
pure resolver; the call has no publication, WAL or manifest capability.

Code placement accepts exact object identity or lineage whose from/to operation,
commit, path and file identities match both endpoints. It maps whole-file
identity and byte ranges proven unchanged by common prefix or suffix, returns a
modified candidate when the selected range intersects changed bytes, and keeps
multiple paths or repeated candidate ranges ambiguous. A complete supported
lineage with no target path is deleted. Missing lineage, unsupported targets and
exhausted byte budgets are unresolved; missing or redacted retained content is
unavailable.

Agent text maps to a newer block only when the session/block identity,
representation, sanitizer and normalizer versions match, retained block/content
lineage names both immutable versions, and the newer bytes retain the complete
old content as an unchanged prefix. Exact content versions remain exact. The
resolver never uses a stable tool or change ID alone, mutates the original
anchor, publishes a JJ operation, changes workflow state or records a permanent
placement decision.

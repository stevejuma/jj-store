# Initial M18.1 correction semantics

Evidence-only V1 event batches may have zero activity events. Their strong
observation reference and selected source receipt/coverage retain
diagnostic-only captures without inventing a session. Known session aliases may
retain partial coverage; unbound omissions select no session. Existing nonempty
batch bytes are unchanged. Across publications, enrichment loads only the
selected session/tool, retains input/output, attempts and evidence, and records
terminal conflicts in source/session coverage. Original event batches retain
contradictory source observations. Exact bound Thread capture retains either one
real session selection or an explicit diagnostic-only capture gap. A gap link
pins the exact SourceState, empty EventBatch and original Thread binding as
strong dependencies. The source receipt and sanitized batch must agree on
integration, source, batch and sequence interval; the empty batch cannot claim
activity or a session alias. Both roots select atomically under the same
authorization and exact observed Thread/assignment preconditions. No session or
primary assignment is invented. Later valid capture uses its own explicit
binding and retains the earlier gap. Receipt replay, unknown outcomes and prior
unlinked bytes remain unchanged.

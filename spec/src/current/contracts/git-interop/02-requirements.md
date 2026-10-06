# Requirements

| ID          | Requirement                                                                                                                                                                                                           |
| ----------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| JST-GIT-001 | Parsed native Git commands MUST route through the existing CliRunner hook and workspace loader; ordinary backends MUST retain upstream dispatch, with explicit native capability errors before unsupported effects.   |
| JST-GIT-002 | Accepted exact Git representations, mappings, observations and attempts MUST be retained by native immutable storage and manifest CAS; deleting all private Git caches MUST preserve identities and recovery.         |
| JST-GIT-003 | The fixed conversion profile MUST preserve native identities and exact imported Git bytes, original parent OIDs, modes, symlinks, gitlinks and commit tags; unsupported or lossy conversions MUST fail explicitly.    |
| JST-GIT-004 | Fetch MUST verify complete bounded dependency closure and atomically publish native code and observation evidence using native ref merge/tracking semantics without replacing concurrent local work.                  |
| JST-GIT-005 | Push MUST freeze selected refs, durably prepare intent and dependencies before transport, use destination-bound exact leases, and publish only selected Git closure; multi-ref updates MUST require atomic transport. |
| JST-GIT-006 | External uncertainty and local apply status MUST remain distinct; recovery MUST inspect and repair idempotently without blind repush, and native undo MUST retain external outcome evidence.                          |
| JST-GIT-007 | Bridge facts MUST participate in native copy, checkpoints, cold recovery and qualified data-only replication; concurrent observations MUST preserve causal conflicts and replay MUST never execute Git.               |
| JST-GIT-008 | Endpoint bindings and credentials MUST remain device-local outside checkouts; transport MUST be bounded and use approved HTTPS/SSH/local paths without hooks, filters, arbitrary helpers or shell interpolation.      |

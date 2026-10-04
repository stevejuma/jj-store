# Capabilities and support

Availability depends on repository profile, access and qualification. Use
`jj store capabilities` for native CLI support and the browser's actual
capability response for its selected repository. A parser entry is not a promise
that every backend accepts the operation.

| Area                        | Implemented context                                                                          | Important boundary                                                                                        |
| --------------------------- | -------------------------------------------------------------------------------------------- | --------------------------------------------------------------------------------------------------------- |
| Native repository authority | LocalTrusted managed RustFS                                                                  | One serving instance per namespace; no automatic promotion or destructive GC                              |
| Authenticated discovery     | Manual-token repository discovery and independent grants                                     | Broader shared/external-provider release qualification is separate                                        |
| Native browser              | History, files, search, typed mutations and conflict editing                                 | Current permission, exact snapshot and operation-specific limits apply                                    |
| Local Git-backed browser    | Explicit folders, pinned code reads, WorkStore, qualified agent reads and repository reviews | Indexed search, recoverable code commands, portable review anchors and conversion remain separately gated |
| Local working-file Save     | Admitted filesystem working file                                                             | Does not snapshot code or publish companion metadata                                                      |
| Projects and tasks          | Native and local companion providers                                                         | Native WorkStore home origin ledger is frozen pending reducer qualification                               |
| Agent capture               | Current native Codex CLI automatic preview; explicit supported hook paths                    | Exact versions and setup matter; automatic OpenCode/Cursor and Codex Desktop need separate qualification  |
| Reviews                     | Native exact anchors; local repository discussions                                           | Portable local code/agent anchors are not inferred from labels                                            |
| Threads and rich reactions  | Implemented domain, providers and application surfaces                                       | Full convergence, runner and cold release qualification remains incomplete                                |
| Native Git bridge           | Explicit fetch and selected bookmark push                                                    | macOS ARM64, JJ 0.45.1, Git 2.55.0, SHA-1 bookmark profile                                                |
| Replication                 | One authenticated upstream, bounded transfer, durable checkpoints and explicit controls      | Expanded pressure/topology and cold two-instance RustFS qualification remains open                        |
| Storage maintenance         | Bounded integrity and non-destructive checkpoints                                            | Incomplete is not healthy; no qualified backup/restore workflow                                           |
| Desktop                     | Managed macOS development host and local resource composition                                | Full packaged app/DMG distribution qualification is separate                                              |

## Interpreting partial qualification

An implemented feature can be useful within a stated context while a wider
release gate remains unfinished. Keep local-source UI acceptance separate from
registry-resolved foundational-package acceptance. The latter has inherited
pending work and must not be implied by a passing documentation or browser test.

This manual does not introduce deployment topologies, native conversion,
continuous Git mirroring, backup/restore, controller execution, Nostr export or
new harness readers. Where a retained command/configuration schema touches an
unqualified context, the guide says so instead of providing it as the default
setup path.

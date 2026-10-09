# Capabilities and support

Availability depends on repository profile, access and qualification. Use
`jj store capabilities` for native CLI support and the browser's actual
capability response for its selected repository. A parser entry is not a promise
that every backend accepts the operation.

| Area                        | Implemented context                                                                                  | Important boundary                                                                                                          |
| --------------------------- | ---------------------------------------------------------------------------------------------------- | --------------------------------------------------------------------------------------------------------------------------- |
| Native repository authority | LocalTrusted managed RustFS                                                                          | One serving instance per namespace; no automatic promotion or destructive GC                                                |
| Authenticated discovery     | Manual-token repository discovery and independent grants                                             | Broader shared/external-provider release qualification is separate                                                          |
| Native browser              | History, files, search, typed mutations and conflict editing                                         | Current permission, exact snapshot and operation-specific limits apply                                                      |
| Local Git-backed browser    | Explicit folders, pinned code reads, WorkStore, qualified agent reads and retained file reviews      | Indexed search, recoverable code commands, portable agent anchors and broader cold flows remain separately gated            |
| Local working-file Save     | Admitted filesystem working file                                                                     | Does not snapshot code or publish companion metadata                                                                        |
| Projects and tasks          | Native and local companion providers                                                                 | Native WorkStore home origin ledger is frozen pending reducer qualification                                                 |
| Agent capture               | Native Codex CLI, configured Cursor transcripts and OpenCode read-only API; explicit supported hooks | Exact versions, binding setup and native admission apply; Codex Desktop installed-product gate remains open                 |
| Reviews                     | Native exact anchors; local repository discussions and qualified retained file anchors               | Portable agent anchors and the native portable carrier remain gated; exact source evidence is required                      |
| Threads and rich reactions  | Implemented domain, providers and application surfaces                                               | Full convergence, runner and cold release qualification remains incomplete                                                  |
| Native Git bridge           | Explicit fetch and selected bookmark push                                                            | macOS ARM64, JJ 0.45.1, Git 2.55.0, SHA-1 bookmark profile                                                                  |
| Replication                 | One authenticated upstream, bounded transfer, durable checkpoints and explicit controls              | Expanded pressure/topology and cold two-instance RustFS qualification remains open                                          |
| Storage maintenance         | Bounded integrity and non-destructive checkpoints                                                    | Incomplete is not healthy; no qualified backup/restore workflow                                                             |
| Desktop                     | Managed macOS development host, repository terminals and local resource composition                  | Full packaged app/DMG distribution qualification is separate                                                                |
| Repository conversion       | Bounded independent local/native copies with retained history and identity reports                   | Unsupported anchored, signed and conflicted profiles fail preflight; broader local full-flow qualification remains separate |

## Interpreting partial qualification

An implemented feature can be useful within a stated context while a wider
release gate remains unfinished. Keep local-source UI acceptance separate from
registry-resolved foundational-package acceptance. The latter has inherited
pending work and must not be implied by a passing documentation or browser test.

This manual does not introduce additional deployment topologies, continuous Git
mirroring, backup/restore, controller execution, Nostr export or unqualified
harness readers. Where a retained command/configuration schema touches an
unqualified context, the guide says so instead of providing it as the default
setup path.

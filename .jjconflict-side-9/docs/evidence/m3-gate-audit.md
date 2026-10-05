# M3 native CLI/server gate audit

The M3 gate is the native CLI/server vertical slice on the qualified current
Unix host and pinned single-node RustFS topology. The evidence below covers the
approved create → edit → describe → log → restart → op log → evolog flow,
independent workspaces, token access and concurrent operations. It does not
establish first-release completion. Progress remains in the canonical
[ledger](../../spec/src/milestones.md), using the enclosing Jujutsu change.

| Gate                                          | Concrete evidence                                                                                                                                                                                                                                             |
| --------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Native in-process and remote stores           | `native_lifecycle.rs#authenticated_native_workspace_edit_describe_reopen_and_cache_rebuild`; `native_rpc.rs#remote_native_factories_preserve_workspace_and_history_after_reconnect`                                                                           |
| Executable create, edit, describe and history | `executable_native.rs#executable_native_create_edit_describe_log_restart_and_evolution`; [real TLS recovery report](m3-tls-server-rustfs.json)                                                                                                                |
| Physical workspaces and concurrent siblings   | `native_lifecycle.rs#independent_workspaces_preserve_concurrent_operations_and_unsnapshotted_edits`; `native_lifecycle.rs#concurrent_workspace_name_claims_cannot_alias_physical_clients`; executable divergence parity and two-workspace real TLS recovery   |
| Access, revocation and guarded publication    | `native_rpc.rs#rpc_authorization_and_native_validation_precede_immutable_storage`; `native_rpc.rs#rpc_guarded_publication_preserves_receipts_siblings_and_protection`; CLI grant/revoke/retry and operator revocation in real TLS recovery                    |
| Recoverable init/attach                       | `attachment_recovery.rs#accepted_attachment_retries_original_operation_after_lost_ack_and_later_publication`; `attachment_recovery.rs#executable_init_reuses_durable_identity_after_denial_and_lost_creation_ack`; repeat initialization in real TLS recovery |
| TLS, foreground lifecycle and presentation    | `server_executable.rs#cold_tls_server_checks_trust_hostname_and_plaintext`; cold foreground/state-lock and PTY tests; six executable semantic-parity cases in E-M3-013                                                                                        |
| Cache-independent acknowledged history        | Real TLS recovery kills server and RustFS, removes server state and native client index, then reads log/op-log/evolog and original binary bytes while preserving unsnapshotted local edits                                                                    |

Named Rust files are under `crates/jj-store-cli/tests`. All current regular
tests run in `pnpm test` and `pnpm spec:check`; the RustFS case runs only
through `python3 scripts/rustfs-contract.py --run --server`. OS-facility cases
have their separate retained evidence in the ledger. External ignored cases are
never counted as regular-suite passes.

TLS uses a generated localhost certificate, explicit trust and disabled
plaintext development override on an isolated loopback socket. This exercises
the production TLS path; external DNS, automatic certificate provisioning,
reverse proxies, other platforms and multi-node RustFS are unqualified.
`serve-local` process handoff is Unix-only. Shared serving still requires one
configured server instance; the local state lock is not distributed fencing.

M4 retains bounded production read/index sessions, HTTP authorization, generated
contracts, SSE and browser acceptance. M5 retains typed application mutations
and their semantic parity. M6 retains committed-log adapters and Tantivy. M8a
retains packaging/platform qualification, operational limits, restore drills,
performance measurements and combined release acceptance. Requirements spanning
those surfaces remain Planned in the verification table even where M3 supplies
partial evidence. Replication and migration remain Deferred.

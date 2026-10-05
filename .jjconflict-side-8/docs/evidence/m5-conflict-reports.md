# Conflict impact reports and M5 backend audit

ResolveConflicts now records an optional conflict_resolution report containing
exact target before/after commit IDs, sorted selected paths, and the complete
remaining native conflict paths for every rewritten or created result commit.
Empty arrays explicitly describe conflict-free results. The map includes
affected descendants, while untouched history remains outside its scope.
Existing rewrite, bookmark and workspace reports retain their native identities
and semantics.

Report traversal shares the command's authorization, metadata, work, byte,
deadline and output limits. An unreadable or oversized report rejects
preparation and cannot publish a partial success. Preview IDs are provisional;
the selected receipt retains the actual report for restart/status/retry and
checkpoint replay.

The optional field defaults to absent and is omitted when absent. A frozen
pre-extension result field layout inside a V1 stored receipt passes both
canonical reencoding and the service's selected receipt decoder. Absence on a
historical receipt means that no conflict report was recorded. Native
persistence and WAL format versions do not change.

Focused regressions cover target/descendant paths in previews and committed
results, explicit empty remaining lists, HTTP upload preview/apply responses,
checkpoint/restart replay with missing staging records, and output-budget
rejection without publication or a receipt. The original descendant test first
failed because the preview had no report; its exact command passes after the
change. Commands and actual results are recorded in E-M5-008 of the
[single progress ledger](../../spec/src/milestones.md).

The backend audit maps the five canonical mutation requirements to executable
acceptance. These tests use native JJ with isolated/fault-injecting test
storage; real RustFS/TLS deployment qualification remains a separate gate.

| Requirement | Acceptance evidence                                                                                                                                                                                                                                                                                                                                               |
| ----------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| JST-MUT-001 | `native_lifecycle/mutations.rs`: original receipts after restart/later publication, principal-scoped status, revoked/read/cross-repository access, identical-request races, unknown outcomes and stale/conflicting intent. `native_lifecycle/http_mutations.rs`: authorized preview/apply/status transport and response-loss handling.                            |
| JST-MUT-002 | `native_lifecycle/commands.rs`: New/Describe and complete bookmark targets; `abandon.rs`, `resolve.rs` and `reconciliation.rs`: every required native command family, including recursive operation ancestry. HTTP capabilities and generated typed contracts cover these variants.                                                                               |
| JST-MUT-003 | `native_lifecycle/abandon.rs`: protected/root rejection, hidden/no-op handling, merge descendants, retain/restore options, conflicted bookmarks, shared working-copy replacements and retained evolution/history.                                                                                                                                                 |
| JST-MUT-004 | `native_lifecycle/resolve.rs`, `conflict_uploads.rs`, `upload_faults.rs`: ordered native term variants, atomic path/fingerprint validation, scoped/verified uploads, binary/empty/copy/structural content, descendant and reference rewrites, retained conflicts and bounded complete reports.                                                                    |
| JST-MUT-005 | `native_rpc.rs::rpc_guarded_publication_preserves_receipts_siblings_and_protection`: an actual RPC protected rewrite with cleared predecessor metadata is denied and authority is unchanged. Application previews and publications use the same semantic guard. The command enum and HTTP route table expose no arbitrary executable, shell or argument endpoint. |

The report extension changes generated TypeScript API declarations only. Browser
mutation controls, conflict editors and search remain deferred under M4.3;
existing browser tests remain enabled. M6 adapters/search and M8a hardening
remain subsequent work, not implied by this audit.

Validation passes: 301 Rust tests, with six opt-in external deployment tests
explicitly ignored; normal checks, workspace build, formatting and all 13
specification lanes. The latter includes eight cold browser interaction and
accessibility tests and 43 captured HTTP/SSE schema responses. No UI feature or
visual baseline changes were made.

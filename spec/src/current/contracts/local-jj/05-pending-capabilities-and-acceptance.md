# Pending capabilities and acceptance

The landed companion foundation, WorkStore CLI, agent capture/provenance and
repository-wide review provider do not complete the local collaboration profile.
The remaining capabilities below are unimplemented or unqualified. Their status
and command evidence are recorded only in
[the milestone ledger](../../progress/index.md). Review capabilities are staged
independently: bounded committed-metadata-pinned list/detail reads,
repository-anchor commands, code anchors and agent anchors. Repository review
reads/actions reuse the shared reconstruction and projector; CLI and browser
derive the same device actor. Local continuations require an explicit committed
metadata pin and bind repository, registration generation, query and page size.
Native code/agent anchors fail closed until portable validation and retention
qualify. GETs never initialize identity or actor state.

| Milestone | Pending capability                                                                                                   | Acceptance gate                                                                                                                                                                                                                                                                                                                     |
| --------- | -------------------------------------------------------------------------------------------------------------------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| M13.3     | Portable code and agent review anchors, backend-aware review CLI, bounded review detail/pagination and durable retry | Validate exact selected capsules and retained Git closures; preserve stale semantics and original receipts across restart, unknown outcomes and ref races; reject missing or unsupported evidence.                                                                                                                                  |
| M13.3     | Automatic hook code-causality associations and native portable capsule carrier                                       | Associate original sanitized captures with observed explicit code operations without inventing causality; qualify native retained capsule publication and replay. Native providers continue rejecting portable records until that carrier passes.                                                                                   |
| M13.4     | Indexed local code search                                                                                            | Build bounded rebuildable local search over explicit operation and companion selections; preserve independent code access, native search contracts and automatic browser snapshot advancement.                                                                                                                                      |
| M13.4     | Recoverable local code mutations                                                                                     | Durably journal the exact unpublished JJ operation before publishing heads; prove stable unknown/retry outcomes, concurrent sibling preservation, undo/abandon safety and protection of dirty unsnapshotted checkout files.                                                                                                         |
| M13.5     | Bounded durable local/native conversion and identity mapping reports                                                 | Create an independent destination with a fresh operation log, selected code state/ancestry, complete collaboration history and exact evidence archives. Reject unsupported active representations in preflight, verify bounded closures, publish native manifests last, export only to a new folder and leave the source untouched. |
| M13.6     | Cold full-flow qualification and retained native regressions                                                         | Exercise the completed local CLI, filesystem server/UI, recovery and conversion in fresh processes; retain native identity, publication, collaboration and browser regression evidence. Focused provider tests alone do not satisfy this gate.                                                                                      |

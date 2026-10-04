# Glossary

| Term            | Meaning in jj-store                                                                                            |
| --------------- | -------------------------------------------------------------------------------------------------------------- |
| Authority       | The committed selection that determines repository state; native manifest or local companion ref as applicable |
| Manifest        | Native record selecting visible repository roots and publication state                                         |
| CAS             | Compare-and-swap: publish only if the observed authority has not changed                                       |
| WAL             | Durable publication/recovery history; a candidate alone is not selected authority                              |
| Receipt         | Durable result associated with an original command identity                                                    |
| Unknown outcome | Publication may have succeeded, but the caller lacks a definitive result                                       |
| LocalTrusted    | Restricted local installation access through loopback HTTP and owner-only Unix RPC                             |
| Profile         | The repository's storage/identity/operation contract, such as native or Git-backed JJ                          |
| Metadata home   | Repository that owns project/task/discussion metadata                                                          |
| Companion       | Local Git-backed collaboration history selected by an owned ref                                                |
| Snapshot/pin    | Explicit immutable selection used for consistent reads                                                         |
| Operation       | JJ repository-state transition, distinct from a code commit                                                    |
| Change ID       | Stable JJ change identity across relevant rewrites; distinct from commit identity                              |
| Evolution       | The relationships between versions of a change                                                                 |
| Cursor          | Bounded continuation tied to an admitted query and selected snapshot                                           |
| Derived index   | Rebuildable search acceleration, never repository authority                                                    |
| Origin ledger   | Accepted publication facts used by native replication                                                          |
| Frontier        | A finite set of origin positions used to measure transfer progress                                             |
| Checkpoint      | Durable progress or maintenance state; meaning depends on the subsystem                                        |
| Anchor          | Exact artifact identity and position used by a review or association                                           |
| Spool           | Private durable sanitized capture queue retaining original pending intent                                      |
| Capability      | Explicitly admitted functionality for a particular profile and context                                         |

Use [contexts](../getting-started/contexts.md) when choosing a setup and
[snapshots](snapshots.md) when interpreting history or an interrupted command.

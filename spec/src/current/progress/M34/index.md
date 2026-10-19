# M34 automatic vault history and backup

The user approved external state, independent backup rules, edit-burst Git
checkpoints, automatic two-way sync, historical text search, open-vault
lifetime, GitHub followed by Forgejo, and separate backup of existing
repositories. Heavy qualification may be deferred during implementation but must
be recorded. M33's conversation gates and unrelated work remain unchanged.

| ID    | Status      | Depends     | Requirements  | Gate                                                                                                     | Next                                       | Evidence                                                                    |
| ----- | ----------- | ----------- | ------------- | -------------------------------------------------------------------------------------------------------- | ------------------------------------------ | --------------------------------------------------------------------------- |
| M34   | In progress | -           | JST-LOCAL-007 | Functional Notes backup, restore, GitHub/Forgejo sync and history search integrated into main workspaces | Complete the staged implementation         | E-VAULT-BACKUP-001                                                          |
| M34.1 | Partial     | -           | JST-LOCAL-007 | Stable vault identity, independent external JJ state and retained checkpoint commits                     | Implement and exercise external storage    | E-VAULT-BACKUP-001                                                          |
| M34.2 | Partial     | M34.1       | JST-LOCAL-007 | Reusable capture coordinator, watch reconciliation and crash/close recovery                              | Qualify cold native lifecycle later        | E-VAULT-BACKUP-001,E-VAULT-BACKUP-002                                       |
| M34.3 | Partial     | M34.2       | JST-LOCAL-007 | Usable local timeline, guarded restore and visible background status                                     | Qualify cold app restore, then forge setup | E-VAULT-BACKUP-003                                                          |
| M34.4 | Partial     | M34.3       | JST-LOCAL-007 | GitHub sign-in, remote setup and fresh-device restore                                                    | Qualify cold app/provider flow later       | E-VAULT-BACKUP-004,E-VAULT-BACKUP-005,E-VAULT-BACKUP-006,E-VAULT-BACKUP-007 |
| M34.5 | In progress | M34.4       | JST-LOCAL-007 | Two-device automatic pull/merge/push with recoverable conflicts                                          | Qualify cold app/provider gates later      | E-VAULT-BACKUP-008,E-VAULT-BACKUP-009                                       |
| M34.6 | Planned     | M34.3       | JST-LOCAL-007 | Historical text/path/metadata search with exact restore anchors                                          | Add rebuildable local projection           | -                                                                           |
| M34.7 | Planned     | M34.5,M34.6 | JST-LOCAL-007 | Root metadata/adoption and Forgejo sync, followed by integration into main                               | Complete providers and storage migration   | -                                                                           |

Only M34.5 is active. M34.4 setup/restore focused proofs pass; cold app/provider
qualification is deferred. M34.3 local restore proofs pass; cold app
qualification is deferred. M34.2 coordinator/watch/close proofs pass; cold app
qualification is deferred. M34.1 native capture/relink proofs pass; cold app
qualification remains deferred. Source and consumer workspaces live under
`vault-backup/{jj-store,lapis-desktop}`. Original checkouts and review data are
preserved. Integration into main happens after all implementation stages.

## Deferred verification

- Full Rust/UI/catalog suites and platform/release qualification: deferred by
  user instruction until the working implementation is present.
- Native compilation currently needs disk reserve: initial free space is
  approximately 33 GiB after guarded idle-cache cleanup; the initial 23 GiB was,
  below jj-store's 30 GiB build reserve. Inspect managed cleanup eligibility
  before any build; do not touch active targets.
- Focused correctness checks, formatting, and canonical document checks remain
  part of each slice. No visual baselines or remote publication are authorized.

<!-- generated-navigation -->

## Pages

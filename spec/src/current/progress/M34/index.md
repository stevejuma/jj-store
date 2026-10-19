# M34 automatic vault history and backup

The user approved external state, independent backup rules, edit-burst Git
checkpoints, automatic two-way sync, historical text search, open-vault
lifetime, GitHub followed by Forgejo, and separate backup of existing
repositories. Heavy qualification may be deferred during implementation but must
be recorded. M33's conversation gates and unrelated work remain unchanged.

| ID    | Status  | Depends     | Requirements  | Gate                                                                                                     | Next                                   | Evidence                                                                    |
| ----- | ------- | ----------- | ------------- | -------------------------------------------------------------------------------------------------------- | -------------------------------------- | --------------------------------------------------------------------------- |
| M34   | Partial | -           | JST-LOCAL-007 | Functional Notes backup, restore, GitHub/Forgejo sync and history search integrated into main workspaces | Review integrated working version      | E-VAULT-BACKUP-001                                                          |
| M34.1 | Partial | -           | JST-LOCAL-007 | Stable vault identity, independent external JJ state and retained checkpoint commits                     | Deferred cold identity/reopen proof    | E-VAULT-BACKUP-001                                                          |
| M34.2 | Partial | M34.1       | JST-LOCAL-007 | Reusable capture coordinator, watch reconciliation and crash/close recovery                              | Qualify cold native lifecycle later    | E-VAULT-BACKUP-001,E-VAULT-BACKUP-002                                       |
| M34.3 | Partial | M34.2       | JST-LOCAL-007 | Usable local timeline, guarded restore and visible background status                                     | Deferred cold restore qualification    | E-VAULT-BACKUP-003                                                          |
| M34.4 | Partial | M34.3       | JST-LOCAL-007 | GitHub sign-in, remote setup and fresh-device restore                                                    | Qualify cold app/provider flow later   | E-VAULT-BACKUP-004,E-VAULT-BACKUP-005,E-VAULT-BACKUP-006,E-VAULT-BACKUP-007 |
| M34.5 | Partial | M34.4       | JST-LOCAL-007 | Two-device automatic pull/merge/push with recoverable conflicts                                          | Qualify cold app/provider gates later  | E-VAULT-BACKUP-008,E-VAULT-BACKUP-009                                       |
| M34.6 | Partial | M34.3       | JST-LOCAL-007 | Historical text/path/metadata search with exact restore anchors                                          | Deferred cold search/restore proof     | E-VAULT-BACKUP-010                                                          |
| M34.7 | Partial | M34.5,M34.6 | JST-LOCAL-007 | Root metadata/adoption and Forgejo sync, followed by integration into main                               | Deferred native/provider qualification | E-VAULT-BACKUP-011,E-VAULT-BACKUP-012,E-VAULT-BACKUP-013,E-VAULT-BACKUP-014 |

Implementation is integrated; no stage is currently active. M34.6 focused
source/desktop search proofs pass; cold app qualification remains deferred.
M34.5 source/desktop recovery proofs pass; cold app/provider qualification is
deferred. M34.4 setup/restore focused proofs pass; cold app/provider
qualification is deferred. M34.3 local restore proofs pass; cold app
qualification is deferred. M34.2 coordinator/watch/close proofs pass; cold app
qualification is deferred. M34.1 native capture/relink proofs pass; cold app
qualification remains deferred. Source and consumer workspaces live under
`vault-backup/{jj-store,lapis-desktop}`. Original checkouts and review data are
preserved. The completed implementation is integrated into main; deferred
qualification remains separate.

E-VAULT-BACKUP-012 adds recoverable native external/root relocation, archive
recovery, pending-publication path lineage and excluded root control files. Six
new native regressions and ten direct capture/transfer/publication regressions
pass. Desktop storage selection and existing-source adoption remain required
implementation; native app/provider and wider deferred gates stay open.

E-VAULT-BACKUP-013 qualifies explicit original Git-ancestry adoption, durable
prepared/published recovery and stopped-request fencing. Fourteen native source
and ten coordinator/search/restore cases pass, with strict affected Clippy.
Original metadata and saved files remain unchanged. Desktop original read scopes
and source selection are being validated separately; cold app/provider
acceptance and main integration remain open. M34.7 stays Active.

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

E-VAULT-BACKUP-014 integrates the source delivery with current main, preserving
M33 evidence and the unrelated working progress edit. Production code matches
the verified adoption slice exactly. M34/M34.7 are Partial because cold native
app/provider and wider qualification remain deferred. No stage is marked Done.

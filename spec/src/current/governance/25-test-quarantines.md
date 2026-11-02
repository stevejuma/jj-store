# Temporary test quarantines

The user authorized temporary quarantine and disabling of confirmed flaky cases
during the Docker audit begun on 2026-11-01. A quarantine preserves the test
body, assertions, identity and diagnostic guards, and disables only its
individual case. Skipped coverage is not a passing acceptance gate (JST-GOV-003,
JST-GOV-007).

Evidence:
[E-LOCAL-TEST-AUDIT-001](../../history/2026/11/01/E-LOCAL-TEST-AUDIT-001/index.md).

| ID            | Case and canonical register                                                                                                                                                                    |
| ------------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Q-STORY-001   | `StoreWorkspaces / KeyboardSearchAndRepositoryOverview`: [owner, failure, coverage and re-enable gates](09-catalog-validation.md#temporary-story-quarantine-register).                         |
| Q-STORY-002   | `AppearanceOverlay / SearchAndMenuShareTheAppSurfaceAcrossAllGeneratedPalettes`: [owner, failure, coverage and re-enable gates](09-catalog-validation.md#temporary-story-quarantine-register). |
| Q-BROWSER-001 | Comparison header resize geometry: owner, disabled assertions and re-enable gates below.                                                                                                       |
| Q-RUST-002    | Literal-filename pinned history: owner, failure, disabled assertions and re-enable gates below.                                                                                                |

## Q-BROWSER-001: comparison header resize geometry

Owner: jj-store maintainers responsible for workspace-header composition and
browser resize acceptance. Case: `web/tests/browser/comparison-parents.spec.ts`
/ `comparison parent and workspace controls stay ordered after viewport resize`.

The complete app browser lane passed 113 cases and failed the original combined
comparison case at the 390px resize geometry check: comparison right edge
290.609375 exceeded workspace left edge 236.60177612304688. The unchanged exact
combined case then passed in three fresh Docker runs. The cause remains
unproven.

Only the original resize-geometry block is moved into its own reported
`test.skip` case with a quarantine annotation. It retains the 1280px/390px
sequence and exact ordering/bounds assertions. Shared page-local setup is
unchanged. The original comparison workflow stays enabled, retains the same
viewport sequence and all full-ID, clipboard, keyboard, accessibility, selection
and reload assertions. No application styling or behavior is changed.

Disabled coverage is the header controls' measured horizontal ordering and
containment immediately after viewport resize (JST-UI-002, JST-UI-005). Run the
registered
[remote Docker profile](09-catalog-validation.md#reproducing-the-audit-profile)
with `web:test:browser web/tests/browser/comparison-parents.spec.ts` after
removing only the geometry case's `test.skip` modifier in a disposable change.
The owner must identify the cause, retain the exact bounds, and pass three fresh
complete two-case family runs and three complete cold app browser lanes before
re-enabling it. Record receipts in new evidence; this quarantine does not
qualify the original responsive layout gate or complete M30.

## Q-RUST-002: literal-filename pinned history

Owner: jj-store maintainers responsible for `jj-history-service` preparation
admission and pinned file-history reads. Case:
`crates/jj-history-service/tests/file_history/mod.rs` /
`file_history::file_history_keeps_spaces_quotes_and_pattern_characters_literal`
in the `admission` test binary.

The cold complete Rust workspace run on 2026-11-02 passed 2,491 tests and failed
this case: its first pinned graph read returned `Busy` at line 67. Three
unchanged fresh exact crate-profile runs then passed 1/1 each, and the unchanged
complete focused admission family passed 12/12. The full workspace and focused
crate profiles unify dependency features differently; the cause remains
unproven. Investigate preparation completion, readiness publication and
non-queuing path-index admission without assuming a filename-parser defect.

Only this individual case has a documented Rust `#[ignore]`. Its original body
and assertions remain intact. The other file-history merge/deletion cases and
all admission, close/revoke/drain, metadata, operation and blame tests stay
enabled. Disabled coverage compares stock JJ against pinned, paginated history
for a filename containing spaces, quotes and pattern characters, with exact
operation identity, source-byte preservation and revoked-access assertions
(JST-ARCH-007, JST-LOCAL-004). This does not change the runtime contract or
qualify M33.4.

Use the
[remote Docker profile](09-catalog-validation.md#reproducing-the-audit-profile)
with the following lane and arguments to run the original ignored case:

```sh
validation:rust test -p jj-history-service --test admission --locked -- file_history::file_history_keeps_spaces_quotes_and_pattern_characters_literal --exact --ignored
```

The owner must identify and repair the cause while retaining the original
assertions, then pass three fresh exact runs, three complete admission families
and three complete Rust workspace runs with the ignore removed. Include the
workspace's unified feature profile, cold execution, actual counts and Docker
receipts in new evidence before re-enabling this coverage.

## Native journal lock lifetime repair

The audit investigated candidate Q-RUST-001 for
`import_receipt_survives_reopen_and_exact_replay_never_retargets_its_source`.
The complete workspace and family returned `Unavailable` at its first lookup,
while three unchanged fresh exact runs passed. A temporary ignore exposed the
same error in its invalid-input neighbor. A diagnostic captured `WouldBlock` at
lock acquisition while the cold subprocess fixture ran.

A deterministic duplicate-descriptor regression then reproduced the actual
runtime defect: dropping the owning guard only closed its descriptor, leaving an
inherited duplicate holding the same open-file description locked. The private
guard now explicitly unlocks on drop. It retains exclusion until its owned
operation ends, and closing an old duplicate cannot release a later writer's
lock. Nonblocking acquisition, private-file binding, receipt identity, capacity
and unknown-outcome checks remain unchanged (JST-COLLAB-018).

The temporary diagnostic, ignore and fixture-only isolation are removed. The
original combined eight-case journal layout and every assertion remain enabled.
Qualification includes the deterministic regression, both original exact
failures, three fresh complete family runs, affected crate checks and the
complete enabled Docker Rust workspace. Q-RUST-001 is not an active quarantine;
M18 and M30 retain their independent gates.

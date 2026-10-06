---
schema: 1
id: E-M15-001
---

# E-M15-001

<!-- evidence-field:id markdown -->
## Id

E-M15-001
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M15
- M15.1
<!-- /evidence-field -->

<!-- evidence-field:date markdown -->
## Date

2026-10-01
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

focused-domain
<!-- /evidence-field -->

<!-- evidence-field:revision markdown -->
## Revision

enclosing Jujutsu change wnzmnxrz
<!-- /evidence-field -->

<!-- evidence-field:before markdown -->
## Before

Clean isolated threads-reactions at c22556aa. Exact cargo test -p jj-collab --test domain fails because package does not exist. Main active UI work preserved.
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

Rust 1.98.1; JJ 0.45.1; emojis 0.9.0; image 0.25.10; unicode-segmentation 1.13.3; pnpm 10.32.1
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

New canonical collaboration V1 records and bounded radix pages; native manifest/local companion formats unchanged in this slice.
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- Cargo.lock
- Cargo.toml
- crates/jj-collab/Cargo.toml
- crates/jj-collab/src/assets.rs
- crates/jj-collab/src/catalog.rs
- crates/jj-collab/src/codec.rs
- crates/jj-collab/src/commands.rs
- crates/jj-collab/src/graph.rs
- crates/jj-collab/src/ids.rs
- crates/jj-collab/src/lib.rs
- crates/jj-collab/src/objects.rs
- crates/jj-collab/src/reactions.rs
- crates/jj-collab/src/targets.rs
- crates/jj-collab/src/threads.rs
- crates/jj-collab/tests/domain.rs
- scripts/check-boundaries.mjs
- spec-validator.config.mjs
- spec/src/architecture.md
- spec/src/collaboration.md
- spec/src/milestones.md
- spec/src/spec-governance.md
- spec/src/verification.md
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- crates/jj-collab/tests/domain.rs#exact_observed_replacement_retains_auditable_history_and_rejects_stale_clear
- crates/jj-collab/tests/domain.rs#delivery_permutations_preserve_conflicts_and_unseen_concurrent_additions
- crates/jj-collab/tests/domain.rs#same_value_duplicates_count_once_and_same_value_set_is_a_noop
- crates/jj-collab/tests/domain.rs#rich_values_preserve_literal_emoji_and_reject_non_emoji_or_multiple_values
- crates/jj-collab/tests/domain.rs#message_revision_identity_and_agent_content_are_exact_and_witnesses_are_separate
- crates/jj-collab/tests/domain.rs#thread_lifecycle_is_independent_and_restore_returns_idle
- crates/jj-collab/tests/domain.rs#canonical_codec_rejects_unknown_fields_whitespace_and_wrong_kinds
- crates/jj-collab/tests/domain.rs#custom_assets_validate_retained_bytes_and_static_preview
- crates/jj-collab/tests/domain.rs#animation_bounds_are_enforced_and_preview_is_first_frame
- crates/jj-collab/tests/domain.rs#optional_sessions_reference_many_threads_and_primary_reassignment_keeps_history
- crates/jj-collab/tests/domain.rs#human_message_edits_keep_old_revisions_and_archiving_preserves_assignments
- crates/jj-collab/tests/domain.rs#bounded_radix_updates_preserve_prior_snapshots_and_validate_keys
- crates/jj-collab/tests/domain.rs#conflicting_reuse_of_stable_reaction_identity_is_rejected
<!-- /evidence-field -->

<!-- evidence-field:runs runs -->
## Validation

<!-- run -->
### Run 1

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
cargo test -p jj-collab --test domain (before)
```

**Notes**

Package absent.
<!-- /run -->

<!-- run -->
### Run 2

```yaml
result: fail
failures: 2
hasNote: true
noteEncoding: markdown

```

```shell
cargo test -p jj-collab --lib (first compile)
```

**Notes**

Serde tag/action conflict corrected.
<!-- /run -->

<!-- run -->
### Run 3

```yaml
result: fail
failures: 3
hasNote: true
noteEncoding: markdown

```

```shell
cargo clippy -p jj-collab --all-targets --locked -- -D warnings (first)
```

**Notes**

Unused import, collapsible conditional, argument count corrected.
<!-- /run -->

<!-- run -->
### Run 4

```yaml
result: pass
failures: 0
hasNote: true
noteEncoding: markdown

```

```shell
cargo test -p jj-collab --test domain
```

**Notes**

13 focused regressions; stable-ID equivocation also fails closed.
<!-- /run -->

<!-- run -->
### Run 5

```yaml
result: pass
failures: 0
hasNote: false
noteEncoding: markdown

```

```shell
cargo clippy -p jj-collab --all-targets --locked -- -D warnings
```
<!-- /run -->

<!-- run -->
### Run 6

```yaml
result: pass
failures: 0
hasNote: false
noteEncoding: markdown

```

```shell
cargo fmt -p jj-collab
```
<!-- /run -->

<!-- run -->
### Run 7

```yaml
result: pass
failures: 0
hasNote: true
noteEncoding: markdown

```

```shell
pnpm install --offline --frozen-lockfile
```

**Notes**

Dependencies restored from cache.
<!-- /run -->

<!-- run -->
### Run 8

```yaml
result: pass
failures: 0
hasNote: true
noteEncoding: markdown

```

```shell
pnpm check:boundaries
```

**Notes**

16 packages.
<!-- /run -->

<!-- run -->
### Run 9

```yaml
result: pass
failures: 0
hasNote: true
noteEncoding: markdown

```

```shell
pnpm spec:validate
```

**Notes**

4 validators, 21 chapters, 140 requirements.
<!-- /run -->

<!-- run -->
### Run 10

```yaml
result: pass
failures: 0
hasNote: true
noteEncoding: markdown

```

```shell
pnpm spec:first
```

**Notes**

17 protected paths; 4 mapped canonical chapters.
<!-- /run -->

<!-- run -->
### Run 11

```yaml
result: pass
failures: 0
hasNote: true
noteEncoding: markdown

```

```shell
pnpm check:progress
```

**Notes**

111 tasks; same-change evidence.
<!-- /run -->

<!-- run -->
### Run 12

```yaml
result: pass
failures: 0
hasNote: true
noteEncoding: markdown

```

```shell
pnpm exec prettier --check spec/src/collaboration.md spec/src/architecture.md spec/src/spec-governance.md spec/src/milestones.md spec/src/verification.md spec-validator.config.mjs scripts/check-boundaries.mjs
```

**Notes**

Changed governance and documentation formatting.
<!-- /run -->

<!-- run -->
### Run 13

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
pnpm spec:first (initial)
```

**Notes**

New crate Cargo boundary also required architecture.md; added canonical dependency boundary.
<!-- /run -->

<!-- run -->
### Run 14

```yaml
result: fail
failures: 36
hasNote: true
noteEncoding: markdown

```

```shell
pnpm spec:validate (initial)
```

**Notes**

New requirements/milestone rows were outside their Markdown tables; joined tables before final validation.
<!-- /run -->
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages

- [E-M15-001 details 2](details-02.md)

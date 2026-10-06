---
schema: 1
id: E-M21-039
---

# E-M21-039 details 3

<!-- evidence-field:runs runs -->
## Validation

<!-- run -->
### Run 1

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

physical-identity-progress.log: All 142 tasks pass evidence validation.
<!-- /run -->

<!-- run -->
### Run 2

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

physical-identity-spec-first-final.log: Final mapped gate passes six protected files and five canonical chapters.
<!-- /run -->

<!-- run -->
### Run 3

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

physical-identity-spec-validation-final.log: Final validation passes five validators, 23 chapters and 173 requirements.
<!-- /run -->
<!-- /evidence-field -->

<!-- evidence-field:result markdown -->
## Result

pending
<!-- /evidence-field -->

<!-- evidence-field:remaining list -->
## Remaining

- The provider primitive is qualified; server runtime must still resolve trusted
  registration, compare the original physical scope and authenticate original
  actor/ID/digest before journal access. Production private-state placement and
  initialization are not implemented.
- Owned registered discovery/status/pending interface/composition, coordinated
  cancellation, local HTTP/generated contracts, production setup/recovery
  lifecycle and all later approved dirty capture, membership/switching,
  transfer, graph/undo and native/cold qualification remain unfinished.
- No existing executable, installation, service, repository or browser is
  changed. Storybook 6057 is untouched; screenshot comparisons and baseline
  updates remain excluded.
<!-- /evidence-field -->

<!-- evidence-field:next markdown -->
## Next

Paused at the user request after the verified provider identity slice. Discuss implementation before resuming registered runtime/private-state composition and API/UI wiring.
<!-- /evidence-field -->

<!-- evidence-field:summary markdown -->
## Summary

Nine complete current-reader parents, strict owning-package all-target lint/check, 50 governance tests and focused source, documentation, formatting, ownership and canonical gates pass. Historical recovery can obtain stable physical identity independently of current code. Registered runtime and production persistence remain unwired; the approved goal is incomplete and work pauses for discussion. All 520 preceding raw ledger bodies remain byte-identical; no milestone is marked Done.
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- crates/jj-store-jj/tests/working_tree_current/identity.rs#stable_identity_matches_observer_and_survives_current_code_loss_without_writes
- crates/jj-store-jj/tests/working_tree_current/identity.rs#stable_identity_distinguishes_root_and_jj_replacement
- crates/jj-store-jj/tests/working_tree_current/identity.rs#stable_identity_rejects_unsafe_missing_and_unbounded_roots
- crates/jj-store-jj/tests/working_tree_current/identity.rs#stable_identity_rechecks_replacement_and_authorization
<!-- /evidence-field -->


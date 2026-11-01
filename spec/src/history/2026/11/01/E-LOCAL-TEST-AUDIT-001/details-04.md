---
schema: 1
id: E-LOCAL-TEST-AUDIT-001
---

# E-LOCAL-TEST-AUDIT-001 details 4

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
Remote Docker validation:rust test -p jj-store-server --test capture_import_state --locked -- import_receipt_survives_reopen_and_exact_replay_never_retargets_its_source --exact (temporary quarantine)
```

**Notes**

Receipt jj-store-validation-034030b5-8f6a-4e87-b242-bfbbcf815462. Temporary Q-RUST-001 selection reports 0 pass/1 ignored/7 filtered; its assertions are not claimed passing. This provisional ignore is subsequently removed after shared fixture interference is identified.
<!-- /run -->

<!-- run -->
### Run 2

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
Remote Docker validation:rust test -p jj-store-server --test capture_import_state --locked (temporary quarantine)
```

**Notes**

Receipt jj-store-validation-40639e28-d8a8-455a-814b-3e9c498145dc. 6 pass/1 fail/1 ignored. With the first case ignored, invalid_import_inputs_select_nothing_and_foreign_generations_do_not_inherit_receipts instead receives Unavailable where Invalid is expected (original line 120). This exposes shared interference rather than justifying broader quarantine.
<!-- /run -->

<!-- run -->
### Run 3

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
Remote Docker validation:rust test -p jj-store-server --test capture_import_state --locked -- --include-ignored (lock refusal reproduction)
```

**Notes**

Receipt jj-store-validation-8f5f713f-fb2b-44c0-9694-d32b31f634da: 7 pass/1 fail. Temporary diagnostic captures test audit journal lock refusal: WouldBlock in the original first-lookup failure while the cold subprocess fixture runs. The child can briefly inherit a concurrent fixture lock before exec closes its descriptor. The cold fixture is isolated into capture_import_cold with shared support.rs. The temporary ignore and production diagnostic are removed; no lock retries, weakened guards or assertion changes are introduced.
<!-- /run -->

<!-- run -->
### Run 4

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
Remote Docker validation:rust test -p jj-store-server --lib --locked -- capture_journal_files::tests::guard_drop_releases_lock_while_a_duplicate_descriptor_is_alive --exact (before guard release fix)
```

**Notes**

Receipt jj-store-validation-95ee671d-ac92-4003-a248-d231526e8825: deterministic 0 pass/1 fail/222 filtered, in 0.00 seconds. A duplicate descriptor retains the old open-file description after the original guard drops; reacquisition incorrectly returns Unavailable. This proves a runtime guard-release defect rather than merely a flaky assertion. The repair explicitly unlocks on owner Drop. The regression also requires exclusion while the owner lives and verifies that closing the old duplicate cannot unlock a subsequent owner. Original combined capture_import_state tests are restored without ignores or assertion edits.
<!-- /run -->

<!-- run -->
### Run 5

```yaml
result: pass
failures: 0
hasNote: true
noteEncoding: markdown

```

```shell
Remote Docker validation:rust test -p jj-store-server --lib --locked -- capture_journal_files::tests::guard_drop_releases_lock_while_a_duplicate_descriptor_is_alive --exact
```

**Notes**

Receipt jj-store-validation-4cbf4fe0-3c77-4637-89ed-0a68b464393d. Deterministic regression now passes 1/1 with 222 deliberately filtered. Explicit owner unlock permits reacquisition while an old duplicate remains alive; exclusion holds while the original guard lives, and closing the old duplicate cannot unlock a later owner.
<!-- /run -->

<!-- run -->
### Run 6

```yaml
result: pass
failures: 0
hasNote: true
noteEncoding: markdown

```

```shell
Remote Docker interim fixture-isolation probe: validation:rust test -p jj-store-server --locked --test capture_import_state -- import_receipt_survives_reopen_and_exact_replay_never_retargets_its_source --exact
```

**Notes**

Receipt jj-store-validation-01d13fb7-ac6e-4b15-953b-c67391f2fbe3. The original exact failure passes without an ignore annotation. This diagnostic isolation confirms subprocess interference; it is reverted once the deterministic runtime guard-lifetime regression reproduces the inherited-descriptor defect. Final qualification restores the original combined test layout.
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
Remote Docker interim fixture-isolation probe: validation:rust test -p jj-store-server --locked --test capture_import_state -- invalid_import_inputs_select_nothing_and_foreign_generations_do_not_inherit_receipts --exact
```

**Notes**

Receipt jj-store-validation-2639a3b7-2e6c-4a6e-adcb-e7f9d2fd21c3. The original exact failure passes without an ignore annotation. This diagnostic isolation confirms subprocess interference; it is reverted once the deterministic runtime guard-lifetime regression reproduces the inherited-descriptor defect. Final qualification restores the original combined test layout.
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
Remote Docker interim fixture-isolation probe: validation:rust test -p jj-store-server --locked --test capture_import_state --test capture_import_cold --no-fail-fast
```

**Notes**

Receipt jj-store-validation-300ca94f-a8d2-432d-a951-146a60251fde. Both separated binaries pass all eight original assertions with no skips. This diagnostic isolation confirms subprocess interference; it is reverted once the deterministic runtime guard-lifetime regression reproduces the inherited-descriptor defect. Final qualification restores the original combined test layout.
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
Remote Docker interim fixture-isolation probe: validation:rust test -p jj-store-server --locked --test capture_import_state --test capture_import_cold --no-fail-fast
```

**Notes**

Receipt jj-store-validation-0da7051a-cb9a-44f2-b2e4-7cc69005c16a. Both separated binaries pass all eight original assertions with no skips. This diagnostic isolation confirms subprocess interference; it is reverted once the deterministic runtime guard-lifetime regression reproduces the inherited-descriptor defect. Final qualification restores the original combined test layout.
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
Remote Docker interim fixture-isolation probe: validation:rust test -p jj-store-server --locked --test capture_import_state --test capture_import_cold --no-fail-fast
```

**Notes**

Receipt jj-store-validation-931aa5ec-218a-4f17-9fc1-7902e32f015e. Both separated binaries pass all eight original assertions with no skips. This diagnostic isolation confirms subprocess interference; it is reverted once the deterministic runtime guard-lifetime regression reproduces the inherited-descriptor defect. Final qualification restores the original combined test layout.
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
Remote Docker validation:rust test -p jj-store-server --test capture_import_state --locked -- import_receipt_survives_reopen_and_exact_replay_never_retargets_its_source --exact
```

**Notes**

Receipt jj-store-validation-03d3145f-9735-4ab1-9599-7549a6d6eb60. Original first-lookup/record/reopen/source-replay regression passes 1/1 with 7 filtered, in its original combined test binary. No ignore or assertion edit remains.
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
Remote Docker validation:rust test -p jj-store-server --test capture_import_state --locked -- invalid_import_inputs_select_nothing_and_foreign_generations_do_not_inherit_receipts --exact
```

**Notes**

Receipt jj-store-validation-741b19eb-39db-4613-8b62-bb3d198d2333. The second exposed invalid-input regression passes 1/1 with 7 filtered. Invalid/foreign input assertions remain unchanged.
<!-- /run -->

<!-- run -->
### Run 13

```yaml
result: pass
failures: 0
hasNote: true
noteEncoding: markdown

```

```shell
Remote Docker validation:rust test -p jj-store-server --test capture_import_state --locked
```

**Notes**

Receipt jj-store-validation-72e6bade-ef50-40d7-be9f-815b5372d8d5. Fresh original combined family attempt 1: all 8 pass, zero failures or ignores, including concurrent cold process recovery.
<!-- /run -->

<!-- run -->
### Run 14

```yaml
result: pass
failures: 0
hasNote: true
noteEncoding: markdown

```

```shell
Remote Docker validation:rust test -p jj-store-server --test capture_import_state --locked
```

**Notes**

Receipt jj-store-validation-5d5ae7b3-3e29-422f-8127-5e2e64ac4da7. Fresh original combined family attempt 2: all 8 pass, zero failures or ignores, including concurrent cold process recovery.
<!-- /run -->

<!-- run -->
### Run 15

```yaml
result: pass
failures: 0
hasNote: true
noteEncoding: markdown

```

```shell
Remote Docker validation:rust test -p jj-store-server --test capture_import_state --locked
```

**Notes**

Receipt jj-store-validation-af3d9d5d-05be-4bdf-a3c6-6c0a9e36b50c. Fresh original combined family attempt 3: all 8 pass, zero failures or ignores, including concurrent cold process recovery.
<!-- /run -->

<!-- run -->
### Run 16

```yaml
result: pass
failures: 0
hasNote: true
noteEncoding: markdown

```

```shell
Remote Docker validation:rust clippy -p jj-store-jj -p jj-store-rpc -p jj-store -p jj-store-server --all-targets --locked -- -D warnings
```

**Notes**

Receipt jj-store-validation-8752e383-017a-4039-8376-afff8e31e655. Affected JJ, RPC, CLI and server packages plus all their targets pass clippy with -D warnings; direct consumer compilation is included.
<!-- /run -->

<!-- run -->
### Run 17

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
Remote Docker validation:rust test --manifest-path desktop/Cargo.toml --workspace --no-default-features --locked --no-fail-fast (root account)
```

**Notes**

Receipt jj-store-validation-569f9f34-531a-41e7-93d3-fb01a6074719. [108, 1, 2] [pass, fail, ignored]. resolve_shell_rejects_a_file_the_user_cannot_execute assumes an ordinary owner at mode 0010; root legitimately passes access(X_OK) for any execute bit. Implementation uses regular-file metadata and OS access checks correctly. No assertion is changed or quarantined; the full headless inventory is rerun under an isolated ordinary container account.
<!-- /run -->

<!-- run -->
### Run 18

```yaml
result: fail
failures: 2
hasNote: true
noteEncoding: markdown

```

```shell
Remote Docker web:test:browser; docs:test:browser (before mapped documentation review)
```

**Notes**

Receipts jj-store-validation-12f721d3-bc6d-41ea-8458-f08884a12252 and jj-store-validation-f19b384d-ea0f-485d-8da3-023975c8911c. Both web builds stop at Documentation review required: local before browser assertions run. Mapped local/services/configuration pages are reviewed and updated for historical visibility, TLS policy and capture lock release; configuration/env/conversion authority remain unchanged. Fingerprints are refreshed through pnpm docs:review with topic-specific explanations, preserving the gate.
<!-- /run -->
<!-- /evidence-field -->


---
schema: 1
id: E-LOCAL-TEST-AUDIT-001
---

# E-LOCAL-TEST-AUDIT-001 details 5

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
Remote Docker catalog:check 
```

**Notes**

Receipt jj-store-validation-1e5670d6-abd9-4ed2-a728-ac8046e5a112. Catalog svelte-check passes with fail-on-warnings; catalog inventory and story parity checks also pass. Two explicit Storybook quarantines remain discovered.
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
Remote Docker web:check 
```

**Notes**

Receipt jj-store-validation-fcb6c72d-eab6-4dda-a3e4-7d3b2ee777d1. Web Svelte/type checks and client build pass.
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
Remote Docker check:ui-ownership 
```

**Notes**

Receipt jj-store-validation-ae8a954f-7642-4784-bd81-cae2e35c6995. Production/canonical UI ownership audit passes.
<!-- /run -->

<!-- run -->
### Run 4

```yaml
result: fail
failures: 5
hasNote: true
noteEncoding: markdown

```

```shell
Remote Docker ordinary-user headless inventory without a fresh TMPDIR (setup diagnostic)
```

**Notes**

The first ordinary-user rerun reused the stopped root-run container and collided with root-owned fixed-name files in /tmp: three fixtures fail writing and two lifecycle readiness files are stale. Full binary inventory: 104 pass/5 fail/2 existing prerequisite ignores; preliminary exact permission test fails before its assertion. This is not claimed qualifying evidence. A fresh account-owned TMPDIR is supplied for the final full rerun; no original fixture, assertion or implementation changes.
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
Remote Docker managed Cargo headless --no-run --message-format=json inventory; runuser --user jj-store-audit -- env TMPDIR=<fresh-owned-directory> <every discovered test binary>; managed Cargo headless --doc
```

**Notes**

Supplementary receipt headless-unprivileged-final.json retains original run jj-store-validation-569f9f34-531a-41e7-93d3-fb01a6074719 source/image/resources and account UID 1002. First, the exact permission regression passes 1/1. Then all 15 Cargo-discovered test executables pass 109 enabled tests, with zero failures and exactly two pre-existing staged-runtime/performance ignores. Both zero-case documentation test targets also pass. The complete inventory, each executable exit status, build command and isolated temporary root are retained. Compilation/doc tests use the container toolchain; permission-sensitive unit/integration tests run as the ordinary account. No terminal test is disabled.
<!-- /run -->

<!-- run -->
### Run 6

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
Remote Docker web:test:browser (after mapped manual review)
```

**Notes**

Receipt jj-store-validation-d8468c88-9f03-4ec8-83fc-8a007cd77226: 113 pass/1 fail from 114. comparison parent dropdown searches descriptions and sends the full recorded identity fails only its immediate resize geometry at original line 90: comparison right 290.609375 exceeds workspace left 236.60177612304688. All other app browser cases pass. Three unchanged fresh exact combined-case runs are used before classification.
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
Remote Docker web:test:browser web/tests/browser/comparison-parents.spec.ts
```

**Notes**

Receipt jj-store-validation-46a70d63-4798-47f8-b736-e18e6ec32f7b. Unchanged original combined comparison case fresh Docker attempt 1 passes 1/1 with no skips, including both resize bounds and all identity/clipboard/keyboard/a11y assertions.
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
Remote Docker web:test:browser web/tests/browser/comparison-parents.spec.ts
```

**Notes**

Receipt jj-store-validation-0b583356-f8c5-42dc-97cf-7cc328f5d2cf. Unchanged original combined comparison case fresh Docker attempt 2 passes 1/1 with no skips.
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
Remote Docker web:test:browser web/tests/browser/comparison-parents.spec.ts
```

**Notes**

Receipt jj-store-validation-8af47de2-2d31-46cb-a4df-1b2f4a3f8011. Unchanged original combined comparison case fresh Docker attempt 3 passes 1/1 with no skips. Only the geometry block is subsequently quarantined; all other original assertions stay enabled.
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
Remote Docker docs:check 
```

**Notes**

Receipt jj-store-validation-7ecb2bec-ea54-4819-a860-d8a45c66fc02. Mapped local/services/configuration Help topics are reviewed and updated. Strict manual build, 9 Mermaid diagrams in both themes, generated references, and the 144-file/26736495-byte link and anchor closure pass. Normal source-review gates remain enabled.
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
Remote Docker web:test:browser web/tests/browser/comparison-parents.spec.ts
```

**Notes**

Receipt jj-store-validation-1f59c61c-feaa-448f-a8b9-cbe63a973867. Both cases remain discovered. The original full-ID/clipboard/keyboard/accessibility/selection/reload workflow passes; only the extracted unchanged resize bounds case is skipped. JUnit records tests=2, failures=0, skipped=1 and the Q-BROWSER-001 quarantine property.
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
Remote Docker web:check 
```

**Notes**

Receipt jj-store-validation-2e6906cd-9660-4496-988b-d7ab116761ce. Web type/Svelte checks pass after the page-local comparison fixture is shared between the retained workflow and isolated geometry case.
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
Remote Docker docs:test:browser 
```

**Notes**

Receipt jj-store-validation-95c7543d-0afd-446a-b337-3d06458472f0. Complete cold Help browser lane passes all 20 cases across Chromium and WebKit with zero failures or skips; production web/docs build and offline Mermaid rendering also complete.
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
Remote Docker web:test:browser 
```

**Notes**

Receipt jj-store-validation-b28b623b-e747-46e8-b98f-7a04db6c3e6f. Complete cold application browser lane: 114 pass, zero failures, exactly one discovered skipped geometry case Q-BROWSER-001 of 115. The retained original comparison workflow and every other enabled browser case pass. JUnit includes the quarantine annotation; native HTTP/SSE/mutation fixture capture and strict production web/docs build also complete.
<!-- /run -->

<!-- run -->
### Run 15

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
Remote Docker validation:rust test --workspace --locked --no-fail-fast (cold final audit)
```

**Notes**

Receipt jj-store-validation-ae8ad974-e2ac-4b03-8149-7dd0d847dd54. Complete cold workspace totals [2491, 1, 61] [pass, fail, ignored] across 333 summaries. All previous Git/index/TLS/journal failures pass; executable_native 22 pass/3 existing ignores, native_lifecycle 240 pass, native_rpc 28 pass. New sole failure: jj-history-service admission / file_history::file_history_keeps_spaces_quotes_and_pattern_characters_literal returns Busy at its first pinned graph read, tests/file_history/mod.rs:67. The unchanged exact case and complete admission family are rerun before classification. Idle cache retirement retained all original logs and source/image/resource receipts; no cache removal used force.
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
Remote Docker validation:rust test -p jj-history-service --test admission --locked -- file_history::file_history_keeps_spaces_quotes_and_pattern_characters_literal --exact
```

**Notes**

Receipt jj-store-validation-673c7900-ca7f-4557-bf1c-65d255fcde28. Unchanged literal-filename history case fresh exact crate-profile attempt 1: 1 pass, zero failures/ignores, 11 filtered. No source/assertion/ignore change. Contrasts with Busy at the first pinned graph read in the full workspace profile; cause remains unproven.
<!-- /run -->

<!-- run -->
### Run 17

```yaml
result: pass
failures: 0
hasNote: true
noteEncoding: markdown

```

```shell
Remote Docker validation:rust test -p jj-history-service --test admission --locked -- file_history::file_history_keeps_spaces_quotes_and_pattern_characters_literal --exact
```

**Notes**

Receipt jj-store-validation-77bc638d-973a-487b-8998-120850fef569. Unchanged literal-filename history case fresh exact crate-profile attempt 2: 1 pass, zero failures/ignores, 11 filtered. No source/assertion/ignore change.
<!-- /run -->

<!-- run -->
### Run 18

```yaml
result: pass
failures: 0
hasNote: true
noteEncoding: markdown

```

```shell
Remote Docker validation:rust test -p jj-history-service --test admission --locked -- file_history::file_history_keeps_spaces_quotes_and_pattern_characters_literal --exact
```

**Notes**

Receipt jj-store-validation-ed348658-7e6b-4935-afa8-9b32bbaf342e. Unchanged literal-filename history case fresh exact crate-profile attempt 3: 1 pass, zero failures/ignores, 11 filtered. All three fresh focused runs pass against the full workspace failure; cause remains unproven.
<!-- /run -->
<!-- /evidence-field -->


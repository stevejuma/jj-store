---
schema: 1
id: E-ORIGINAL-REMOTE-001
---

# E-ORIGINAL-REMOTE-001 details 2

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
pnpm dev:cargo -- test --locked -p jj-store-jj --no-default-features --features vault-backup --test original_remotes
```

**Notes**

Initial original-module regression fails E0432 before implementation. Initial source compile then exposed three API errors; final local reruns were refused by the managed 30 GiB disk reserve before Rust ran.
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
pnpm validation:rust -- test --locked -p jj-store-jj --no-default-features --features vault-backup --test original_remotes
```

**Notes**

Remote Docker source discovery/admission/stale-endpoint profile: 3 passed, 1 opt-in ignored before save/publication additions.
<!-- /run -->

<!-- run -->
### Run 3

```yaml
result: fail
failures: 2
hasNote: true
noteEncoding: markdown

```

```shell
pnpm test:vault-ssh -- --original
```

**Notes**

First save compilation reports two E0277 errors: refspec String/BStr and sha2 0.11 hex formatting. Both corrected without suppression. Subsequent real SSH run reaches two assertions but fixture public-key authentication fails beneath world-writable Linux /tmp; preflight confirms it.
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
pnpm test:vault-ssh -- --original
```

**Notes**

With a disposable fixture beneath a secure Linux parent and StrictModes retained: 2 real SSH tests pass. Tracked bookmark advances; working-copy commit/unsnapshotted files survive; one selected branch publishes; a dropped accepted response reconciles without another receive-pack. Final divergence/configuration-review hardening is being requalified.
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
pnpm validation:rust -- test --locked -p jj-store-jj --no-default-features --features vault-backup --lib --test original_remotes --test vault_remote --test vault_publication --test vault_transport_validation
```

**Notes**

62 library, 4 original, 2 publication, 4 remote and 3 endpoint cases pass: 75 passed, 5 opt-in ignored. Explicit unreachable remote save proves zero transfer and origin preservation.
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
pnpm validation:rust -- clippy --locked -p jj-store-jj --no-default-features --features vault-backup --lib --test original_remotes --test vault_remote --test vault_publication --test vault_transport_validation -- -D warnings
```

**Notes**

One redundant-closure lint; use the associated function directly. Strict rerun pending.
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
node --test scripts/validation.test.mjs
```

**Notes**

All 12 runner regressions pass; the opt-in SSH lane selects the full native image.
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
pnpm validation:rust -- clippy --locked -p jj-store-jj --no-default-features --features vault-backup --lib --test original_remotes --test vault_remote --test vault_publication --test vault_transport_validation -- -D warnings
```

**Notes**

Strict native source lint passes on the remote Docker worker after the redundant closure repair and final review configuration/divergence checks.
<!-- /run -->

<!-- run -->
### Run 9

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
JJ_STORE_VALIDATION_RUNTIME=native pnpm check:progress
```

**Notes**

The local manual review changed its fingerprint after initial evidence inventory creation. Added that exact coverage file to this fresh uncommitted receipt; no historical record changed.
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
pnpm test:vault-ssh -- --original; pnpm test:vault-ssh
```

**Notes**

Final guarded original profile: 2 real SSH cases pass, including divergent review refusal without receive-pack. Backup runner regression separately passes its real SSH case. Strict host key verification and fixture StrictModes stay enabled.
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
JJ_STORE_VALIDATION_RUNTIME=native JJ_STORE_DOC_REFERENCE=.local/remotes-qualification/docs_reference pnpm docs:check
```

**Notes**

Supported prebuilt-reference path: 9 light/dark Mermaid diagrams, 144 files and 26,742,422-byte documentation closure pass.
<!-- /run -->

<!-- run -->
### Run 12

```yaml
result: fail
failures: 5
hasNote: true
noteEncoding: markdown

```

```shell
JJ_STORE_VALIDATION_RUNTIME=native pnpm spec:validate; pnpm spec:first
```

**Notes**

Initial source status used unsupported In progress in a binary verification table (1); corrected to Planned with explicit remaining host gates. First spec-first found missing architecture/public-distribution mappings (2). Adding their owning clauses then exposed the existing 400-line chapter bound (1) and a stale evidence file inventory (1 invocation). Condensed the validation paragraph and included both exact chapters without changing validator rules.
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
JJ_STORE_VALIDATION_RUNTIME=native pnpm spec:first
```

**Notes**

Mapped final protected source/runner changes to native/local, architecture, public distribution and documentation chapters; 16 protected files and 7 canonical chapters pass.
<!-- /run -->

<!-- run -->
### Run 14

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
JJ_STORE_VALIDATION_RUNTIME=native pnpm spec:validate
```

**Notes**

The fresh evidence grew into a second governed page; add its scoped navigation include so every chapter has exactly one summary entry.
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
JJ_STORE_VALIDATION_RUNTIME=native pnpm spec:validate; pnpm check:progress; pnpm spec:first
```

**Notes**

Final canonical closure passes: 8 validators, 4,659 chapters, 212 requirements; 187 tasks and 1,051 evidence records; 16 protected files and 7 mapped chapters. The slice is rebased onto merged main f53d20ba with an identical base source tree.
<!-- /run -->
<!-- /evidence-field -->


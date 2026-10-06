---
schema: 1
id: E-UI-044
---

# E-UI-044 details 2

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
node --test --test-name-pattern='development browser policy' scripts/dev-demo-state.test.mjs (before implementation)
```

**Notes**

Configuration lacks the explicit annotation setting. .scratch/annotations-before.log.
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
node --test --test-name-pattern='development browser policy' scripts/dev-demo-state.test.mjs
```

**Notes**

1/1 exact regression passes after implementation. .scratch/annotations-after.log.
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
pnpm install --frozen-lockfile
```

**Notes**

Released registry dependencies installed in the isolated workspace; tracked dependency files unchanged. .scratch/annotations-install.log.
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
cp -cR /Users/stevejuma/code/lapismd/jj-store/target /Users/stevejuma/code/lapismd/jj-store-dev-annotations/target
```

**Notes**

APFS copy-on-write build cache reused in the isolated workspace; main artifacts unmodified.
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
rustfmt --edition 2024 crates/jj-store-http/src/assets.rs crates/jj-store-http/tests/assets.rs crates/jj-store-server/src/config.rs crates/jj-store-server/src/config/types.rs crates/jj-store-server/src/installation.rs crates/jj-store-server/src/process.rs crates/jj-store-server/tests/configuration.rs
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
pnpm exec prettier --write scripts/dev-demo.mjs scripts/dev-demo-state.mjs scripts/dev-demo-state.test.mjs scripts/real-browser-acceptance.mjs web/tests/real/browser-annotations.spec.ts spec/src/api.md spec/src/cli.md spec/src/security.md spec/src/spec-governance.md
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
cargo test -p jj-store-http --test assets --locked
```

**Notes**

5/5 static asset tests. .scratch/annotations-assets.log.
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
node --test scripts/dev-demo-state.test.mjs scripts/governance.test.mjs
```

**Notes**

28/28. .scratch/annotations-harness.log.
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
node --test scripts/dev-demo-*.test.mjs scripts/governance.test.mjs
```

**Notes**

37/37. .scratch/annotations-harness-all.log.
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
cargo test -p jj-store-server --locked
```

**Notes**

23 default server tests pass, including 12 configuration tests and 6 local installation tests. Two explicit real-RustFS storage tests ignored, not claimed by this lane. .scratch/annotations-server.log.
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
pnpm spec:validate
```

**Notes**

Pre-integration: 20 chapters, 125 requirements. .scratch/annotations-spec.log.
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
pnpm spec:first
```

**Notes**

12 protected files, 5 chapters. .scratch/annotations-spec-first.log.
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
pnpm check:progress
```

**Notes**

Pre-integration: 96 tasks. .scratch/annotations-progress.log.
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
jj rebase -r rlvvyrkm -o ptqprvlv
```

**Notes**

Main M13 work is committed at e3abe7b4. Rebase before cold acceptance; only the shared ledger conflicts. Resolution retains every exact parent evidence record and task row, then appends this entry.
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
pnpm check:progress (during unresolved ledger rebase)
```

**Notes**

Expected refusal with JJ conflict markers. First helper expected snapshot-style markers but JJ supplied a diff. Recover this entry from pre-rebase 577316f9 and append it to the exact new parent ledger; post-resolution progress check passes with all parent evidence preserved.
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
cargo check -p jj-store-http -p jj-store-server --all-targets --locked
```

**Notes**

Post-integration package check passes. Isolated .scratch/annotations-check.log.
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
cargo test -p jj-store-cli --test server_executable --locked -- --exact cold_tls_server_serves_only_the_configured_browser_bundle
```

**Notes**

1/1 cold TLS/default bundle gate passes; 16 unrelated cases filtered. Isolated .scratch/annotations-cold-server.log.
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
node --test scripts/dev-demo-*.test.mjs scripts/governance.test.mjs
```

**Notes**

Post-integration 37/37 pass. Isolated .scratch/annotations-harness-integrated.log.
<!-- /run -->
<!-- /evidence-field -->


---
schema: 1
id: E-FORGE-CUSTODY-FEATURE-001
---

# E-FORGE-CUSTODY-FEATURE-001

<!-- evidence-field:id markdown -->
## Id

E-FORGE-CUSTODY-FEATURE-001
<!-- /evidence-field -->

<!-- evidence-field:date markdown -->
## Date

2026-11-15
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M31
<!-- /evidence-field -->

<!-- evidence-field:requirements list -->
## Requirements

- JST-FORGE-007
- JST-DOC-002
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

implementation
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- Cargo.toml
- crates/jj-forge/Cargo.toml
- crates/jj-forge/src/accounts/store.rs
- crates/jj-forge/tests/injected_only.rs
- crates/jj-store-service/Cargo.toml
- spec/src/current/contracts/architecture/24-forge-stacks.md
- spec/src/current/progress/M31/index.md
- spec/src/current/verification/forge.md
- spec/src/current/governance/spec-governance.md
- spec/src/current/contracts/mutations/06-durable-command-service.md
- docs/site/guides/git.md
- docs/maintenance/coverage.json
- spec/src/current/contracts/documentation/13-native-remotes-source-qualification.md
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- crates/jj-forge/tests/injected_only.rs#native_custody_is_unavailable_without_the_platform_backend
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
cargo tree --locked -p jj-store-service --no-default-features -i keyring --prefix none
```

**Notes**

Baseline includes one unwanted keyring dependency through jj-forge and jj-store-jj despite disabling service defaults. Hosted Lapis Server run 43684671428 separately fails compiling libdbus-sys without the OS library; no product test ran there.
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
cargo tree --locked -p jj-store-service --no-default-features -i keyring --prefix none
```

**Notes**

After the source fix the command reports package ID keyring did not match any packages (exit 101): the injected-only graph excludes the OS backend. The default service graph still includes keyring.
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
pnpm validation:rust -- test --locked -p jj-forge --no-default-features
```

**Notes**

30 tests pass, including unavailable native-custody get/set/remove. No platform keyring call or real credential is used.
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
pnpm validation:rust -- test --locked -p jj-forge
```

**Notes**

29 tests pass with the unchanged default OS feature. The injected-only regression is intentionally cfg-excluded here.
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
pnpm validation:rust -- clippy --locked -p jj-forge --no-default-features --all-targets -- -D warnings
```

**Notes**

Strict lint passes for the injected profile.
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
pnpm validation:rust -- check --locked -p jj-store-service --no-default-features
```

**Notes**

The complete directly affected injected-custody service graph builds.
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
JJ_STORE_VALIDATION_RUNTIME=native pnpm test:governance
```

**Notes**

All 121 governance cases pass, including feature-family audits.
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
pnpm validation:rust -- check --locked -p jj-store-service
```

**Notes**

Default desktop-compatible service graph still builds with the OS backend enabled.
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
JJ_STORE_VALIDATION_RUNTIME=native pnpm spec:first
```

**Notes**

Initial spec-first requires the mapped mutations chapter for service feature selection. Added an explicit unchanged-command-policy confirmation there; assertions and mapping are retained.
<!-- /run -->

<!-- run -->
### Run 10

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
pnpm evidence:new -- --id E-FORGE-CUSTODY-FEATURE-001 --task M31 --requirement JST-FORGE-007
```

**Notes**

The helper wrote the new record, then global navigation regeneration refused an oversized existing historical index. Restored all 750 unrelated generated navigation files from the clean parent and used the scoped writeEvidence API, registering only the new day links. No historical evidence or navigation remains rewritten.
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
pnpm validation:rust -- clippy --locked -p jj-forge --all-targets -- -D warnings
```

**Notes**

Default OS-enabled profile passes strict lint.
<!-- /run -->

<!-- run -->
### Run 12

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
JJ_STORE_VALIDATION_RUNTIME=native JJ_STORE_DOC_REFERENCE=.local/remotes-qualification/docs_reference pnpm docs:check
```

**Notes**

The first documentation run requires forge-stacks source review. Updated the embedding manual and refreshed only that governed fingerprint; final closure is rerun below.
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
JJ_STORE_VALIDATION_RUNTIME=native pnpm spec:validate
JJ_STORE_VALIDATION_RUNTIME=native pnpm spec:first
JJ_STORE_VALIDATION_RUNTIME=native pnpm check:progress
JJ_STORE_VALIDATION_RUNTIME=native JJ_STORE_DOC_REFERENCE=.local/remotes-qualification/docs_reference pnpm docs:check
cargo fmt --all --check
pnpm exec prettier --check <changed authored Markdown and JSON>
```

**Notes**

Final canonical, progress, documentation closure and formatting checks are recorded with the enclosing source slice; no product or UI acceptance is inferred.
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

Evidence grew into a continuation page; validation correctly required its unique summary entry. Added scoped evidence navigation and the include in this day only. Final validation rerun follows.
<!-- /run -->
<!-- /evidence-field -->

<!-- evidence-field:result markdown -->
## Result

pass
<!-- /evidence-field -->

<!-- evidence-field:remaining list -->
## Remaining

[]
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

Rust/Cargo 1.99.0; Deno-free spec-validator 0.2.0; Node 24.10.0. Isolated revision-labelled remote Docker validation.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

Additive default-enabled native-secrets Cargo feature. Public account/store interfaces and credential formats remain unchanged.
<!-- /evidence-field -->


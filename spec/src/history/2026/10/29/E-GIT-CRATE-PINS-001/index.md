---
schema: 1
id: E-GIT-CRATE-PINS-001
---

# E-GIT-CRATE-PINS-001

<!-- evidence-field:id markdown -->
## Id

E-GIT-CRATE-PINS-001
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M30
<!-- /evidence-field -->

<!-- evidence-field:requirements list -->
## Requirements

- JST-ARCH-005
- JST-ARCH-007
- JST-GOV-002
- JST-DESK-001
<!-- /evidence-field -->

<!-- evidence-field:date markdown -->
## Date

2026-10-29
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

verification
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- Cargo.lock
- Cargo.toml
- desktop/Cargo.lock
- desktop/Cargo.toml
- scripts/ci.test.mjs
- spec/src/current/contracts/architecture/21-local-compute-resources.md
- spec/src/current/contracts/architecture/23-public-distribution.md
- spec/src/current/contracts/desktop/08-interactive-agent-execution.md
- spec/src/current/governance/23-shared-ai-clients.md
- spec/src/current/progress/M30/index.md
- spec/src/history/2026/10/29/E-GIT-CRATE-PINS-001/index.md
- spec/src/history/2026/10/29/E-GIT-CRATE-PINS-001/SUMMARY.nav.md
- spec/src/history/2026/10/29/index.md
- spec/src/history/2026/10/29/SUMMARY.nav.md
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- scripts/ci.test.mjs#CI pins the shared local-source package and rejects mutable revisions
<!-- /evidence-field -->

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
cargo fetch --locked
cargo fetch --locked --manifest-path desktop/Cargo.toml
```

**Notes**

Locked fetch succeeded for the root workspace and the desktop workspace. The
root lock records lapis-ai-capture, lapis-ai-runtime, and lapis-ai-config
0.2.0 from tag v0.2.0 at 9a98293c, and lapis-agent-redaction 0.1.0 from tag
lapis-agent-redaction@0.1.0 at 02a6b492. Desktop records the same runtime pin.
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
cargo check --locked -p jj-store-capture -p jj-store-server --target-dir /Users/stevejuma/code/lapismd/jj-store/target
```

**Notes**

jj-store-capture and jj-store-server finished the dev profile in 1m 07s. No
0.2.0 call site changes were required.
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
cargo check --locked --manifest-path desktop/Cargo.toml --target-dir /Users/stevejuma/code/lapismd/jj-store/desktop/target
```

**Notes**

jj-store-desktop finished the dev profile in 6.40s against lapis-ai-runtime
0.2.0. No desktop call site changes were required. Desktop is its own Cargo
workspace and is excluded from the root members list.
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
node --test --test-name-pattern 'CI pins the shared local-source package' scripts/ci.test.mjs
```

**Notes**

The manifest assertion matches the lapis-ai-runtime git tag v0.2.0 line. 1
passed, 0 failed.
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
JJ_STORE_VALIDATION_RUNTIME=native pnpm check:progress
JJ_STORE_VALIDATION_RUNTIME=native pnpm spec:first
```

**Notes**

Progress passed with 186 tasks and 979 evidence records. Spec-first passed
with 5 protected files and 4 canonical chapters. The wider product suite was
not run.
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

lapis-ai git tag v0.2.0 (9a98293c) crates 0.2.0; lapis-agent-redaction git tag
lapis-agent-redaction@0.1.0 (02a6b492) crate 0.1.0. @lapismd/mira stays ~0.1.1.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

No stored format, DTO, or public API schema change. Root and desktop Cargo
locks now select the published git tags instead of sibling path crates.
<!-- /evidence-field -->

<!-- evidence-field:scope markdown -->
## Scope

Replace the four external Cargo path dependencies with published git tags and
refresh both lockfiles. In-repo path crates, store-consumer fixture paths, and
the Mira pnpm range stay as they are. Helper staging still uses the recorded
sibling source checkout.
<!-- /evidence-field -->

<!-- evidence-field:next markdown -->
## Next

Open the pin pull request. Leave the Mira ~0.1.2 lock refresh until that
version is visible on npm. Full release qualification stays with M30.
<!-- /evidence-field -->

<!-- evidence-field:contract_review yaml -->
## Contract review

```yaml
status: changed
reason: JST-ARCH-005, JST-ARCH-007, and JST-DESK-001 now consume lapis-ai
  crates from git tag v0.2.0 and lapis-agent-redaction from git tag
  lapis-agent-redaction@0.1.0. Local compute, shared AI clients, public
  distribution, and desktop resource text name those tags. Helper staging
  still uses the sibling source checkout.

```
<!-- /evidence-field -->

<!-- evidence-field:milestone_review yaml -->
## Milestone review

```yaml
status: unchanged
reason: M30 stays In progress. This pin does not complete Cargo, native
  bundle, Pages, or GHCR release qualification. M25 and M33 statuses and
  gates stay unchanged.

```
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages



---
schema: 1
id: E-UI-050
---

# E-UI-050 details 3

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
/tmp/jj-store-cli-install-review/jj-store version --format=json; /tmp/jj-store-cli-install-review/jj-store-jj --version
```

**Notes**

Server version succeeded; custom JJ exposes version as a subcommand rather than --version.
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
/tmp/jj-store-cli-install-review/jj-store-jj version --format=json
```

**Notes**

Upstream version does not support format option.
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
/tmp/jj-store-cli-install-review/jj-store-jj version
```

**Notes**

Installed custom tool executes.
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
node --test --test-name-pattern=agent seed installs scripts/dev-demo-collaboration.test.mjs against parent dev-demo-agents.mjs
```

**Notes**

New regression against parent proves source-cwd escape; restored current module immediately. Initial cold run transiently installed source hooks; teardown restored the source checkout.
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
node --test scripts/dev-demo-collaboration.test.mjs
```

**Notes**

4 focused tests pass with explicit fixture cwd on setup, hooks and remove.
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
node --test scripts/dev-demo-repository.test.mjs scripts/dev-demo-state.test.mjs scripts/dev-demo-work.test.mjs scripts/dev-demo-profiles.test.mjs scripts/dev-demo-folder.test.mjs scripts/dev-demo-collaboration.test.mjs scripts/install-cli.test.mjs
```

**Notes**

Final complete directly affected script suite: 19 pass, zero failures.
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
JJ_STORE_DEV_ROOT=/tmp/jj-store-three-profiles-cold JJ_STORE_DEV_PORT=7461 JJ_STORE_DEV_RUSTFS_RUNTIME=$PWD/runtime/embedded-rustfs/target/debug/jj-store-rustfs DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm dev:restart
```

**Notes**

Fresh isolated native installation, UI and debug binary cold builds. Real embedded RustFS; three genuine distinct profiles seeded; native-only checkout removed. Existing sanitizer cache compilation made initial seed slow; no contract weakened.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm dev:restart
```

**Notes**

Owned interactive upgrade preserves native repository 1bdb89c409c1b891ac7f180c01a265d5, seeds server-only 817cfb949001fb0c693298a628c81e3a and local Git 61c5a6c6e2dfb74cd9d636dee2138b41; original three projects each have one distinct active link; no reset or global installation.
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
JJ_STORE_DEV_ROOT=$PWD/.local/dev-demo node scripts/dev-demo-profiles.acceptance.mjs
```

**Notes**

Acceptance adapter initially omitted Sec-Fetch headers and was correctly denied. Add existing same-origin request headers; security policy unchanged.
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
JJ_STORE_DEV_ROOT=$PWD/.local/dev-demo node scripts/dev-demo-profiles.acceptance.mjs
```

**Notes**

Read-only real service qualification: three projects, nine task targets, only two native discovery entries, ready native folder binding, empty server-only links, no temporary checkout, current Git companion identity.
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
JJ_STORE_DEV_ROOT=/tmp/jj-store-three-profiles-cold node scripts/dev-demo-profiles.acceptance.mjs
```

**Notes**

Both before and after a second cold service restart pass the same real-service checks.
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
JJ_STORE_DEV_ROOT=/tmp/jj-store-three-profiles-cold JJ_STORE_DEV_PORT=7461 JJ_STORE_DEV_RUSTFS_RUNTIME=$PWD/runtime/embedded-rustfs/target/debug/jj-store-rustfs DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm dev:restart
```

**Notes**

Final wrapper in fresh processes reuses fixture, links and identities; no new WorkStore publication.
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
node --input-type=module
```

**Notes**

assert.deepEqual compares the complete before/after acceptance JSON: identities and exact native code/WorkStore snapshots retained; no read or restart-induced publications.
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
node .scratch/agent-seed-cwd-acceptance.mjs
```

**Notes**

Final agent seed executed in the isolated real Git-backed JJ fixture. Explicitly reused the isolated compiled Kingfisher cache; 37 setup/capture/remove subprocesses kept fixture cwd; no hook configuration remained in fixture or source checkout.
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
cua_repl: built main demo reload, three linked projects, disclosure flags/reload and active-child collapse/reload; open server-only Overview; return to Projects
```

**Notes**

Expanded projects and collapsed repository section restore after browser reload; explicit active-child collapse restores false. Server-only main and README load without a physical folder. Saved .scratch/three-profiles-projects.jpg; user-owned tab unchanged. An initial heading locator omitted the Markdown collapse-section accessible prefix; AX state confirmed correct content.
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
pnpm exec prettier --check (all changed source, test, package and canonical files)
```

**Notes**

Final changed-file formatting passes after redundant story option formatting.
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
pnpm check:architecture
```

**Notes**

Final 457-module source audit passes.
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
pnpm check:boundaries
```

**Notes**

15 package boundaries pass.
<!-- /run -->
<!-- /evidence-field -->


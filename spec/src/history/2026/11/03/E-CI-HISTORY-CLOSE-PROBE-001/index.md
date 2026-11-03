---
schema: 1
id: E-CI-HISTORY-CLOSE-PROBE-001
---

# E-CI-HISTORY-CLOSE-PROBE-001

<!-- evidence-field:id markdown -->
## Id

E-CI-HISTORY-CLOSE-PROBE-001
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M33.4
- M30
<!-- /evidence-field -->

<!-- evidence-field:requirements list -->
## Requirements

- JST-ARCH-007
- JST-ARCH-005
- JST-GOV-003
- JST-GOV-007
<!-- /evidence-field -->

<!-- evidence-field:date markdown -->
## Date

2026-11-03
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

verification
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- crates/jj-history-service/tests/blame/mod.rs
- spec/src/history/2026/11/03/E-CI-HISTORY-CLOSE-PROBE-001/index.md
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- crates/jj-history-service/tests/blame/mod.rs#blame_close_waits_for_the_owned_read_to_exit
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
gh workflow run ci-heavy.yml --ref main -f ref=201be1b237e9de61b090e19714250c4f31bc111f
gh api --allow-escape-sequences repos/stevejuma/jj-store/actions/jobs/125946741170/logs
```

**Notes**

Complete qualification run 41642914060 failed standard Rust shard 1/4 because
`blame_close_waits_for_the_owned_read_to_exit` timed out at its five-second
admission-withdrawal probe. The owned blame read was still deliberately blocked.
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
npx --yes pnpm@10.32.1 dev:disk
```

**Notes**

The registered workspace targets used 16.3 GiB with 1,582.2 GiB free, within
the 40 GiB target, 80 GiB total and 30 GiB free-space policy.
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
npx --yes pnpm@10.32.1 dev:cargo -- test -p jj-history-service --test admission --locked -- blame::blame_close_waits_for_the_owned_read_to_exit --exact
JJ_STORE_TEST_STOCK_JJ=/tmp/jj-store-stock-jj.KIPdQq/jj npx --yes pnpm@10.32.1 dev:cargo -- test -p jj-history-service --test admission --locked -- blame::blame_close_waits_for_the_owned_read_to_exit --exact
JJ_STORE_TEST_STOCK_JJ="$install_dir/jj" npx --yes pnpm@10.32.1 dev:cargo -- test -p jj-history-service --test admission --locked -- blame::blame_close_waits_for_the_owned_read_to_exit --exact
```

**Notes**

The first local invocation found the unconfigured mise Cargo shim. The next two
setup attempts named the temporary installer root rather than its verified
`jj-store-ci-tools/jj` member and failed before the test body with `NotFound`.
No product or assertion failure occurred in these three harness attempts.
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
RUNNER_TEMP="$install_dir" GITHUB_PATH="$install_dir/github-path" node scripts/ci-setup-jj.mjs
PATH="$PWD/.local/cargo/bin:$PATH" CARGO_HOME="$PWD/.local/cargo" RUSTUP_HOME="$PWD/.local/rustup" JJ_STORE_TEST_STOCK_JJ="$install_dir/jj-store-ci-tools/jj" npx --yes pnpm@10.32.1 dev:cargo -- test -p jj-history-service --test admission --locked -- blame::blame_close_waits_for_the_owned_read_to_exit --exact
```

**Notes**

The exact repaired case passed 1/1 with checksum-verified upstream JJ 0.45.1.
Each observation attempt is now bounded independently, so a probe that entered
before handle removal is canceled and retried instead of occupying the complete
five-second withdrawal deadline.
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
PATH="$PWD/.local/cargo/bin:$PATH" CARGO_HOME="$PWD/.local/cargo" RUSTUP_HOME="$PWD/.local/rustup" npx --yes pnpm@10.32.1 dev:cargo -- check -p jj-history-service --tests --locked
```

**Notes**

The affected package and its test targets passed `cargo check`.
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
JJ_STORE_TEST_STOCK_JJ="$install_dir/jj-store-ci-tools/jj" npx --yes pnpm@10.32.1 dev:cargo -- test -p jj-history-service --test admission --locked
```

**Notes**

The first complete 12-case admission-family run passed the repaired blame case
but returned an unrelated preparation `Busy` from
`revocation_preserves_other_windows_and_failed_preparation_is_observable`.
The existing Q-RUST-002 literal-path case remained intentionally ignored.
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
JJ_STORE_TEST_STOCK_JJ="$install_dir/jj-store-ci-tools/jj" npx --yes pnpm@10.32.1 dev:cargo -- test -p jj-history-service --test admission --locked -- revocation_preserves_other_windows_and_failed_preparation_is_observable --exact
JJ_STORE_TEST_STOCK_JJ="$install_dir/jj-store-ci-tools/jj" npx --yes pnpm@10.32.1 dev:cargo -- test -p jj-history-service --test admission --locked
```

**Notes**

The unrelated case then passed exactly 1/1. A fresh complete admission family
passed 11 enabled cases with zero failures; Q-RUST-002 was the one documented
ignore. Wider Rust, UI and complete qualification remain remote gates.
<!-- /run -->

<!-- run -->
### Run 8

```yaml
result: fail
failures: 2
hasNote: true
noteEncoding: markdown
```

```shell
npx --yes pnpm@10.32.1 exec prettier --check crates/jj-history-service/tests/blame/mod.rs spec/src/history/2026/11/03/E-CI-HISTORY-CLOSE-PROBE-001/index.md spec/src/history/2026/11/03/index.md spec/src/history/2026/11/03/SUMMARY.nav.md
node node_modules/@lapismd/spec-validator/dist/cli.js first --base @- --head @ --json
```

**Notes**

Cargo formatting passed before Prettier correctly rejected the Rust file as an
unsupported parser input. The first spec-first pass rejected the umbrella test
binary reference and a nonempty Remaining field; the evidence now names the
actual protected module and scopes the remaining full run to Next.
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
npx --yes pnpm@10.32.1 dev:cargo -- fmt --check -p jj-history-service
npx --yes pnpm@10.32.1 exec prettier --check spec/src/history/2026/11/03/E-CI-HISTORY-CLOSE-PROBE-001/index.md spec/src/history/2026/11/03/index.md spec/src/history/2026/11/03/SUMMARY.nav.md
node node_modules/@lapismd/spec-validator/dist/cli.js validate --only governance,summary,fileLimits,verification --json
node node_modules/@lapismd/spec-validator/dist/cli.js first --base @- --head @ --json
node scripts/check-progress.mjs
```

**Notes**

Rust and Markdown formatting passed. Four selected specification validators
checked 4,601 chapters and 211 requirements, spec-first accepted the protected
test/evidence pair, and progress accepted 186 tasks and 1,023 evidence records.
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

Node.js 26.8.2; pnpm 10.32.1; Rust/Cargo 1.99.0; upstream JJ
0.45.1-7c41cdeb16b6b321c64e789a966b6adf723816a5.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

No production source, API, wire, cache or persisted format changed.
<!-- /evidence-field -->

<!-- evidence-field:scope markdown -->
## Scope

History-service admission acceptance only. The close implementation, owned read
lock, cancellation, caller/vault/generation admission and blame behavior are
unchanged.
<!-- /evidence-field -->

<!-- evidence-field:next markdown -->
## Next

Land the focused acceptance repair, then rerun complete integrated
qualification on the merged stack.
<!-- /evidence-field -->

<!-- evidence-field:contract_review yaml -->
## Contract review

```yaml
status: unchanged
reason: JST-ARCH-007 still requires closure to withdraw the handle before
  waiting for the exact in-flight reader. The test retains that assertion and
  the blocked-reader drain assertion; it only bounds a status probe that can
  itself race handle removal and wait behind the owned reader.
```
<!-- /evidence-field -->

<!-- evidence-field:milestone_review yaml -->
## Milestone review

```yaml
status: unchanged
reason: M33.4 and M30 remain In progress. Focused history-service acceptance
  passes, while Q-RUST-002, complete merged qualification and independent
  release gates retain their existing status.
```
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages



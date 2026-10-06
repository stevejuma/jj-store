---
schema: 1
id: E-UI-044
---

# E-UI-044 details 3

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
rustfmt --check --edition 2024 crates/jj-store-http/src/assets.rs crates/jj-store-http/tests/assets.rs crates/jj-store-server/src/config.rs crates/jj-store-server/src/config/types.rs crates/jj-store-server/src/installation.rs crates/jj-store-server/src/process.rs crates/jj-store-server/tests/configuration.rs
```

**Notes**

Changed Rust files formatted. Isolated .scratch/annotations-rustfmt.log.
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
pnpm exec prettier --check scripts/dev-demo.mjs scripts/dev-demo-state.mjs scripts/dev-demo-state.test.mjs scripts/real-browser-acceptance.mjs web/tests/real/browser-annotations.spec.ts spec/src/api.md spec/src/cli.md spec/src/security.md spec/src/spec-governance.md spec/src/milestones.md
```

**Notes**

Changed JS/TS/documents formatted. Isolated .scratch/annotations-format.log.
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
JJ_STORE_DEV_PORT=7483 JJ_STORE_DEV_BROWSER_ANNOTATIONS=1 JJ_STORE_DEV_RUSTFS_RUNTIME=/Users/stevejuma/code/lapismd/jj-store/runtime/embedded-rustfs/target/debug/jj-store-rustfs JJ_STORE_REAL_TEST='browser-annotations.spec.ts|native-repository.spec.ts|work-seed.spec.ts|work-store.spec.ts' pnpm web:test:real
```

**Notes**

8/8 cold direct browser checks pass in 2.0 minutes. Actual inline script execution and styled visible overlay succeed; physical restart with strict opt-out blocks both and preserves the complete WorkStore snapshot. Five native repository cases, seeded WorkStore and complete task/thread/accessibility flows pass after that strict restart. Real RustFS initial startup 1024 ms, prepared restart 675 ms, resident 173232 KiB; full fixture preparation 396633 ms. Disposable service stopped and root removed. Isolated .scratch/annotations-native.log and web/test-results-real screenshot.
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
curl -sS -D - -o /dev/null http://127.0.0.1:7483/
```

**Notes**

HTTP 200, development inline script/style allowance and retained resource directives.
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
pnpm spec:validate
```

**Notes**

Post-integration: 4 validators, 21 chapters, 131 requirements. Isolated .scratch/annotations-spec-integrated.log.
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
pnpm spec:first
```

**Notes**

Post-integration: 12 protected files, 5 canonical chapters. Isolated .scratch/annotations-spec-first-integrated.log.
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
pnpm check:progress
```

**Notes**

Resolved ledger: 103 tasks; parent evidence retained. Isolated .scratch/annotations-progress-integrated.log.
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
jj edit rlvvyrkm
```

**Notes**

Apply the verified isolated slice to clean main on the same committed M13 parent.
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
node --input-type=module (main native snapshot and identity capture)
```

**Notes**

Original main PID 60331, 3 projects, WorkStore version 28, manifest/publication 101. Exact records, snapshot and identity fingerprints in main .scratch/annotations-main-before.json.
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
JJ_STORE_DEV_RUSTFS_RUNTIME=/Users/stevejuma/code/lapismd/jj-store/runtime/embedded-rustfs/target/debug/jj-store-rustfs pnpm dev:restart
```

**Notes**

Only owned main demo restarted, without reset/reseed. Search ready; restart 3743 ms, resident 160432 KiB. Main .scratch/annotations-main-restart.log.
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
node --input-type=module (main development policy and persistence verification)
```

**Notes**

Main PID 7429 serves HTTP 200 with development inline scripts/styles and no unsafe-eval. API CSP remains strict. Exact WorkStore snapshot/project records and fixture/installation/catalog SHA-256 fingerprints unchanged. Main .scratch/annotations-main-verify.json.
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
lsof -nP -iTCP:7460 -iTCP:6046 -sTCP:LISTEN
```

**Notes**

Main PID 7429 on 7460; original Storybook PID 27372 on 6046.
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
test ! -e /var/folders/_3/k1jv9rtn47s_0k5bm6qn_lk80000gn/T/jj-store-real-browser-jQh6qV
```

**Notes**

Disposable fixture root removed.
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

<!-- evidence-field:scope markdown -->
## Scope

Explicit server configuration crosses the static HTTP asset owner, server validation/composition/installation and shared development runtime. After the exact regression, run complete HTTP asset tests and the server package suite (public configuration changes can affect its direct consumers), the existing cold TLS browser-bundle executable gate, and all direct demo browser fixtures using an explicit file regex. Generic acceptance retains strict CSP by default. The independent Rust-provisioned Git bridge browser fixture is not a dev-demo consumer and is excluded from this invocation. Root aggregates, unrelated UI/Storybook and full Rust workspace lanes remain out of scope.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

Additive format-2 server setting development_browser_annotations defaults to false; enabled only for loopback LocalTrusted with a browser bundle. Development asset CSP adds unsafe-inline for scripts/styles but retains the resource origins, framing/base/form restrictions and no unsafe-eval. API CSP/admission and all native authorization/publication authority remain unchanged. Demo rewrite enables it by default; JJ_STORE_DEV_BROWSER_ANNOTATIONS=0 opts out and generic cold acceptance defaults to that strict setting.
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

macOS ARM64; JJ 0.45.1; Rust 1.98.1; Node 24.10.0; pnpm 10.32.1; existing pinned browser/RustFS dependencies.
<!-- /evidence-field -->

<!-- evidence-field:next markdown -->
## Next

Retain explicit-development policy and strict-default/deployment rejection regressions.
<!-- /evidence-field -->

<!-- evidence-field:deferred list -->
## Deferred

- Actual Codex Annotate UI confirmation remains separate from the inline-helper
  browser proof while the in-app browser attachment times out.
- Visual baseline approval is not part of this change.
<!-- /evidence-field -->

<!-- evidence-field:artifacts markdown -->
## Artifacts

Focused/cold artifacts: /Users/stevejuma/code/lapismd/jj-store-dev-annotations/.scratch and web/test-results-real. Main restart/state verification: /Users/stevejuma/code/lapismd/jj-store/.scratch.
<!-- /evidence-field -->


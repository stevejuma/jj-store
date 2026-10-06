---
schema: 1
id: E-FRESH-005
---

# E-FRESH-005 details 2

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
JJ_STORE_DEV_ROOT=/tmp/jj-store-event-freshness-cold-20261004 JJ_STORE_DEV_PORT=7570 JJ_STORE_DEV_BROWSER_ANNOTATIONS=0 JJ_STORE_DEV_RUSTFS_RUNTIME=/Users/stevejuma/code/lapismd/jj-store/runtime/embedded-rustfs/target/debug/jj-store-rustfs pnpm dev:restart
```

**Notes**

Initial isolated real-RustFS/native/filesystem seed completed; final built UI restart completed after scheduler repair. Fixture preserved, no user fixture reset or installed executable replaced. cold-start2.log, cold-final-restart.log, cold-deadline-restart.log.
<!-- /run -->

<!-- run -->
### Run 2

```yaml
result: fail
failures: 2
hasNote: true
noteEncoding: markdown

```

```shell
pnpm exec playwright test --config web/playwright.real.config.ts web/tests/real/event-freshness.spec.ts
```

**Notes**

First cold run: 2 pass/2 fail. Files section is named Overview; selector corrected. Work event followed by Storage notice postponed the queued read to outage backoff; scheduled deadline now retained. cold-browser-final.log.
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
pnpm exec playwright test --config web/playwright.real.config.ts web/tests/real/event-freshness.spec.ts --grep "two clients"
```

**Notes**

1 pass after repair: two native clients refresh within five seconds, preserve draft, recover missed replay and defer reads while hidden. cold-work-recovery.log.
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
pnpm exec playwright test --config web/playwright.real.config.ts web/tests/real/automatic-updates.spec.ts
```

**Notes**

2 pass against the built service: own/external comments preserve drafts; open task form retains original version guards after a real event. Prior synthetic update no longer depends on polling. cold-existing-work-regressions.log.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project catalog-unit web/tests/vite/application-freshness.test.ts web/tests/vite/automatic-refresh.test.ts web/tests/vite/automatic-store-refresh.test.ts web/tests/vite/automatic-work-refresh.test.ts web/tests/vite/production-automatic-refresh.test.ts web/tests/vite/capture-freshness.test.ts web/tests/vite/capture-settings.test.ts web/tests/vite/capture-event-scope.test.ts web/tests/vite/settings-freshness.test.ts web/tests/vite/index-freshness.test.ts
```

**Notes**

52 pass/10 files after scheduler repair. Healthy pending retries are bounded, accepted reads stop timers, unrelated notices cannot postpone events; scoped capture status/sources/configuration regression passes. scheduler-consumers-final.log.
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
pnpm exec tsx --test web/tests/repository-conversion-manager.test.ts
```

**Notes**

4 pass: exact unknown request retained, altered/foreign receipt refused, validated receipt accepted. conversion-receipts-final.log.
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
cargo test -p jj-store-cli --test native_lifecycle --locked application_events_share_committed_observation_keep_checkpoint_narrow_and_reauthorize_delivery -- --nocapture
```

**Notes**

1 pass after bounded-read integration; external committed publication, two subscribers, checkpoint Storage-only and current revoke. native-feed-integrated-final.log. Inherited compact-unwind linker warning.
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
cargo test -p jj-store-cli --test local_browser --locked index_tasks_acceptance -- --nocapture
```

**Notes**

5 pass after integration: exact preparation, cancellation, quota/expiry and release without status polling. Corrected E-FRESH-003 filter name; local-task-filter-final.log.
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
cargo test -p jj-store-service --lib freshness --locked
```

**Notes**

2 pass, including independent principal audiences and bounded replay/witness suppression. hub-final2.log. Initial test compile used nonexistent PrincipalId::from_bytes; repaired to new.
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
cargo clippy -p jj-store-service -p jj-store-server -p jj-store-http -p jj-store-rpc -p jj-store-tantivy --no-deps --lib --locked -- -D warnings -A clippy::large_enum_variant
```

**Notes**

Integrated affected libraries pass with one inherited large-enum exemption. Strict upstream argument/range and service enum warnings remain pre-existing; not an unqualified strict/workspace lint pass. clippy-integrated.log.
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
pnpm web:check
```

**Notes**

0 errors/warnings after scheduler repair; web-deadline-final.log.
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
pnpm format:rust:check
```

**Notes**

Pass; rust-format-final.log.
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
pnpm docs:check
```

**Notes**

Serial build pass: 139 files, 13,696,662 bytes, valid closure; docs-final-serial.log.
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
pnpm spec:validate
```

**Notes**

5 validators, 23 chapters, 172 requirements; spec-final-complete.log.
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
pnpm check:boundaries
```

**Notes**

Pass; boundaries-complete.log.
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
pnpm check:architecture
```

**Notes**

986 production modules pass; architecture-complete.log.
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
pnpm check:ui-ownership
```

**Notes**

Pass; ownership-complete.log.
<!-- /run -->

<!-- run -->
### Run 18

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
pnpm exec prettier --check crates/jj-store-cli/tests/native_lifecycle/application_events.rs web/tests/real/event-freshness.spec.ts web/tests/real/automatic-updates.spec.ts web/tests/vite/capture-event-scope.test.ts
```

**Notes**

Prettier cannot parse Rust; corrected to owning cargo formatter plus scoped JS/Markdown checks. final-test-format.log.
<!-- /run -->
<!-- /evidence-field -->


---
schema: 1
id: E-UI-019
---

# E-UI-019 details 3

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
DEVELOPER_DIR=/Library/Developer/CommandLineTools VITEST_MAX_WORKERS=1 pnpm spec:check
```

**Notes**

All 18 canonical specification lanes pass after the two story-readiness repairs, including architecture, contracts, diagnostics, 68 Storybook interaction/accessibility cases, cold catalog and production-browser acceptance, the locked Rust workspace and mdBook.
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
pnpm dev:restart; curl browser root; inspect runtime process, metadata and RustFS ownership
```

**Notes**

A cold public restart rebuilt the browser and native binaries, started the checkout-owned pinned RustFS container and native server, seeded repository 91884361f1dd4eb2cce59b97142b0356 with conflict/demo, and returned HTTP 200. The recorded PID runs this checkout's binary with the generated server.toml, the container is running with the checkout ownership label, and runtime.json contains no token or other credential field. The review stack remains running; the one-time bearer credential is reported only to the user.
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
pnpm spec:first; pnpm check:progress; pnpm format:check
```

**Notes**

Final protected-file mapping covers 33 files across five canonical chapters; all 46 milestone tasks, rustfmt and Prettier checks pass after recording the aggregate and cold-restart outcomes.
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

Community source 9d82df591548a1efc49681b744f7a5f244b8ac99; Design Core source 7beeb4eb9a9d4c75da61df9ae9f3ad02694078df inspected with public MergeEditor, DialogFrame, Button, Alert, Spinner and confirmation primitives consumed through registry range ^0.2.0 and the guarded local source hook; @tanstack/svelte-query 6.3.0; Storybook 10.5.7; Vitest 4.1.11; Playwright 1.63.x; RustFS rustfs/rustfs@sha256:b7014e0ce2bc703c1316b3ef760e29dfae61fe4a50d1a66fa89638e0f8ea211f.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

The UI consumes existing V1 conflict descriptors, conflict-upload responses and resolve_conflicts mutation requests. Every command retains full native commit IDs, path fingerprints, ordered terms and scoped upload handles. Staging content does not publish; only a committed durable receipt advances the selected snapshot. No native object, WAL, API, credential, URL or persistent-layout format changes. The dev:restart task rebuilds and restarts only checkout-owned services and prints credentials without storing the bearer token in the checkout or runtime metadata. Visual tests remain skipped at the user's request and stories remain visual-pending.
<!-- /evidence-field -->


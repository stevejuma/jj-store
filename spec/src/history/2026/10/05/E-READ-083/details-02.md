---
schema: 1
id: E-READ-083
---

# E-READ-083 details 2

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
pnpm dev:backend:build
```

**Notes**

.local/bounded-reads/managed/main-backend-build.log: initial main backend success in 2m01s.
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
pnpm install --frozen-lockfile
```

**Notes**

.local/bounded-reads/managed/preview-dependencies.log: inherited pnpmfileChecksum mismatch.
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
pnpm install --force --no-frozen-lockfile --offline
```

**Notes**

.local/bounded-reads/managed/preview-dependencies-offline.log; jj restore -- pnpm-lock.yaml retains tracked bytes.
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
JJ_STORE_DEV_ROOT=/Users/stevejuma/code/lapismd/jj-store-codex-preview/.local/codex-preview JJ_STORE_DEV_PORT=7470 JJ_STORE_DEV_BACKEND_PORT=7472 JJ_STORE_DEV_SERVER=/Users/stevejuma/code/lapismd/jj-store/target/debug/jj-store pnpm dev:ui
```

**Notes**

.local/bounded-reads/managed/preview-cold-start.log, preview-cold-start-corrected.log and preview-cold-navigation-start.log: preserved fixture, fresh Vite/backend; initial stale dependency lacked panel-toggle.tokens.css until repaired.
<!-- /run -->

<!-- run -->
### Run 5

```yaml
result: fail
failures: 2
hasNote: true
noteEncoding: markdown

```

```shell
python3 .local/bounded-reads/managed/linux-app-proof.py
```

**Notes**

.local/bounded-reads/managed/linux-app-proof-header-before.log and linux-app-proof-shape-before.log: harness header and flattened snapshot assumption errors.
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
python3 .local/bounded-reads/managed/linux-app-proof.py
```

**Notes**

.local/bounded-reads/managed/linux-app-proof.log: eleven 200 responses; 150 distinct history rows with bounded metadata.
<!-- /run -->

<!-- run -->
### Run 7

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
python3 .local/bounded-reads/managed/linux-final-proof.py
```

**Notes**

.local/bounded-reads/managed/linux-final-startup-busy.log: Busy during concurrent cold browser startup. Bounded admission; settled probes retried.
<!-- /run -->

<!-- run -->
### Run 8

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
python3 .local/bounded-reads/managed/linux-final-proof.py
```

**Notes**

.local/bounded-reads/managed/linux-final-native-method-before.log: native GET reference route sent POST; corrected.
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
python3 .local/bounded-reads/managed/linux-final-proof.py
```

**Notes**

.local/bounded-reads/managed/linux-final-source-scan-before.log: all 7 HTTP calls pass; source hash scan omitted store/extra/heads directory. Corrected byte proof succeeds.
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
python3 .local/bounded-reads/managed/linux-final-bytes.py
```

**Notes**

.local/bounded-reads/managed/linux-final-bytes.log: 18 unchanged source files, 89,135,722 bytes, 2 reused segments.
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
CARGO_TARGET_DIR=/Users/stevejuma/code/lapismd/jj-store/target pnpm dev:backend:build
```

**Notes**

.local/bounded-reads/managed/monitor-backend-build.log: final landed-equivalent source from bounded workspace, success in 1m54s.
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
JJ_STORE_DEV_ROOT=/Users/stevejuma/code/lapismd/jj-store-codex-preview/.local/codex-preview JJ_STORE_DEV_PORT=7470 JJ_STORE_DEV_BACKEND_PORT=7472 JJ_STORE_DEV_SERVER=/Users/stevejuma/code/lapismd/jj-store/target/debug/jj-store pnpm dev:backend:restart
```

**Notes**

.local/bounded-reads/managed/preview-final-backend-restart.log: only 7472 backend replaced; Vite 7470 and unrelated listeners preserved.
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
python3 .local/bounded-reads/managed/linux-final-proof.py
```

**Notes**

.local/bounded-reads/managed/linux-final-corrected-proof.log: final seven 200 responses and complete source/restart byte assertions.
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
cua_repl: cold Linux Commits/diff rendering and 15-second settled network observation
```

**Notes**

.local/bounded-reads/managed/linux-cold-final-dom.txt, linux-cold-final.png, linux-idle-network.json: rendered selected text diff, no rebuild/loading condition, no further data requests.
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
pnpm spec:first
```

**Notes**

.local/bounded-reads/managed/landing-docs-before.log: owning architecture chapter edit did not apply because the match crossed formatted lines; added the acceptance boundary explicitly.
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
pnpm exec prettier --check docs/implementation-plan.md spec/src/architecture.md spec/src/milestones.md && pnpm spec:validate && pnpm spec:first && pnpm check:progress
```

**Notes**

.local/bounded-reads/managed/landing-final-docs.log: targeted documentation/governance checks.
<!-- /run -->
<!-- /evidence-field -->


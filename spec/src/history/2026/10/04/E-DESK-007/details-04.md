---
schema: 1
id: E-DESK-007
---

# E-DESK-007 details 4

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
Launch unique debug app with JJ_STORE_DESKTOP_DATA_DIR=/private/tmp/jst-xvsdkapu/state; CUA native WKWebView controls; ps/process identity checks
```

**Notes**

Cold real embedded RustFS and production assets:registered local repository opens at verified cwd; input/output, tabs, navigation retention, maximize/restore, header toggle/expanded icon and Cmd+J work. Initial long fixture path exceeded Unix socket bound; shortened only disposable state path. Quit killed app, service, RustFS, terminal shells and separate-group sleep job. Restart opens a fresh shell, never old sessions. Web Inspector location.reload() preserved output and PID3945; new input reported the same PID. Production CSP still rejects ipc://; Tauri postMessage fallback succeeds. Inspector hotkey denied devtools permission, context Inspect Element used. Reload logs cancelled old HTTP requests/stale callback; accepted new renderer and terminal reattached. No new permission/CSP relaxation. Native SSE reconnect log noise retained as observation, not terminal failure.
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
Native WKWebView close; pnpm exec vitest run --config catalog/vitest.config.ts --project catalog-unit web/tests/vite/terminal.test.ts -t "ignores callbacks" (before)
```

**Notes**

close-before.log:one failed/four skipped; closing tab correctly kills shell but late focus callback emits obsolete Terminal unavailable toast. Added session-liveness guards to input/scroll/focus/ack/resize and callback failure handling while retaining failures for active sessions.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project catalog-unit web/tests/vite/terminal.test.ts; pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-terminal/Terminal.stories.svelte; pnpm web:check; pnpm check:architecture; focused Prettier
```

**Notes**

close-after.log5/5; close-stories.log4/4 with axe; close-web.logzero errors/warnings; close-architecture.log940 modules. New exact regression proves obsolete errors are ignored after close and active-session errors remain visible. Wider shell consumers already22/22 and unchanged by this private controller repair.
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
Rebuild/stage/rebundle production UI; launch unique debug app cold; CUA close and Quit; Python os.kill(pid,0) assertions
```

**Notes**

native-final.json:successful tab close produced no error notice and removed shell8880 and background sleep9372. Explicit Quit removed app8574, service8638 and RustFS8639 as well. Renderer reload/header evidence above retained. CUA querying an already-quit app launched a failed default-state test instance; terminated only that owned acceptance instance before correctly relaunching private state. All final disposable processes are gone.
<!-- /run -->

<!-- run -->
### Run 5

```yaml
result: fail
failures: 7
hasNote: true
noteEncoding: markdown

```

```shell
pnpm spec:first; pnpm check:progress; focused document Prettier (first final audit)
```

**Notes**

final-first.log:architecture chapter missing for package dependency changes; added owned desktop engine/renderer boundary. final-progress.log:five unsupported whole-file test references; retained valid Rust named tests and exact UI commands in runs. final-doc-format.log:one desktop chapter formatting issue; formatted only changed chapters. Product checks unchanged.
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
pnpm spec:validate; cargo fmt --manifest-path desktop/Cargo.toml --all -- --check; historical evidence object/raw-prefix assertions
```

**Notes**

final-spec.log:22 chapters/168 requirements; native Rust formatting passes. All413 prior evidence objects and their raw JSON prefix preserved; only new M17.5 row, M17 requirement/evidence links and E-DESK-007 changed.
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
pnpm spec:first; pnpm spec:validate; pnpm check:progress; pnpm exec prettier --check spec/src/desktop.md spec/src/architecture.md spec/src/verification.md desktop/README.md .local/desktop-terminal-evidence/final-entry.json (after final audit repairs)
```

**Notes**

final-first2.log:81 protected files/7 canonical chapters; final-spec2.log:22 chapters/168 requirements; final-progress2.log:137 tasks; final-format2.log:formatting passes. All logs and native-final.json retained in ignored .local/desktop-terminal-evidence. No product edits after final native close regression acceptance.
<!-- /run -->
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

macOS27.0 ARM64 (26A428); Rust1.98.1; Tauri2.11.5/Rust runtime2.11.4; Tauri JS API2.11.1; portable-pty0.9.0; Alacritty terminal0.26.0; Buzz ef2aa1ae38fadcc0bc22b8bf6ed96b35933146be; pnpm10.32.1 (dlx reports12.6.0).
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

Desktop-only typed terminal targets/sessions/frames with subscription sequences and viewport generation. Existing protected HTTP folder reads unchanged. Versioned device dock geometry; in-memory shell sessions/output/open identities; no repository publication format change.
<!-- /evidence-field -->

<!-- evidence-field:scope markdown -->
## Scope

Completed repository-terminal implementation in isolated Jujutsu workspace desktop-terminal at b11c0486, including user-requested header toggle beside details, expanded bottom-panel icon and Buzz-style muted rounded toolbar with no footer. Native runtime, owned PTY engine and direct shell consumers only; repository authority/HTTP shell access unchanged. UI acceptance uses existing local Design Core substitution, while tracked manifest/lock retain registry ranges. Registry-resolved package acceptance stays with M16; wider M17.2 host/dirty-edit/distribution gates remain open, so M17.5 is Partial despite passing narrow terminal acceptance. Ignored release latency/staged managed-runtime tests are not claimed passed; native cold terminal proof is independently recorded. Root aggregates, unrelated crates/stories, visual baselines, resets, installation, main landing and publication are out of scope.
<!-- /evidence-field -->

<!-- evidence-field:result markdown -->
## Result

pass
<!-- /evidence-field -->

<!-- evidence-field:remaining list -->
## Remaining

[]
<!-- /evidence-field -->

<!-- evidence-field:next markdown -->
## Next

Retain this committed workspace for integration; complete existing M16 registry consumer and M17.2-M17.4 desktop distribution gates separately.
<!-- /evidence-field -->


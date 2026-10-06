---
schema: 1
id: E-DESK-005
---

# E-DESK-005 details 3

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
User manual review after AppKit correction; CUA cold restart with saved Glass background
```

**Notes**

User confirmed default hover glyphs, reported a transient geometry reset, then said alignment seemed okay. A later cold inactive screenshot still showed default16px centers; reopened late-layout correction instead of treating that confirmation as full native acceptance.
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
CUA Settings > Appearance: Glass background off then on; Cmd+Q; lsof -nP -iTCP:57653 -sTCP:LISTEN; ps -p 18424,18436 -o pid=,command=; CUA cold getApp exact main bundle
```

**Notes**

Glass-off restores the opaque Buzz gradient; glass-on49percent restores the muted native wash while main content stays white/opaque. Quit removes both recorded host/service PIDs and listener. Cold restart keeps original57653 and saved glass wash. AX click on menu text needed screenshot-coordinate fallback; one noWindowsAvailable and stale-state refusals were refreshed, not product passes.
<!-- /run -->

<!-- run -->
### Run 3

```yaml
result: fail
failures: 2
hasNote: true
noteEncoding: markdown

```

```shell
pnpm spec:first (before mapped chapter updates)
```

**Notes**

Required architecture.md and spec-governance.md were missing; added explicit independent native presentation and audited interop/observer ownership, without changing gate rules.
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
cargo check --manifest-path desktop/Cargo.toml --offline; cargo check --manifest-path desktop/Cargo.toml --locked
```

**Notes**

Late-window-update observer compiles with minimal cached ObjC/notification features; independent lock adds only root dependency edges. /tmp/jj-desktop-glass-update-observer-{check,locked}.log.
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
pnpm desktop:bundle (late-window-update observer)
```

**Notes**

Native-only debug bundle completed even though the tool turn was interrupted; inspected final success log and no remaining build process. Staged212-asset web UI/paired service reused; unrelated Settings edits were not repackaged. /tmp/jj-desktop-glass-update-observer-bundle.log.
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
CUA Cmd+Q; lsof -nP -iTCP:57653 -sTCP:LISTEN; pgrep -fl jj-store-desktop/desktop-supervised; CUA cold launch final bundle and settled screenshot
```

**Notes**

Old host/service and listener absent before launch. Final notification correction retains23px native/button centers and16px leading inset after settled cold startup; saved native glass wash returns and main stays opaque. Original macOS buttons are retained; observer corrections are idempotent/reentrancy guarded, token removed on exit.
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
cargo fmt --manifest-path desktop/Cargo.toml -- --check; pnpm spec:first; pnpm spec:validate; pnpm check:progress
```

**Notes**

Rust formatting passes; mapped chapters restored spec-first,5validators/22chapters/152requirements validate and125tasks progress bookkeeping passes. Native broad gesture/distribution gates remain unpromoted.
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
pnpm exec prettier --write (explicit owned Markdown/JSON/CSS/Svelte/TypeScript paths from /tmp/jj-desktop-glass-owned-paths.json); cargo fmt --manifest-path desktop/Cargo.toml -- --check
```

**Notes**

Only the owned slice formatted; canonical desktop/governance paragraphs wrapped, production UI files unchanged by formatting.
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
pnpm desktop:test (final); pnpm check:ui-ownership (final); pnpm spec:first (final); pnpm spec:validate (final)
```

**Notes**

5 host tests pass, staged real-runtime target remains ignored. Ownership and5validators/22chapters/152requirements pass; spec-first sees58 protected files including separate shared-main Settings changes.
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
pnpm check:progress (premature evidence pass status)
```

**Notes**

Checker correctly rejected pass while remaining native review gates exist. Restored pending evidence status without changing checks or removing gaps; focused successful runs remain individually recorded.
<!-- /run -->
<!-- /evidence-field -->

<!-- evidence-field:result markdown -->
## Result

pending
<!-- /evidence-field -->

<!-- evidence-field:remaining list -->
## Remaining

- Final observer version retains alignment at cold startup; repeated manual
  active movement/resize and default hover glyphs after that final observer
  refinement remain a bounded review follow-up. User verified glyphs on the
  preceding identical native tracking implementation.
- Visual baseline approval and remaining M17 render, native drag/double-click,
  dirty-edit, accessibility and distribution gates remain separate.
<!-- /evidence-field -->

<!-- evidence-field:scope markdown -->
## Scope

Main workspace native presentation and direct UI consumers only. The boolean glass command is presentation-only. Root aggregates, unrelated Rust/UI suites and HTTP generation remain out of scope. Shared-main Settings/Reviews edits belong to the coordinated separate task and are excluded from this commit.
<!-- /evidence-field -->

<!-- evidence-field:next markdown -->
## Next

The verified compile/browser/cold-start slice is ready for a file-scoped commit. Leave the final main debug app open; manually review repeated movement/resize and hover after the final observer change, then continue the separate M17 gates.
<!-- /evidence-field -->


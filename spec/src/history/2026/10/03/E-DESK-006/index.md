---
schema: 1
id: E-DESK-006
---

# E-DESK-006

<!-- evidence-field:id markdown -->
## Id

E-DESK-006
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M17
- M17.2
- M17.3
<!-- /evidence-field -->

<!-- evidence-field:date markdown -->
## Date

2026-10-03
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

native-chrome-manual-and-zoom-qualification
<!-- /evidence-field -->

<!-- evidence-field:revision markdown -->
## Revision

rsouwqrt (enclosing change)
<!-- /evidence-field -->

<!-- evidence-field:before markdown -->
## Before

E-DESK-005 retained the final observer build cold-start proof, but movement, edge resizing and hover glyphs after that last refinement were pending. Main is clean after native vssplolt/bdf88069 and separate Settings soytvzqt/89243006.
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

macOS ARM64; Tauri2.11.5, Tao0.35.3, Wry0.55.1; native debug bundle contains final vssplolt AppKit update observer. Its staged production renderer predates the separate Settings commit. Native host/chrome source is unchanged by Settings; latest Settings bundle qualification is outside this entry.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

No implementation, API, native authority, appearance preferences or baseline changes. Persistent installation and original loopback57653 retained.
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- spec/src/milestones.md
- desktop/src/macos_chrome.rs
- desktop/src/native_window.rs
- desktop/src/native_glass.rs
- web/src/lib/ui/community-app-shell/desktop-chrome.css
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

[]
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
CUA getApp exact main jj-store.app; native window Raise; screenshot; drag integrated header from700,44 to740,80
```

**Notes**

Raise retained correct inactive geometry, but native mouse dragging returned noWindowsAvailable. This attempt is inconclusive for movement, not a product failure or pass; movement is qualified by the separate user review below.
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
CUA native fullscreen-button secondary action zoom the window; refreshed AX and screenshot; repeat secondary action to restore; screenshot
```

**Notes**

Actual native Zoom/Restore changes screenshot size2530x1504 -&gt;3456x1988 -&gt;2530x1504. Original close/minimize/zoom controls retain46px Retina screenshot centers, matching the sidebar toggle at23 logical pixels after resize and settled relayout. Does not qualify dragging or header double-click.
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
User manual review in final running bundle: move window, resize by edge, hover native traffic lights
```

**Notes**

User explicitly answered Aligned; hover glyphs appear. Resolves the final observer version movement/edge-resize/native-hover gap from E-DESK-005 without replacing native controls or claiming screen-sharing-obscured automation glyph evidence.
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
CUA final active-window AX and screenshot; lsof -nP -iTCP:57653 -sTCP:LISTEN
```

**Notes**

After manual edge resizing,2540x1658 active screenshot still shows native colored controls centered with sidebar toggle. Saved native glass wash and opaque content retained; original listener remains live. App stays open for review.
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
pnpm exec prettier --write spec/src/milestones.md; pnpm exec prettier --check spec/src/milestones.md; pnpm check:progress; pnpm spec:validate
```

**Notes**

Canonical formatting passes;125-task bookkeeping and5validators/22chapters/152requirements pass. Documentation-only qualification: no build, root aggregate or unrelated suite needed.
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

Only final traffic-light alignment and default native hover behavior. Prior native glass toggle/cold preference restoration evidence remains in E-DESK-005. No code changed, so Rust/UI builds and suites were not repeated. Root aggregates and unrelated consumers are out of scope. M17 first-render, dirty-edit, folder controls, native header double-click/drag qualification, accessibility, visual baselines and distribution remain separate; no milestone is promoted.
<!-- /evidence-field -->

<!-- evidence-field:next markdown -->
## Next

Retain this narrow successful review and continue separately governed M17 gates. The original E-DESK-005 history remains unchanged; E-DESK-006 resolves its bounded final movement/resize/hover follow-up.
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages



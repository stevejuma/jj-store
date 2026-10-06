---
schema: 1
id: E-DOC-001
---

# E-DOC-001 details 3

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
pnpm docs:test:browser (final after WebKit trigger fix)
```

**Notes**

Fresh production web/docs build followed by 8/8 cold Chromium/WebKit tests under the real Rust asset router. Covers network-denied external requests, local search, SVG load, deep links, same-origin navigation, desktop-width and 390px layout, Escape/search dismissal, iframe Tab boundary, actual-trigger focus return, unsent draft and route/scroll retention, missing docs retry, no-repository installation sidebar and Axe. No native packaged WKWebView claim.
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
pnpm docs:generate; node --test scripts/docs.test.mjs (final reference aliases); node scripts/docs.mjs stage
```

**Notes**

Visible argument aliases also derive from Clap; no parser command executes. Six tooling regressions remain passing. Final strict site generation and delivery stage retain the complete manual and reference closure. Docs source and specification remain separate.
<!-- /run -->
<!-- /evidence-field -->

<!-- evidence-field:result markdown -->
## Result

pending
<!-- /evidence-field -->

<!-- evidence-field:remaining list -->
## Remaining

- Registry-resolved Design Core qualification remains blocked by pre-existing
  published API gaps.
- Packaged native WKWebView/DMG cold acceptance and visual baselines are not
  claimed by browser-engine and resource-staging tests.
<!-- /evidence-field -->

<!-- evidence-field:scope markdown -->
## Scope

Manual and asset delivery only. Shared HTTP/desktop asset loaders justify complete direct asset/packaging regressions. Existing repository identity/storage/API protocols remain unchanged. No full workspace suite, new real-RustFS qualification, fixture reset, installed CLI replacement, package publication or hosting.
<!-- /evidence-field -->

<!-- evidence-field:next markdown -->
## Next

Review the manual and Help in the documentation workspace. Keep M19 Partial until registry-only foundational-package and packaged native-host release qualification are completed; no hosting/publication requested.
<!-- /evidence-field -->


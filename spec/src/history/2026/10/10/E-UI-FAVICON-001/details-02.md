---
schema: 1
id: E-UI-FAVICON-001
---

# E-UI-FAVICON-001 details 2

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
sips --resampleHeightWidth 64 64 desktop/icons/icon.png --out .local/favicon-proof/favicon-64.png
```

**Notes**

Exported accepted outlined 1024px application icon at 64px. Python stdlib base64 wrapped its 8309-byte RGBA PNG in the self-contained SVG. Pillow was unavailable (ModuleNotFoundError: PIL); no dependency was installed.
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
curl -sS -D .local/favicon-proof/headers.txt http://127.0.0.1:7460/agent-icons/favicon.svg -o .local/favicon-proof/served.svg
```

**Notes**

HTTP 200, image/svg+xml, 11231 bytes, existing Vite CSP and nosniff retained. Final source and served bytes match.
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
cua.createBrowserTab("2", "http://127.0.0.1:7460/#/threads/80d8731bbc255ca6373c797e8b9eb4b4/e2e2e2e2e2e2e2e2e2e2e2e2e2e2e2e2", {visible:false}); faviconCdp.send("Runtime.evaluate", {expression: "new Promise(resolve=>{const icon=document.querySelector("link[rel=icon]");const image=new Image();image.onload=()=>resolve({loaded:true,url:image.src,width:image.naturalWidth,height:image.naturalHeight});image.onerror=()=>resolve({loaded:false,url:image.src});image.src=icon.href;})", awaitPromise:true, returnByValue:true})
```

**Notes**

Live app entry registers the same-origin SVG; browser decodes width=64, height=64, loaded=true. Temporary tab 19 closed. dev.logs({levels:["warn","error"],limit:10}) returned [] during this narrow verification; this does not qualify or resolve the independent ResizeObserver issue.
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
pnpm web:check
```

**Notes**

Zero Svelte errors and warnings. /tmp/favicon-web-check.log.
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
node --test scripts/web-manifest.test.mjs
```

**Notes**

Both existing manifest tests pass, 2 tests / 0 failures. /tmp/favicon-manifest-tests.log. No implementation-mirroring test added for a static icon.
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
pnpm check:ui-ownership
```

**Notes**

Owned-source boundary passes. /tmp/favicon-ownership.log. No production/catalog/sibling imports changed.
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
pnpm web:build
```

**Notes**

Initial Vite compilation and manifest generation passed; documentation staging refused an outdated application source-review fingerprint. /tmp/favicon-build.log. Reviewed the application manual and updated browsing guide and coverage reason; no gate waived.
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
pnpm docs:review application "Reviewed browsing and settings: local tab branding uses the accepted logo; existing compact footer menus and inbox controls retain documented navigation and device-local behavior."
```

**Notes**

Updated only the application review after reading its guides. The stale fingerprint was inherited from prior source changes; favicon itself adds no web/src change.
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
pnpm web:build
```

**Notes**

Rerun passes Vite build, asset inventory and strict documentation staging. /tmp/favicon-build-retry.log. Vite retains its existing &gt;500kB chunk warning; no bundle-performance claim is made. 9 Mermaid diagrams rendered offline; documentation closure validates 141 files / 22477109 bytes.
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
pnpm exec vite preview --config web/vite.config.ts --port 7464
```

**Notes**

Fresh loopback built-bundle server. Temporary tab 20 opened the built HTML at http://127.0.0.1:7464/. CDP Image load from the registered link returned loaded=true, registeredType=image/svg+xml, width=64, height=64. Preview has no API backend and showed its expected connection refusal, so this verifies built static registration/decoding only. Tab closed and owned listener PID 9035 stopped; no existing service restarted/reset.
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
node --input-type=module -e 'import {readFileSync} from "node:fs"; import assert from "node:assert/strict"; const source=readFileSync("web/public/agent-icons/favicon.svg"); assert.deepEqual(source,readFileSync("web/dist/agent-icons/favicon.svg")); assert.deepEqual(source,readFileSync(".local/favicon-proof/served.svg")); assert.ok(JSON.parse(readFileSync("web/dist/asset-manifest.json")).files.includes("agent-icons/favicon.svg")); assert.match(readFileSync("web/dist/index.html","utf8"),/href="\/agent-icons\/favicon.svg"/); console.log("Favicon source, build, HTTP bytes, manifest and HTML registration pass");'
```

**Notes**

Final 11231-byte SVG has SHA256 4ad94fcc1ade832d407a0bcec068b595c2958b1463e185744dded823f78a8d9f. Python stdlib additionally validated embedded PNG signature, RGBA color type 6, 64x64 dimensions, SVG width/height, and no scripts or external image references.
<!-- /run -->

<!-- run -->
### Run 12

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
pnpm spec:search -- JST-UI-018
```

**Notes**

CLI rejects forwarded --. Retried with the supported invocation below.
<!-- /run -->

<!-- run -->
### Run 13

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
pnpm spec:search JST-UI-018
```

**Notes**

QMD native module ABI does not match active Node ABI 147. Used the advertised bounded rg fallback and read canonical source; no tooling reinstall or broad dependency change.
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
rg -n -i --glob '*.md' JST-UI-018 spec/src/current/contracts/ui spec/src/current/contracts/api spec/src/current/verification
```

**Notes**

Located UI 03-requirements.md and owning 08-device-appearance.md, API 26-browser-appearance-state.md and verification evidence.
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
pnpm exec prettier --write docs/maintenance/coverage.json docs/site/guides/browsing.md web/index.html web/public/agent-icons/SOURCES.md spec/src/current/contracts/ui/08-device-appearance.md spec/src/current/contracts/api/26-browser-appearance-state.md spec/src/current/progress/M24/agents-presentation-review.md
```

**Notes**

Formatted authored entry/provenance/manual/contracts and retained compact coverage arrays.
<!-- /run -->

<!-- run -->
### Run 16

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
pnpm check:progress
```

**Notes**

Initial revision-pinned check overlapped an evidence navigation edit; rejected as working copy changed. Rerun with edits finished.
<!-- /run -->

<!-- run -->
### Run 17

```yaml
result: fail
failures: 8
hasNote: true
noteEncoding: markdown

```

```shell
pnpm check:progress
```

**Notes**

Rerun found four informal labels in tests instead of repository path#named-test references (four missing-file and four invalid-reference diagnostics). Replaced them with the two actual manifest test references. Browser/asset checks stay recorded as actual runs.
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
pnpm spec:validate --only summary,fileLimits,governance,verification,book
```

**Notes**

Five validators passed, 3481 chapters / 181 requirements. /tmp/favicon-spec.log.
<!-- /run -->
<!-- /evidence-field -->


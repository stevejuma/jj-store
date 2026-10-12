---
schema: 1
id: E-REMOTE-AGENTS-002
---

# E-REMOTE-AGENTS-002 details 2

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
cargo build -p jj-store-cli --bins --locked
```

**Notes**

Initial and post-fix builds passed; logs /tmp/jj-remote-real-build.log and /tmp/jj-remote-real-build2.log. Inherited macOS large __eh_frame linker warning retained.
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
node node_modules/vite/bin/vite.js build --config web/vite.config.ts
```

**Notes**

Fresh production UI build passed; inherited chunk size/dynamic import advisories remain. /tmp/jj-remote-real-web.log.
<!-- /run -->

<!-- run -->
### Run 3

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
node scripts/web-manifest.mjs && node scripts/docs.mjs stage
```

**Notes**

Manifest generation passed. Docs stage initially failed because uv was not in PATH; corrected environment only. /tmp/jj-remote-real-web-stage.log.
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
PATH="/Users/stevejuma/.local/bin:$PATH" node scripts/docs.mjs stage
```

**Notes**

Strict documentation build and stage passed: 141 files, 24,592,989 bytes, links/anchors valid. /tmp/jj-remote-real-web-stage2.log.
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
python3 scripts/remote-agents-acceptance.py --server target/debug/jj-store --jj target/debug/jj --rustfs /Users/stevejuma/code/lapismd/jj-store/runtime/embedded-rustfs/target/debug/jj-store-rustfs --web web/dist --privacy-cache /Users/stevejuma/code/lapismd/jj-store/.local/dev-demo/local/agent-execution/privacy-cache
```

**Notes**

Seven initial runs are recorded in /tmp/jj-remote-real-acceptance1.log through acceptance7.log: (1) harness omitted production asset manifest, (2) wrong WorkStore command tag, (3) outsider denial expected as empty list, (4) invalid PNG fixture, (5) duplicate origin attempted via Add instead of sign-in, (6) product freshness Origin omission invalidated sign-in, (7) harness read event envelope at wrong level. Harness mistakes corrected against owned contracts; no product assertion or admission rule weakened.
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
cargo test -p jj-store-server --lib connections::event_tests::gateway_freshness_uses_registered_origin_and_session_authentication --locked -- --exact
```

**Notes**

Before fix: exact new regression 0 passed / 1 failed (Unavailable). Registered-origin admission in the fixture rejects the omitted Origin. /tmp/jj-remote-freshness-before.log.
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
cargo test -p jj-store-server --lib connections:: --locked
```

**Notes**

After fix: all five gateway tests pass, including exact regression, retained cursor/session authentication, disconnect and identity/binary checks. /tmp/jj-remote-freshness-after.log.
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
python3 scripts/remote-agents-acceptance.py --server target/debug/jj-store --jj target/debug/jj --rustfs /Users/stevejuma/code/lapismd/jj-store/runtime/embedded-rustfs/target/debug/jj-store-rustfs --web web/dist --privacy-cache /Users/stevejuma/code/lapismd/jj-store/.local/dev-demo/local/agent-execution/privacy-cache
```

**Notes**

Cold isolated acceptance passes in acceptance8.log and acceptance9.log; the latter additionally observes a real committed profile-change event through WebSocket, rather than only a ready handshake. Agent publication is read through the authorized AgentStore API after restart; one started event and the original completed request prove no replay. The deterministic process edited a private native code workspace; dirty physical source remains unchanged. Final tightened busy-code recovery run recorded in acceptance10.log.
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
cargo check -p jj-store-server --locked
```

**Notes**

Affected server crate check passed. /tmp/jj-remote-server-check.log.
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
node scripts/check-boundaries.mjs
```

**Notes**

17 package boundaries pass. /tmp/jj-remote-real-boundaries.log.
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
node node_modules/@lapismd/spec-validator/dist/cli.js validate --only ui-ownership
```

**Notes**

UI ownership passes; no UI implementation/dependency changes. /tmp/jj-remote-real-ownership.log.
<!-- /run -->

<!-- run -->
### Run 12

```yaml
result: fail
failures: 19
hasNote: true
noteEncoding: markdown

```

```shell
node scripts/web-architecture.mjs
```

**Notes**

Same 19 inherited oversized files as E-REMOTE-AGENTS-001. All new files comply; no old file size limit weakened. /tmp/jj-remote-real-architecture.log.
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
node --check scripts/remote-agents-browser.mjs
```

**Notes**

JavaScript syntax passes.
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
python3 -c 'import ast; from pathlib import Path; ast.parse(Path("scripts/remote-agents-acceptance.py").read_text()); print("Python syntax passed")'
```

**Notes**

Python AST parse passes without writing bytecode.
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
node scripts/docs.mjs check
```

**Notes**

Configuration topic review fingerprint needed renewal after source correction. Reviewed and updated configuration and agent-guide pages; no review gate bypassed. /tmp/jj-remote-real-docs-check.log.
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
node scripts/docs.mjs review configuration "Reviewed the registered remote Origin correction, unchanged admission and installation identity fence; documented actual gateway freshness and the separate real-storage deterministic-ACP acceptance harness."
```

**Notes**

Updated configuration topic review and formatted coverage metadata.
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
PATH="/Users/stevejuma/.local/bin:$PATH" node scripts/docs.mjs check
```

**Notes**

Final strict documentation build/closure passed: 141 files, 24,594,665 bytes. /tmp/jj-remote-real-docs-check2.log.
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
node node_modules/@lapismd/spec-validator/dist/cli.js validate --only summary,verification,ui-ownership,governance,fileLimits
```

**Notes**

Five selected source validators passed; final evidence is revalidated before commit. /tmp/jj-remote-real-spec.log.
<!-- /run -->
<!-- /evidence-field -->


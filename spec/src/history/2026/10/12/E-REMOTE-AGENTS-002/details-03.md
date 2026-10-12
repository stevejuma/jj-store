---
schema: 1
id: E-REMOTE-AGENTS-002
---

# E-REMOTE-AGENTS-002 details 3

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
cargo fmt --all -- --check
```

**Notes**

Rust formatting passes. /tmp/jj-remote-real-rust-format.log.
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
node node_modules/prettier/bin/prettier.cjs --check docs/agent-execution.md scripts/remote-agents-browser.mjs spec/src/current/contracts/security/12-remote-connections-and-agents.md spec/src/current/progress/M27/index.md spec/src/current/progress/M27/history/M27/index.md spec/src/current/verification/sec.md spec/src/current/verification/api.md spec/src/current/verification/ui-04.md spec/src/history/2026/10/12/index.md spec/src/history/2026/10/12/SUMMARY.nav.md spec/src/history/2026/10/12/E-REMOTE-AGENTS-002
```

**Notes**

Changed documents/browser harness formatting passes; final evidence and added public pages checked again before commit. /tmp/jj-remote-real-prettier-check.log.
<!-- /run -->

<!-- run -->
### Run 3

```yaml
result: fail
failures: 7
hasNote: true
noteEncoding: markdown

```

```shell
node scripts/check-progress.mjs
```

**Notes**

Initial new evidence listed two executable harness files as named tests without #names. Corrected to the actual Python qualify entry point; the browser harness remains in files and executable run evidence. Five E-NPM-REGISTRY-001 diagnostics are inherited and unchanged. /tmp/jj-remote-real-progress.log.
<!-- /run -->

<!-- run -->
### Run 4

```yaml
result: fail
failures: 4
hasNote: true
noteEncoding: markdown

```

```shell
node --input-type=module <<'EOF'
import { execFileSync } from 'node:child_process';
const files=execFileSync('jj',['--no-pager','diff','--name-only'],{encoding:'utf8'}).trim().split('\n');
execFileSync(process.execPath,['node_modules/@lapismd/spec-validator/dist/cli.js','first',...files.flatMap(file=>['--file',file])],{stdio:'inherit'});
EOF
```

**Notes**

Explicit file-to-chapter mapping initially required CLI, collaboration, documentation and governance confirmations in addition to security. Added narrow unchanged-contract qualification paragraphs to those owning chapters. Automatic ledger trust gate remains distinct and blocked by inherited E-NPM-REGISTRY-001 diagnostics.
<!-- /run -->

<!-- run -->
### Run 5

```yaml
result: fail
failures: 5
hasNote: true
noteEncoding: markdown

```

```shell
node scripts/check-progress.mjs
```

**Notes**

After correcting new named references, only the five inherited E-NPM-REGISTRY-001 diagnostics remain. New M27 evidence/ledger has no own diagnostic. Historical evidence remains immutable. /tmp/jj-remote-real-progress2.log.
<!-- /run -->

<!-- run -->
### Run 6

```yaml
result: fail
failures: 5
hasNote: true
noteEncoding: markdown

```

```shell
node node_modules/@lapismd/spec-validator/dist/cli.js first
```

**Notes**

Automatic change-set trust gate stops on the same five inherited registry evidence errors. No guard, fixture or ledger validator weakened. /tmp/jj-remote-real-first-auto.log.
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
node --input-type=module <<'EOF'
import { execFileSync } from 'node:child_process';
const files=execFileSync('jj',['--no-pager','diff','--name-only'],{encoding:'utf8'}).trim().split('\n');
execFileSync(process.execPath,['node_modules/@lapismd/spec-validator/dist/cli.js','first',...files.flatMap(file=>['--file',file])],{stdio:'inherit'});
EOF
```

**Notes**

Explicit full changed-file mapping passes: nine protected files, five canonical chapters. This proves chapter coverage only, not the blocked automatic ledger trust gate. /tmp/jj-remote-real-mapping.log.
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

<!-- evidence-field:next markdown -->
## Next

M27 retains installed-provider, Linux real-storage, signed desktop and shared deployment release gates. No real provider credentials used. UI/controller type, interaction and contract suites from E-REMOTE-AGENTS-001 are not repeated: this slice changes only gateway Origin and isolated acceptance/documentation. Root aggregate build/check/test and unrelated Rust/UI suites are out of scope under targeted verification policy. Original development data, services and registry dependencies remain retained.
<!-- /evidence-field -->

<!-- evidence-field:contract_review yaml -->
## Contract review

```yaml
status: unchanged
reason: "JST-SEC-009/JST-API-011/JST-UI-022 remain unchanged: this corrects the
  existing required remote WebSocket Origin path and adds qualification.
  Security clarification explicitly prohibits forwarding local browser
  authority. Public contracts, fixed-origin gateway, same-principal independent
  grants, publication and receipt recovery remain unchanged; no CORS, CSP or
  authorization relaxation. JST-DOC-001 gains an executable disposable-harness
  guide."

```
<!-- /evidence-field -->

<!-- evidence-field:milestone_review yaml -->
## Milestone review

```yaml
status: updated
reason: M27 remains In progress. Credit only macOS two-instance real RustFS and
  cold built-App integration with a deterministic ACP process. Do not promote
  installed providers, Linux real storage, signed desktop or shared deployment
  release. M24/M26 and immutable historical evidence remain unchanged. Inherited
  ledger and architecture diagnostics remain separate.

```
<!-- /evidence-field -->


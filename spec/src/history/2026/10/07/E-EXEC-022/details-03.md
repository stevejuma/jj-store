---
schema: 1
id: E-EXEC-022
---

# E-EXEC-022 details 3

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

Latest explicit development backend binaries built; Vite HMR remained the UI source.
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
env PATH="/Users/stevejuma/.npm/_npx/bb64e4387f65cc68/node_modules/.bin:$PATH" JJ_STORE_ACP_EXECUTABLE=/Users/stevejuma/code/lapismd/jj-acp/target/debug/jj-acp JJ_STORE_AGENT_JJ_EXECUTABLE=/Users/stevejuma/code/lapismd/jj-store/target/debug/jj JJ_STORE_AGENT_PRIVACY_CACHE=/private/tmp/jj-store-agent-fixture-cache-20261006 pnpm dev:backend:restart
```

**Notes**

Only the owned backend was replaced, preserving the existing fixture and Vite. No connected sessions existed before restart. Current chats were not resumed or prompted. Transitional ambient adapter PATH remains for the existing device install until an explicit private installation.
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
pnpm docs:generate
```

**Notes**

Regenerated public installation/type documentation for the new owned interfaces.
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
pnpm docs:review application "Reviewed runtime success chips, shared semantic guide and dialog buttons, Ready install suppression and capture-tab isolation; no chat execution from settings controls"
```

**Notes**

Application source fingerprints reviewed after the final presentation change. Service, HTTP, configuration and public-contract ownership reviews were also refreshed during this slice.
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
cargo fmt --all -- --check
```

**Notes**

Affected Rust source formatting.
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
pnpm docs:check
```

**Notes**

Documentation generation, ownership fingerprints, links, bounds and diagrams.
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
pnpm check:progress; pnpm spec:first
```

**Notes**

Initial authored E-EXEC-022 draft had unexpanded file references, unnamed tests, absent run failure counts and out-of-scope boundaries in remaining. Corrected the evidence metadata using exact files/named tests, explicit counts and excluded_scope; no validator weakened. A second run identified ten generated navigation pages changed after the file list was captured; their exact paths are now recorded with unchanged payload confirmation.
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
pnpm spec:validate
```

**Notes**

7 validators; 3127 chapters, 178 requirements before final evidence metadata correction. Repeat checks after correction are recorded in this enclosing change.
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
pnpm check:progress
```

**Notes**

144 tasks and 646 evidence records; exact file/named-test references and immutable historical payloads validated.
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
pnpm spec:first
```

**Notes**

36 protected files and 9 canonical chapters; owning updates and fresh evidence cover this change.
<!-- /run -->

<!-- run -->
### Run 11

```yaml
result: fail
failures: 13
hasNote: true
noteEncoding: markdown

```

```shell
node --input-type=module <<'JS'
import {execFileSync} from 'node:child_process';
const files=execFileSync('jj',['--no-pager','diff','--name-only'],{encoding:'utf8'}).trim().split('\n').filter(p=>/\.(?:md|json|svelte|css|ts|mjs)$/.test(p));
execFileSync('pnpm',['exec','prettier','--check',...files],{stdio:'inherit'});
JS
```

**Notes**

Generated navigation normalization reintroduced formatting differences in 13 existing milestone/location pages. Final targeted Prettier write/check corrects those without altering historical payloads.
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
node --input-type=module <<'JS'
import {execFileSync} from 'node:child_process';
const files=execFileSync('jj',['--no-pager','diff','--name-only'],{encoding:'utf8'}).trim().split('\n').filter(p=>/\.(?:md|json|svelte|css|ts|mjs)$/.test(p));
execFileSync('pnpm',['exec','prettier','--check',...files],{stdio:'inherit'});
JS
```

**Notes**

Final changed-file formatting check after the navigation update; generated evidence/API documentation remain under their existing formatter exclusion policy.
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

macOS ARM64; Node.js 24.10.0; pnpm 10.32.1; rustc 1.98.1; cargo 1.98.1; Vitest 4.1.11; Storybook 10.5.7. Fixed private Codex catalog: @agentclientprotocol/codex-acp 1.13.1 and @openai/codex 0.156.1. Buzz reference ef2aa1ae38fadcc0bc22b8bf6ed96b35933146be (Apache-2.0), managed_node_paths.rs and agent_discovery.rs read only; Buzz pins its Node runtime but its adapter npm installation is not pinned.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

Additive public version-1 LocalTrusted runtime inventory/install/outcome DTOs, regenerated OpenAPI/TypeScript. Backward-compatible defaulted private journal runtime_selection/runtime_installs, private atomic JSON journal; installation receipts do not change setup CAS revision or session-command receipts. Canonical AgentStore, ThreadStore, ReactionStore, WAL/manifest/object bytes and jj-acp protocol remain unchanged.
<!-- /evidence-field -->

<!-- evidence-field:scope markdown -->
## Scope

Installation-owned Settings → Agents → Runtimes and explicit fixed private Codex npm provisioning. Three LocalTrusted Admin endpoints; fixed packages/registry, no shell or lifecycle scripts, private npm configuration and environment, 120-second deadline, 512-MiB/50,000-entry bounds, executable/version/physical-containment checks and sync before journal selection. Discovery and launch/resume share the selected private bin, ambient PATH and conventional device paths. Interrupted installs never replay; failed updates preserve old selection; unknown durable outcomes retain their original request. Installs refuse while supervised sessions are open. Ready tooling uses green Community success chips, suppresses Install even for unmanaged device tools, and stays separate from agent presence/authentication. UI retains rows and independent request state through refresh and tab navigation. Before user corrections, Ready Codex incorrectly exposed Install and guide/confirmation icons were unstyled. Existing public Design Core and copied Community primitives now provide the presentation. Generated navigation normalized spacing and detail-page labels in existing evidence and unrelated milestone pages; their historical payloads, completion status and curated Later context remain unchanged.
<!-- /evidence-field -->

<!-- evidence-field:next markdown -->
## Next

Maintain the fixed runtime catalog, exact receipts and independently refreshed Settings. Qualify private Node/Cursor provisioning and authenticated cold chat launch/resume separately. Root aggregate pnpm check/test/build/spec:check and full Rust suites are out of scope under targeted policy; affected contract generation, packages, direct settings consumers, cold private installation and browser accessibility were verified.
<!-- /evidence-field -->

<!-- evidence-field:contract_review yaml -->
## Contract review

```yaml
status: changed
reason: Owning JST-UI-012, JST-API-001, JST-MUT-001, JST-COLLAB-031/032/033 and
  JST-CLI-004 topics now govern fixed installation-owned tooling and explicit
  Admin commands. JST-COMPAT-001/003/005 record additive private journal and
  public DTO compatibility. JST-ARCH-001/004 storage independence, manifest
  authority and owned service interfaces remain unchanged. JST-GOV-008 reuses
  owned Community presentation and public Design Core composition without
  aliases, package patches or source imports. JST-DOC-002 guides, generated API
  docs and source reviews updated; JST-GOV-002 actual validation and M23 review
  retained.

```
<!-- /evidence-field -->

<!-- evidence-field:milestone_review yaml -->
## Milestone review

```yaml
status: unchanged
reason: M23 retains its original Done gate qualification. E-EXEC-022 verifies
  maintenance of the installation runtime catalog, private Codex selection,
  receipt recovery and current Settings UI; it does not qualify private Node or
  Cursor provisioning, pooling, native code publication, another harness or
  visual baselines. Next actions and the evidence ledger include these
  boundaries.

```
<!-- /evidence-field -->

<!-- evidence-field:excluded_scope list -->
## Excluded scope

- Strict Clippy qualification remains blocked by unchanged upstream JJ/service
  diagnostics; no warnings suppressed or unrelated code edited.
- Node.js 22+ and npm must already exist; private Node provisioning and Cursor
  installation/update are not implemented. Cursor exposes the official guide.
- No real authenticated current-chat launch/resume acceptance was performed in
  this maintenance slice.
- Production built-bundle/CSP and visual baseline qualification remain separate;
  current-browser acceptance uses Vite HMR. Stories remain visual-pending.
- Agent pooling, other harness installers, native code-publication qualification
  and M15.4 feedback remain separate work.
<!-- /evidence-field -->


---
schema: 1
id: E-PROFILES-001
---

# E-PROFILES-001 details 3

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
pnpm docs:generate
```

**Notes**

Regenerated CLI and HTTP/type references after adding identity commands and directory contracts; final references match owning sources. Prettier changes to generated references were regenerated.
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
cargo fmt --all -- --check
```

**Notes**

Rust formatting passes. Final authored-file formatting excludes generated command/API references.
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
cargo test -p jj-store-server --lib directory::tests --locked
```

**Notes**

Final directory authority regressions: 8 passed, including corruption, identity isolation, owner-only draft media, unique shortcodes, retirement, receipts and restart.
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
cargo test -p jj-store-server --test directory_api --locked
```

**Notes**

Final LocalTrusted/authenticated HTTP tests: 2 passed, including zero-repository access and independent revocation.
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
cargo test -p jj-store-server --lib process::tests::capture_factory::management::cold_foreground_advertises_authenticated_disabled_capture_without_creating_policy --locked -- --exact
```

**Notes**

A new startup audit initially rejected an unreachable external store. Final regression passes after preserving existing startup on transport outage while treating missing selected objects/canonical corruption separately and rejecting them.
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
cargo test -p jj-store-wal --test collab_commands library_snapshot_message_and_reaction_publish_complete_closures_and_replay --locked -- --exact
```

**Notes**

Modern Conversation Submit retains snapshots and reaction assets through publication, cold reopen and exact receipt replay. An initial fixture expected Submit.message, but Submit returns a CommentBatch link; it now reads the actual selected message revision. Final exact regression passes.
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
cargo test -p jj-store-wal --locked
```

**Notes**

Full WAL package suite passed after the modern composer snapshot regression was corrected.
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
cargo check -p jj-store-cli --all-targets --locked
```

**Notes**

CLI and all directly dependent Rust contracts/constructors check successfully.
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
env PATH="/Users/stevejuma/.local/bin:$PATH" pnpm docs:check
```

**Notes**

Strict generated references, documentation review, offline Mermaid rendering and bundled link/anchor closure pass. Bare PATH initially lacked uv; explicit installed uv path repaired execution.
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
cargo test -p jj-collab -p jj-store-service -p jj-store-wal -p jj-store-http -p jj-store-rpc --locked
```

**Notes**

Final complete affected package suites: 518 tests passed, 6 explicitly ignored, across 74 passing test binaries including doc-test groups. This is not a full Rust workspace or server-suite claim.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project catalog-unit web/tests/vite/directory.test.ts web/tests/vite/global-settings-history.test.ts web/tests/vite/production-store-bootstrap.test.ts
```

**Notes**

Final stable run: 12 tests across 3 files passed. Dynamic imports in a test initially consumed its test timeout under concurrent builds; imports now occur before test timing. An SSR-only effect assertion was replaced by real browser credential-switch acceptance.
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
env PATH="/Users/stevejuma/.local/bin:$PATH" node scripts/profile-acceptance.mjs
```

**Notes**

Final real-RustFS built UI acceptance passed in 11.1 seconds (browser test 9.8 seconds): local and authenticated saves/restarts, actual emoji decoding, axe, zero-repository access, CLI identity enrollment, public email omission, and switching between enrolled users without retaining the first private email draft. Initial switch coverage looked for the profile menu inside Settings; the corrected test returns through Back to jj-store before Sign out. Later author-projection extraction has separate story/type acceptance and does not change this Settings lifecycle.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/AgentChatBody.stories.svelte
```

**Notes**

23 chat-body interaction/axe stories passed after owned author-profile projection extraction. Visual approval remains separate.
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
pnpm contracts:check
```

**Notes**

Final generated contracts match; the 300 response corpus, 8 schema tests and contract TypeScript check pass.
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
pnpm spec:validate
```

**Notes**

Final specification passes 7 validators, 3631 chapters and 185 requirements; all evidence pages remain within 400 lines / 32768 bytes.
<!-- /run -->

<!-- run -->
### Run 16

```yaml
result: fail
failures: 5
hasNote: true
noteEncoding: markdown

```

```shell
pnpm check:progress
```

**Notes**

All fresh M26 evidence, named test references and changed-file coverage pass. Only the five inherited E-NPM-REGISTRY-001 diagnostics remain.
<!-- /run -->
<!-- /evidence-field -->

<!-- evidence-field:result markdown -->
## Result

pending
<!-- /evidence-field -->

<!-- evidence-field:remaining list -->
## Remaining

- Independent visual approval remains pending; no baseline changes.
- Full-server capture/provider concurrency qualification remains unresolved; 20
  observed failures before interruption, with focused provider and foreground
  checks recorded separately.
- Inherited architecture/source-budget diagnostics and five E-NPM-REGISTRY-001
  ledger diagnostics remain.
- M15 reducer, replication, other-platform, deployment/release and destructive
  GC gates are separate and unchanged.
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

macOS ARM64; Rust 1.98.1; JJ 0.45.1; Node 26.9.0; pnpm 10.32.1; Svelte 5.57.1; Design Core 0.3.1; Mira 0.0.3; Vitest 4.1.11; Playwright 1.63.0; Storybook 10.5.7; uv 0.10.0; embedded RustFS uses the repository locked runtime.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

New installation directory format 1: typed SHA-256 envelopes and raster bytes selected by installations/&lt;installation&gt;/directory/manifest.json CAS. New secret-free identity request journal format 1. Optional omitted message/submission snapshots and custom emoji provenance preserve legacy canonical bytes. Native manifest and local companion envelope versions remain unchanged.
<!-- /evidence-field -->

<!-- evidence-field:scope markdown -->
## Scope

Enclosing Jujutsu change qwxwqrxp (parent 6063b9bd). Before: no installation profile/emoji APIs or Settings pages, no independent persisted token-to-person binding, and emoji libraries were repository-local. After: private owner email, display name/Markdown description, uploaded/HTTPS/emoji avatar and normalized SHA-256 Gravatar fallback; local admin and enrolled users share Profile settings; custom raster emoji are owner-managed installation-wide with unique active shortcodes. Read-only operations never initialize state. Immutable definitions and asset closures are retained in repository messages/reactions through both native WAL and local companion publication. Independent manage-identities operator policy remains a local server file outside the checkout. Cross-layer contracts changed, so relevant complete UI/model/service/WAL/HTTP/RPC suites were selected, while root aggregates, publishing, installed binaries, deployment resets and visual baselines stayed out of scope. Production components remain independent of Community and catalog checkouts.
Reactive memory-only token changes clear private directory drafts/media and invalidate pending reads. Browser credential switching proves this behavior; SSR unit effects are not substituted for browser lifecycle acceptance. The final author projection extraction is presentation-only and preserves item IDs and source offsets.
<!-- /evidence-field -->

<!-- evidence-field:next markdown -->
## Next

Retain focused and cold acceptance; separately qualify the remaining broad capture/server and visual/release gates.
<!-- /evidence-field -->


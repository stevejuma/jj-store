---
schema: 1
id: E-M13-019
---

# E-M13-019 details 6

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
JJ_STORE_DEV_ROOT=/tmp/jlb-final-zTsELr node scripts/dev-demo-profiles.acceptance.mjs
```

**Notes**

Final demo preservation: 3 deterministic sample project identities, separate code links, native metadata home and 9 sample tasks preserved alongside additional projects. No reseeding. .scratch/local-browser-profiles-qualified.log.
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
CARGO_TARGET_DIR=/Users/stevejuma/code/lapismd/jj-store-local-browser/.scratch/local-browser-runtime JJ_STORE_TEST_FIXTURE_ROOT=/var/folders/_3/k1jv9rtn47s_0k5bm6qn_lk80000gn/T/jj-store-filesystem-browser-BZfoHF JJ_STORE_FILESYSTEM_PORT=7481 JJ_STORE_LOCAL_BROWSER_KEEP=1 node .scratch/reuse-filesystem-final.mjs
```

**Notes**

Final cold fresh filesystem installation: all 3 tests passed. Files/history, recorded sessions/transcript, strict CSP, keyboard/axe; UI/CLI projects/tasks/comments and lost acknowledgement recovery; moved-folder reconnect/removal with unchanged code/companion state. Reused seeded code/agents with fresh state-final; no RustFS/native RPC. .scratch/local-browser-filesystem-qualified.log.
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
JJ_STORE_DEV_PORT=7480 JJ_STORE_DEV_ROOT=/tmp/jlb-final-zTsELr node scripts/dev-demo.mjs stop
```

**Notes**

Owned mixed service stopped and /tmp/jlb-final-zTsELr data preserved. Default port 7460 and workspace remain untouched. Final filesystem service also stopped; retained fixture preserved. Removed only the temporary target symlink, retaining sealed binaries and logs.
<!-- /run -->

<!-- run -->
### Run 4

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

Checker correctly rejected an acceptance entry marked pass with final-governance tasks still in remaining. Product acceptance had passed; remove that completed administration gap after gates, then rerun exact progress check. .scratch/local-browser-check-progress-commit.log.
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
pnpm check:boundaries
```

**Notes**

Final canonical gate passed. .scratch/local-browser-check-boundaries-commit.log.
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
pnpm spec:validate
```

**Notes**

Final canonical gate passed. .scratch/local-browser-spec-validate-commit.log.
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
pnpm check:architecture
```

**Notes**

Final canonical gate passed. .scratch/local-browser-check-architecture-commit.log.
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
pnpm spec:first
```

**Notes**

Final canonical gate passed. .scratch/local-browser-spec-first-commit.log.
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
cargo fmt --all -- --check
```

**Notes**

Final canonical gate passed. .scratch/local-browser-cargo-fmt-all-check-commit.log.
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
pnpm exec prettier --write AGENTS.md scripts/dev-demo-local-folder.mjs scripts/dev-demo-profiles.acceptance.mjs scripts/dev-demo-profiles.mjs scripts/dev-demo-repository.test.mjs scripts/dev-demo.mjs scripts/local-browser-acceptance.mjs spec/src/api.md spec/src/cli.md spec/src/collaboration.md spec/src/local-jj.md spec/src/milestones.md spec/src/projects.md spec/src/security.md spec/src/spec-governance.md spec/src/ui.md web/src/lib/api/local-client.ts web/src/lib/api/local-events.ts web/src/lib/api/repositories.ts web/src/lib/controller/contracts.ts web/src/lib/controller/http-local-code-data-source.ts web/src/lib/controller/http-local-store-data-source.ts web/src/lib/controller/http-local-work-data-source.ts web/src/lib/controller/http-repository-directory.ts web/src/lib/controller/http-store-data-source.ts web/src/lib/controller/local-folder-manager.ts web/src/lib/controller/local-repository-directory.ts web/src/lib/controller/native-activity-reader.ts web/src/lib/controller/native-activity-types.ts web/src/lib/controller/native-agent-context.ts web/src/lib/controller/native-data-source.ts web/src/lib/controller/native-presentation.ts web/src/lib/controller/production-evolution-reader.ts web/src/lib/controller/production-file-pages.ts web/src/lib/controller/production-inspection-pages.ts web/src/lib/controller/production-profile-route.ts web/src/lib/controller/production-projects-hub.svelte.ts web/src/lib/controller/production-projects-integration.ts web/src/lib/controller/production-snapshot.ts web/src/lib/controller/production-store-collaboration.svelte.ts web/src/lib/controller/production-store-conflicts.ts web/src/lib/controller/production-store-controller.svelte.ts web/src/lib/controller/production-store-events.ts web/src/lib/controller/production-store-index.ts web/src/lib/controller/production-store-loaders.ts web/src/lib/controller/production-store-mutations.ts web/src/lib/controller/production-store-state.svelte.ts web/src/lib/controller/production-store-view.ts web/src/lib/controller/production-work.svelte.ts web/src/lib/controller/projects-activity.svelte.ts web/src/lib/controller/projects-hub-types.ts web/src/lib/controller/projects-repository-types.ts web/src/lib/controller/projects-reviews.svelte.ts web/src/lib/controller/repository-profile.ts web/src/lib/controller/repository-read-types.ts web/src/lib/controller/repository-source.ts web/src/lib/controller/review-comment-anchor.ts web/src/lib/controller/store-error.ts web/src/lib/controller/store-query-keys.ts web/src/lib/controller/store-route-codec.ts web/src/lib/controller/work-types.ts web/src/lib/graph.ts web/src/lib/ui/store-application/StoreApplication.svelte web/src/lib/ui/store-application/StoreRepositoryApplication.svelte web/src/lib/ui/store-application/StoreRepositoryTabs.svelte web/src/lib/ui/store-collaboration/StoreAgentContext.svelte web/src/lib/ui/store-collaboration/StoreAgentTranscript.svelte web/src/lib/ui/store-collaboration/StoreCommentableCommitDiff.svelte web/src/lib/ui/store-conflicts/StoreConflictEditor.svelte web/src/lib/ui/store-conflicts/StoreConflictHeaderActions.svelte web/src/lib/ui/store-conflicts/StoreConflictPath.svelte web/src/lib/ui/store-conflicts/model.ts web/src/lib/ui/store-settings/StoreLocalFolders.svelte web/src/lib/ui/store-settings/StoreRepositorySettings.svelte web/src/lib/ui/store-settings/StoreServiceSettings.svelte web/src/lib/ui/store-work/StoreProjectsFilterControls.svelte web/tests/real/local-browser.spec.ts web/tests/real/local-mixed.spec.ts web/tests/real/native-repository.spec.ts web/tests/real/project-polish.spec.ts web/tests/real/projects-activity.spec.ts web/tests/real/work-store.spec.ts web/tests/vite/local-browser.test.ts web/tests/vite/production-store-bootstrap.test.ts
```

**Notes**

All 84 changed Markdown/TypeScript/Svelte/script files formatted and checked. .scratch/local-browser-format-commit-write.log.
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
pnpm exec prettier --check AGENTS.md scripts/dev-demo-local-folder.mjs scripts/dev-demo-profiles.acceptance.mjs scripts/dev-demo-profiles.mjs scripts/dev-demo-repository.test.mjs scripts/dev-demo.mjs scripts/local-browser-acceptance.mjs spec/src/api.md spec/src/cli.md spec/src/collaboration.md spec/src/local-jj.md spec/src/milestones.md spec/src/projects.md spec/src/security.md spec/src/spec-governance.md spec/src/ui.md web/src/lib/api/local-client.ts web/src/lib/api/local-events.ts web/src/lib/api/repositories.ts web/src/lib/controller/contracts.ts web/src/lib/controller/http-local-code-data-source.ts web/src/lib/controller/http-local-store-data-source.ts web/src/lib/controller/http-local-work-data-source.ts web/src/lib/controller/http-repository-directory.ts web/src/lib/controller/http-store-data-source.ts web/src/lib/controller/local-folder-manager.ts web/src/lib/controller/local-repository-directory.ts web/src/lib/controller/native-activity-reader.ts web/src/lib/controller/native-activity-types.ts web/src/lib/controller/native-agent-context.ts web/src/lib/controller/native-data-source.ts web/src/lib/controller/native-presentation.ts web/src/lib/controller/production-evolution-reader.ts web/src/lib/controller/production-file-pages.ts web/src/lib/controller/production-inspection-pages.ts web/src/lib/controller/production-profile-route.ts web/src/lib/controller/production-projects-hub.svelte.ts web/src/lib/controller/production-projects-integration.ts web/src/lib/controller/production-snapshot.ts web/src/lib/controller/production-store-collaboration.svelte.ts web/src/lib/controller/production-store-conflicts.ts web/src/lib/controller/production-store-controller.svelte.ts web/src/lib/controller/production-store-events.ts web/src/lib/controller/production-store-index.ts web/src/lib/controller/production-store-loaders.ts web/src/lib/controller/production-store-mutations.ts web/src/lib/controller/production-store-state.svelte.ts web/src/lib/controller/production-store-view.ts web/src/lib/controller/production-work.svelte.ts web/src/lib/controller/projects-activity.svelte.ts web/src/lib/controller/projects-hub-types.ts web/src/lib/controller/projects-repository-types.ts web/src/lib/controller/projects-reviews.svelte.ts web/src/lib/controller/repository-profile.ts web/src/lib/controller/repository-read-types.ts web/src/lib/controller/repository-source.ts web/src/lib/controller/review-comment-anchor.ts web/src/lib/controller/store-error.ts web/src/lib/controller/store-query-keys.ts web/src/lib/controller/store-route-codec.ts web/src/lib/controller/work-types.ts web/src/lib/graph.ts web/src/lib/ui/store-application/StoreApplication.svelte web/src/lib/ui/store-application/StoreRepositoryApplication.svelte web/src/lib/ui/store-application/StoreRepositoryTabs.svelte web/src/lib/ui/store-collaboration/StoreAgentContext.svelte web/src/lib/ui/store-collaboration/StoreAgentTranscript.svelte web/src/lib/ui/store-collaboration/StoreCommentableCommitDiff.svelte web/src/lib/ui/store-conflicts/StoreConflictEditor.svelte web/src/lib/ui/store-conflicts/StoreConflictHeaderActions.svelte web/src/lib/ui/store-conflicts/StoreConflictPath.svelte web/src/lib/ui/store-conflicts/model.ts web/src/lib/ui/store-settings/StoreLocalFolders.svelte web/src/lib/ui/store-settings/StoreRepositorySettings.svelte web/src/lib/ui/store-settings/StoreServiceSettings.svelte web/src/lib/ui/store-work/StoreProjectsFilterControls.svelte web/tests/real/local-browser.spec.ts web/tests/real/local-mixed.spec.ts web/tests/real/native-repository.spec.ts web/tests/real/project-polish.spec.ts web/tests/real/projects-activity.spec.ts web/tests/real/work-store.spec.ts web/tests/vite/local-browser.test.ts web/tests/vite/production-store-bootstrap.test.ts
```

**Notes**

All 84 changed Markdown/TypeScript/Svelte/script files formatted and checked. .scratch/local-browser-format-commit.log.
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
pnpm check:progress
```

**Notes**

Final acceptance ledger and pending-capability update pass. .scratch/local-browser-check-progress-ledger-final.log.
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
pnpm spec:first
```

**Notes**

Final acceptance ledger and pending-capability update pass. .scratch/local-browser-spec-first-ledger-final.log.
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
pnpm exec prettier --check spec/src/local-jj.md spec/src/milestones.md
```

**Notes**

Final acceptance ledger and pending-capability update pass. .scratch/local-browser-exec-prettier-check-spec-src-local-jj-md-spec-src-milestones-md-ledger-final.log.
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
pnpm exec prettier --write spec/src/milestones.md
```

**Notes**

Final evidence append formatted; 218 historical entries remain semantically unchanged.
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

JJ 0.45.1, Git SHA-1, macOS ARM64.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

Explicit local code/companion snapshot unions; no native identity remapping. Enclosing Jujutsu change zzzswtxu. Native bootstrap test renamed to requested-repository behavior; sole-repository auto-open remains covered by existing units. No bootstrap invariant is waived. Independently qualified browser/collaboration stage; E-M13-019 is pass, M13.4 stays In progress. All required stage acceptance and canonical gates pass; root aggregates remain out of scope.
<!-- /evidence-field -->

<!-- evidence-field:next markdown -->
## Next

Retain qualified browser/collaboration regressions; indexed search and recoverable code commands remain M13.4 gates.
<!-- /evidence-field -->

<!-- evidence-field:scope markdown -->
## Scope

Profile dispatch affects shared repository, Projects, Activity and Agents runtime/transport consumers. Escalation: complete web unit/type consumers, affected screen interaction/accessibility families, and cold filesystem-only plus real-RustFS mixed browser lanes. Local worker waiting additionally requires the exact concurrent HTTP/CLI regression and HTTP contract checks. Root aggregate, unrelated Rust suites and release qualification remain out of scope. Strict CSP is shared browser integration: retain the complete asset test binary plus cold strict-policy dialogs and nonempty inline-style rejection. Private build cache setup failures are environmental and do not qualify product gates. Busy retries touch shared code/AgentStore/WorkStore read transports, so complete Vite consumers, web units/types/build, catalog type consumers and fresh filesystem plus real-RustFS mixed navigation were requalified. Capture keys include distinct refresh generations. Read retries cannot advance pins or replay mutations.
<!-- /evidence-field -->

<!-- evidence-field:deferred list -->
## Deferred

- M13.4 remains In progress; search, code writes, reviews, conversion and sync
  remain deferred; visual baselines are unapproved.
<!-- /evidence-field -->

<!-- evidence-field:date markdown -->
## Date

2026-10-01
<!-- /evidence-field -->

<!-- evidence-field:removed_tests list -->
## Removed tests

- web/tests/real/native-repository.spec.ts#LocalTrusted bootstrap auto-opens
  without browser credentials
<!-- /evidence-field -->


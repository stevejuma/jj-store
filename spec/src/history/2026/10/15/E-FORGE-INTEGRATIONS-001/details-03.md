---
schema: 1
id: E-FORGE-INTEGRATIONS-001
---

# E-FORGE-INTEGRATIONS-001 details 3

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
pnpm spec:search -- JST-FORGE-007; pnpm spec:search JST-FORGE-007; rg -n -i --glob "*.md" JST-FORGE-007 spec/src/current
```

**Notes**

First syntax rejected the literal --; correct syntax found pre-existing unbuilt QMD native modules. Used its documented rg fallback and opened the canonical forge chapter. No broader toolchain repairs.
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
pnpm evidence:new -- --id E-FORGE-INTEGRATIONS-001 --task M31 --requirement JST-FORGE-007 --requirement JST-FORGE-005 --requirement JST-UI-012 --requirement JST-UI-016 --requirement JST-SEC-002 --requirement JST-SEC-006 --requirement JST-API-001
```

**Notes**

Created fresh Markdown evidence and restored 446 unrelated navigation-only changes from @-, retaining new record/day navigation and authored files.
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
pnpm check:progress; pnpm spec:first; pnpm exec spec-validator validate --only fileLimits,governance,summary; changed-file formatting
```

**Notes**

Fresh evidence, milestone review, owning contracts and document limits/navigation validated. Formatting excludes generated evidence, which uses its canonical renderer.
<!-- /run -->

<!-- run -->
### Run 4

```yaml
result: fail
failures: 2
hasNote: true
noteEncoding: markdown

```

```shell
pnpm dev:cargo -- clippy -p jj-forge --lib --locked -- -D warnings
```

**Notes**

Strict Clippy reports two existing collapsible_if diagnostics in unchanged engine.rs at lines 132 and 397. jj diff for engine.rs is empty and the parent revision contains both exact blocks. Deferred these unrelated style fixes under the user test-scope instruction; new account modules emitted no Clippy diagnostics. .local/forge-integrations-clippy.log.
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
Final credential-store review; rustfmt --edition 2024 --check on changed account sources; changed-file Prettier checks
```

**Notes**

Locked credential-store reads now reach the shared error/toast boundary instead of silently reporting unavailable accounts. Atomic metadata replacement no longer tries Unix directory fsync on Windows. Native Windows/Linux acceptance remains unqualified. Seven account and two recovery tests rerun; direct HTTP/native-discovery consumers and backend rebuilt.
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
Focused grouped-check/narrow-account stories, followed by all direct Forge consumers
```

**Notes**

The added scroll/float regression initially failed once; hiding changes alone did not preserve scroll. The owned detachable-panel adapter now retains nested offsets across reparenting, including cleanup after window removal. Exact grouped checks regression: three passed. Final direct consumer rerun: 43 stories across nine files, including 640px Enterprise setup, all five status groups in order, refresh and float/redock/resize retention. A new story used an unsupported testing-library exact option; replaced it with the equivalent anchored accessible-name matcher. Iterative logs .local/forge-integrations-stories-final-regressions.log, scroll-regression.log, scroll-diagnosis.log, scroll-regression-fixed.log, scroll-regression-fixed-2.log; the earlier narrow-story catalog type failure is in catalog-final.log. No assertion or accessibility threshold was relaxed.
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
pnpm dev:cargo -- test -p jj-store --test forge_cli --locked
```

**Notes**

1 passed: cold CLI configuration for GitHub, GitLab and Forgejo preserves the native operation, requires no workspace initialization and keeps transient credentials out of device records/output. .local/forge-integrations-cli-final.log.
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
Final backend-only restart and cold Integrations browser reload
```

**Notes**

Task-owned backend rebuilt/restarted on 7462 while the preserved fixture and Vite 7460 remain active. A reload during startup showed the expected connection-failure toast; a cold reload after readiness rendered Integrations. Previous device code expired before any grant, so a fresh code was requested and entered on GitHub; the same repository/workflow/profile/email permission review remains awaiting the pending explicit confirmation. Native keychain/account/restart-read gate remains pending. .local/forge-integrations-backend-restart-reviewed.log and github-integration-authorization-renewed.png.
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
Visual inspection of new dark-mode previews; focused fixture theme ownership and complete direct-story rerun
```

**Notes**

The application appearance owner superseded Storybook globals. The two affected Forge fixtures now provide isolated appearance stores only for an explicitly requested colorMode; new dark stories explicitly set and assert dark mode, while pre-existing fixture defaults are preserved. All 43 direct Forge stories rerun, with a fresh Vitest/Vite process and no visual baseline updates. .local/grouped-checks-workspace.png records the contextual checks UI.
<!-- /run -->

<!-- run -->
### Run 10

```yaml
result: fail
failures: 2
hasNote: true
noteEncoding: markdown

```

```shell
Trial inherited dark-theme fixture run and focused menu rerun
```

**Notes**

Trial inheritance of Storybook dark globals into all fixture instances exposed an existing unchanged diff-text contrast failure (4.34 versus 4.5) in the older PR header story, plus a menu focus timing failure. The menu exact regression passed on rerun (one passed, eight filtered); no menu assertion changed. Restricted the appearance extension to explicit new-story colorMode inputs, preserving the existing fixture defaults. Fixed initial-prop capture warnings with an explicit untracked initial value. Existing diff-theme qualification/repair remains outside this account/check slice. Trial failure: .local/forge-integrations-stories-themes-final.log; exact menu rerun: .local/forge-integrations-menu-rerun.log.
<!-- /run -->
<!-- /evidence-field -->

<!-- evidence-field:result markdown -->
## Result

pending
<!-- /evidence-field -->

<!-- evidence-field:remaining list -->
## Remaining

- Strict Clippy remains blocked by two existing engine.rs style diagnostics;
  unrelated fixes are deferred.
- Real GitHub permission approval, OS-keychain sign-in and cold authenticated
  restart/read acceptance remain pending user confirmation.
- Existing dark diff-text contrast qualification (4.34 versus 4.5), outside the
  new account/check surface, remains deferred; no diff styles or accessibility
  assertions were changed.
- Windows/Linux native credential qualification, live Enterprise/PAT and
  GitLab/Forgejo qualification, packaged application, native submission and
  visual approval remain separate gates.
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

Node 26.9.0; pnpm 10.32.1; JJ 0.45.1; Rust 1.99.0 (b940084d7 2026-09-28); macOS ARM64; Svelte 5.57.1; Storybook 10.5.7; Vitest 4.1.11/Chromium; released Design Core 0.3.2.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

GitHub account registry is private bounded JSON outside checkout; tokens/refresh credentials are OS-keyring-only, without plaintext fallback. GitHub public OAuth client ID uses backend environment or a public-ID-only device file. Existing forge request v1, plans, Restack plans and device state gain optional opaque account references. Existing identity/endpoint/receipt and code authority contracts remain unchanged; legacy records without account references continue to decode. OpenAPI and TypeScript regenerated.
<!-- /evidence-field -->

<!-- evidence-field:scope markdown -->
## Scope

Global GitHub Integrations with device OAuth, PAT, explicit Enterprise endpoint, account metadata/Forget and native credential custody; canonical remote detection, one-match auto-binding, explicit ambiguity, default branch discovery, session-deduplicated setup toast and account-gated PR actions; account-pinned publication/Restack with pending intent retained. Shared CLI/app orchestration resolves saved credentials without returning them to the browser. Checks stay inside PR details with compact summary/status ring, expandable groups, flat-list option and bounded Scroll Area; logs, stale/incomplete status, capability guards, drafts and floating lifecycle remain retained. Production reuses owned settings/avatar compositions and released primitives. No source/runtime dependency on JJButler, Changeyard or Community checkouts. Scoped tests only; no root aggregate, unrelated suite repair, visual baseline changes or live sample PR mutations.
<!-- /evidence-field -->

<!-- evidence-field:next markdown -->
## Next

Approve the prepared GitHub permission review, then verify saved account, automatic jj-sample-repo binding, PR/check discovery and cold restart. M31 stays In progress.
<!-- /evidence-field -->

<!-- evidence-field:contract_review yaml -->
## Contract review

```yaml
status: changed
reason: Owning forge, Settings, device credential, API, CLI, compatibility, JJ
  integration and local-JJ chapters now specify global saved accounts, native
  secret custody, detection/account-bound external effects, and grouped bounded
  checks. Additive public account-management/request contracts are generated and
  tested; storage, manifest, WAL and native identity are unchanged.

```
<!-- /evidence-field -->

<!-- evidence-field:milestone_review yaml -->
## Milestone review

```yaml
status: unchanged
reason: M31 remains In progress. Focused automated account, native discovery,
  API, controller and browser evidence is recorded; the OAuth app is registered
  but real permission/keychain/restart, platform, packaged/native and visual
  gates are not inferred from fixtures.

```
<!-- /evidence-field -->


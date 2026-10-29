---
schema: 1
id: E-STACKS-DETAILS-VISIBILITY-001
---

# E-STACKS-DETAILS-VISIBILITY-001 details 3

<!-- evidence-field:runs runs -->
## Validation

<!-- run -->
### Run 1

```yaml
result: fail
failures: 2
hasNote: true
noteEncoding: markdown

```

```shell
pnpm spec:validate
```

**Notes**

Evidence writer created two detail fragments before their navigation entries were regenerated. Regenerated task histories/navigation and restored unrelated generated changes before rerun.
<!-- /run -->

<!-- run -->
### Run 2

```yaml
result: fail
failures: 3
hasNote: true
noteEncoding: markdown

```

```shell
git diff --check; pnpm exec prettier --write spec/src/history/2026/10/16/E-STACKS-DETAILS-VISIBILITY-001/*.md
```

**Notes**

Read-only whitespace inspection found three generated trailing blank lines. History is formatter-ignored; final normalization trims these new evidence fragments manually without changing historical payloads.
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
pnpm progress:history; pnpm spec:validate
```

**Notes**

Regenerated histories/navigation, restored 461 unrelated generated paths, formatted the current M21 pages and validated all 7 document validators (4062 chapters, 205 requirements).
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

The newly regenerated M21.1 history also requires fresh evidence file coverage. Added the actual changed history/navigation paths to this record before rerunning.
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
cd ../design-core && pnpm checks
```

**Notes**

Complete required nonvisual source aggregate passes: specification validation/book/spec-first, repository formatting, all four type/consumer checks (zero errors/warnings), no-utility check, 746 unit tests, 3 Turbo tests, 16 release/tooling tests, static Storybook build/metadata audit, all 764 Storybook interaction/accessibility cases, 15 workspace pointer tests, 11 shell/mobile/docs tests, 3 AI browser tests, and 89 shared-control Chromium/WebKit browser tests. One Chromium instance of a WebKit-only case is intentionally skipped. No screenshot comparison or baseline update.
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
cd ../design-core && pnpm release:check
```

**Notes**

Release configuration is valid; no publication or remote push performed.
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
pnpm check:progress; pnpm spec:validate; pnpm exec prettier --check catalog/components/store-stack-workspace/StoreStackWorkspaceApplication.stories.svelte spec/src/current/contracts/ui/04-local-stack-workspace-catalog.md spec/src/current/progress/M21/index.md
```

**Notes**

Corrected record coverage passes at 156 tasks/846 records; all 7 specification validators and changed-current-page formatting pass. The new evidence fragments are trimmed to a single final newline.
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
pnpm check:progress; pnpm spec:first; git diff --check; pnpm exec prettier --check catalog/components/store-stack-workspace/StoreStackWorkspaceApplication.stories.svelte spec/src/current/contracts/ui/04-local-stack-workspace-catalog.md spec/src/current/progress/M21/index.md
```

**Notes**

Final completed record: progress passes for 156 tasks/846 records; spec-first passes for the protected story and canonical chapter; whitespace and changed-current-page formatting checks pass. No broader jj-store UI or Rust suite is claimed.
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

Owning Design Core visibility repair plus application regression. Community shell source was inspected first, then Design Core. Local packaged consumer validation is separate from registry integration: source release and a registry-only consumer dependency update remain the next delivery step. Broad jj-store/Rust/build/release lanes are out of scope under targeted policy. Screenshot comparison and baseline updates are excluded under the user preference; source functional, keyboard, accessibility, type, package and required nonvisual checks remain selected. The selected local repair and packaged review acceptance are complete. Publication is excluded by the explicit development-slice rule and requires separate user authorization; normal registry installation still resolves 0.5.1 until that release. The consumer regression is intentionally retained at full strength for release integration.
<!-- /evidence-field -->

<!-- evidence-field:next markdown -->
## Next

Publish the prepared Design Core patch through the authorized standard release flow, update jj-store to its registry version, remove the temporary ignored validation override and rerun the exact Settings round-trip regression.
<!-- /evidence-field -->

<!-- evidence-field:contract_review yaml -->
## Contract review

```yaml
status: unchanged
reason: JST-UI-012 and JST-UI-014 already require Settings to hide Details and
  restore its prior state. JST-UI-020 retains focused Stacks context. The
  canonical Stacks text makes the zero-width closed state explicit; the source
  repair restores the existing contract without new commands, authority or
  formats.

```
<!-- /evidence-field -->

<!-- evidence-field:milestone_review yaml -->
## Milestone review

```yaml
status: unchanged
reason: M21.1 remains Partial. This focused layout regression does not qualify
  provider/backend, packaged-platform, registry-release or visual gates.

```
<!-- /evidence-field -->

---
schema: 1
id: E-REMOTE-AGENTS-001
---

# E-REMOTE-AGENTS-001 details 4

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
node scripts/remote-connection-browser.mjs
```

**Notes**

Final cold App fixture includes remote Settings navigation: local administration categories are absent. Extracted mobile/settings callbacks preserve controller owners and default remote entry chooses Profile. /tmp/jj-remote-cold-browser7.log.
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
PATH="/Users/stevejuma/.local/bin:$PATH" node scripts/docs.mjs build
```

**Notes**

Final reviewed source fingerprints and strict link/anchor closure pass after the remote Settings refinement. /tmp/jj-remote-docs-final2.log.
<!-- /run -->

<!-- run -->
### Run 3

```yaml
result: fail
failures: 5
hasNote: true
noteEncoding: markdown

```

```shell
node node_modules/vitest/vitest.mjs run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/SettingsNavigation.stories.svelte catalog/components/store-application/ServerConnections.stories.svelte catalog/components/store-application/ServerConnectionOwners.stories.svelte
```

**Notes**

Final remote-administration filtering exposed five fixture failures: Local service expectations used the generic authenticated profile. Added optional accessMode to the owned catalog fixture/controller and explicitly declared LocalTrusted in those installation Settings stories. Production restrictions and assertions remain intact; other fixture consumers keep their authenticated default. Initial count was 5 failed / 10 passed.
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
cargo fmt --all -- --check
```

**Notes**

Final Rust formatting passes, including the documented remote-agent parser regression.
<!-- /run -->

<!-- run -->
### Run 5

```yaml
result: fail
failures: 6
hasNote: true
noteEncoding: markdown

```

```shell
node node_modules/vitest/vitest.mjs run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/SettingsNavigation.stories.svelte catalog/components/store-application/ServerConnections.stories.svelte catalog/components/store-application/ServerConnectionOwners.stories.svelte catalog/components/store-application/StoreApplication.stories.svelte
```

**Notes**

The three focused settings/connection files pass all 15 cases. Extended shell family adds 6 passes and 6 inherited failures: total 21 passed / 6 failed. SettingsNavigation fixture regressions are resolved; all six older shell Settings cases also fail on the parent. Current remote default shows Profile, so some old scenarios now stop earlier. /tmp/jj-remote-settings-stories-final2.log.
<!-- /run -->

<!-- run -->
### Run 6

```yaml
result: fail
failures: 6
hasNote: true
noteEncoding: markdown

```

```shell
node node_modules/vitest/vitest.mjs run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/StoreApplication.stories.svelte
```

**Notes**

Executed in a temporary Jujutsu workspace at parent qwxwqrxp: 6 passed / 6 failed. Failures are Settings, SettingsCollapsed, SettingsDegradedSearch, SettingsUnavailableSearch, SettingsStatusFailure and SettingsMobile. Parent assertions still expect Repository token and obsolete global Repository/Search controls. Temporary fs.allow admitted linked released node_modules only; no source assertions changed. Baseline workspace was forgotten and removed after verification. /tmp/jj-remote-parent-shell-stories.log.
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
node node_modules/vitest/vitest.mjs run --config catalog/vitest.config.ts --project catalog-unit catalog/controller/store-controller.test.ts
```

**Notes**

Six default-controller regressions pass after the optional explicit fixture profile. Existing authenticated default, independent ownership and navigation remain intact. /tmp/jj-remote-fixture-default-tests.log.
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
node node_modules/svelte-check/bin/svelte-check --tsconfig catalog/tsconfig.json --fail-on-warnings
```

**Notes**

Final optional fixture profile and all stories compile with zero errors/warnings. /tmp/jj-remote-catalog-final7.log.
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
node node_modules/svelte-check/bin/svelte-check --tsconfig web/tsconfig.json
```

**Notes**

Final production connection/settings refinements compile with zero errors/warnings. /tmp/jj-remote-web-final12.log.
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
node node_modules/@lapismd/spec-validator/dist/cli.js validate --only summary,verification,ui-ownership,governance,fileLimits
```

**Notes**

Five focused source/ownership/document validators pass. /tmp/jj-remote-spec-final.log. Rechecked after final evidence generation before commit.
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
node --input-type=module <<'JS'
import {execFileSync} from 'node:child_process';
const files=execFileSync('jj',['--no-pager','diff','--name-only'],{encoding:'utf8'}).trim().split('\n');
execFileSync(process.execPath,['node_modules/@lapismd/spec-validator/dist/cli.js','first',...files.flatMap(f=>['--file',f])],{stdio:'inherit'});
JS
```

**Notes**

Complete explicitly selected source-to-chapter mapping passes. This validates mapped chapter coverage only; automatic ledger trust remains blocked by inherited evidence errors.
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
const files=execFileSync('jj',['--no-pager','diff','--name-only'],{encoding:'utf8'}).trim().split('\n').filter(f=>/\.(md|json|ts|svelte|mjs)$/.test(f));
execFileSync(process.execPath,['node_modules/prettier/bin/prettier.cjs','--check',...files],{stdio:'inherit'});
JS
```

**Notes**

Changed authored JS/TS/Svelte/JSON/Markdown formatting passes; governed evidence pages use their bounded writer. No visual baseline changed.
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
node node_modules/vitest/vitest.mjs run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/SettingsNavigation.stories.svelte catalog/components/store-application/ServerConnections.stories.svelte catalog/components/store-application/ServerConnectionOwners.stories.svelte
```

**Notes**

Exact focused family rerun passes all 15 cases after the explicit LocalTrusted fixture correction. /tmp/jj-remote-settings-stories-verified.log.
<!-- /run -->

<!-- run -->
### Run 14

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
node --input-type=module <<'JS'
import {execFileSync} from 'node:child_process';
const files=execFileSync('jj',['--no-pager','diff','--name-only'],{encoding:'utf8'}).trim().split('\n').filter(f=>/\.(md|json|ts|svelte|mjs)$/.test(f));
execFileSync(process.execPath,['node_modules/prettier/bin/prettier.cjs','--check',...files],{stdio:'inherit'});
JS
```

**Notes**

Changed-file formatting check found that the last documentation review writer used plain JSON indentation. Re-formatted coverage.json and reran the complete changed-file formatter check; no fingerprint or source behavior changed.
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
node --input-type=module <<'JS'
import {execFileSync} from 'node:child_process';
const files=execFileSync('jj',['--no-pager','diff','--name-only'],{encoding:'utf8'}).trim().split('\n').filter(f=>/\.(md|json|ts|svelte|mjs)$/.test(f));
execFileSync(process.execPath,['node_modules/prettier/bin/prettier.cjs','--check',...files],{stdio:'inherit'});
JS
```

**Notes**

Final entire changed-file formatting check passes after coverage metadata formatting. /tmp/jj-remote-prettier-verified.log. No further source changes were made.
<!-- /run -->
<!-- /evidence-field -->

<!-- evidence-field:result markdown -->
## Result

pending
<!-- /evidence-field -->

<!-- evidence-field:remaining list -->
## Remaining

- Cold two-instance real-RustFS deployment, installed ACP/provider and
  production built-CSP/desktop acceptance are not qualified by
  MemoryStore/source/browser fixtures.
- Linux source admission/delegation pass independently; positive custom-JJ
  execution and installed-runtime/platform delivery remain unqualified there.
- "Three pre-existing AgentExecution Add-agent-dialog stories fail identically
  on the parent: CommunityAvatarAndHarnessPickers, StaleSaveRetainsDraft and
  ConfigurationPreparationRetainsDraft. Current affected family 3 failed / 33
  passed; parent family 3 failed / 11 passed. Catalog dialog owners retain this
  separate backlog."
- Inherited architecture diagnostics and five E-NPM-REGISTRY-001 ledger
  diagnostics remain. Historical evidence is immutable; no dependency hook,
  baseline or checker was weakened.
- Root progress-history/navigation regeneration is blocked by inherited
  oversized E-PROJECT-ROWS-001 history. New M27 history and affected bounded
  navigation are refreshed narrowly, preserving old records.
- Root aggregate tests/build/spec check were out of scope under the targeted
  policy; no full-suite, release or visual approval claim is made.
- Automatic spec-first remains blocked by inherited ledger trust errors; an
  explicit complete file-to-chapter mapping check passes separately. Generated
  spec-book HTML acceptance is not claimed.
- Six obsolete StoreApplication shell Settings stories also fail on the
  unchanged parent; they belong to the earlier global/repository Settings
  separation backlog. Final combined family is 21 passed / 6 failed, with 15
  focused connection/settings cases passing.
<!-- /evidence-field -->

<!-- evidence-field:next markdown -->
## Next

Qualify the remaining M27 installed-runtime, real-storage and Linux/macOS delivery gates before enabling a deployment; address inherited dialog/ledger/size diagnostics in separate governed slices.
<!-- /evidence-field -->


---
schema: 1
id: E-VAULT-HISTORY-FRONTIER-003
---

# E-VAULT-HISTORY-FRONTIER-003

<!-- evidence-field:id markdown -->
## Id

E-VAULT-HISTORY-FRONTIER-003
<!-- /evidence-field -->

<!-- evidence-field:date markdown -->
## Date

2026-11-15
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M34.6
<!-- /evidence-field -->

<!-- evidence-field:requirements list -->
## Requirements

- JST-LOCAL-007
- JST-JJ-005
- JST-UI-002
- JST-DOC-002
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

verification
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- package.json
- pnpm-lock.yaml
- spec/src/current/contracts/architecture/06-browser-presentation-ownership.md
- spec/src/current/contracts/ui/11-shell-composition.md
- spec/src/current/progress/M34/index.md
- spec/src/current/verification/local-03.md
- spec/src/history/2026/11/15/SUMMARY.nav.md
- spec/src/history/2026/11/15/index.md
- catalog/components/store-application/ApplicationMenu.stories.svelte
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

[]
<!-- /evidence-field -->

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
node scripts/validation.mjs run test:storybook catalog/components/store-application/ApplicationMenu.stories.svelte -t NativeChangeDraftSurvivesMobileToDesktopRelocation
```

**Notes**

Before: published Core0.13.3 failed0/1 in immutable receipt002, report ffd67554-bb8c-45fe-b498-8af69e1a00f6. After: normal registry Core0.13.4 passes1/1; six other family cases excluded by exact filter, not qualified. Report jj-store-validation-63a7237f-b2f3-4fb2-98cf-803a3972a608. Assertion and story source unchanged.
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
node scripts/validation.mjs run test:storybook
```

**Notes**

All four fresh browser shards pass:273+284+238+252=1047tests across216files;2existing governed quarantines remain separate. ApplicationMenu full family passes7/7, including original focus target. Report jj-store-validation-8dd3f245-a45c-46dc-9024-7efecce968fe.
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
node scripts/validation.mjs run web:check
```

**Notes**

Svelte0errors/0warnings. Report jj-store-validation-d47c77cb-7389-4a01-9622-42220be43a01.
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
node scripts/validation.mjs run catalog:check
```

**Notes**

Svelte0errors/0warnings; catalogue/parity checks pass. Report jj-store-validation-8e851369-4b9f-4a3f-af8a-efc96d83c052.
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
node scripts/validation.mjs run web:test
```

**Notes**

180/180. Report jj-store-validation-2b3cc75f-eee7-4b39-a673-0f131ddfee48.
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
node scripts/validation.mjs run catalog:test
```

**Notes**

1221passed/0failed,1existing skipped case across193files. Report jj-store-validation-54115c8f-ec36-44a5-9c3c-1b1aebe721ee.
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
node scripts/validation.mjs run web:test:browser
```

**Notes**

Cold production build/browser114passed/0failed,1existing skipped case; skip is not qualification. Report jj-store-validation-07a99a8f-de87-4efe-ba57-4635c84d562f.
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
node scripts/validation.mjs run catalog:test:browser
```

**Notes**

Static Storybook build and cold served-catalogue browser1/1. Report jj-store-validation-7aa15b6a-1215-4455-9c24-09948bbc04f2.
<!-- /run -->

<!-- run -->
### Run 9

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
node scripts/validation.mjs run check:progress
```

**Notes**

The ledger recognizer accepts fn/def/test declarations, not generated Svelte Storybook exports. Its unsupported CamelCase test reference was removed; actual UI source is listed in Files and exact scenario/command/report remain in Run1, matching existing UI receipts. No parser or gate changed. Report jj-store-validation-e0113903-570d-4312-9295-66afcca9e6ce.
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
node scripts/validation.mjs run check:progress
```

**Notes**

187tasks/1061evidence records; bookkeeping only. Report jj-store-validation-d226cc4b-f919-43a7-94b4-e88b1e9d9375.
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
node scripts/validation.mjs run spec:validate
```

**Notes**

8validators/4678chapters/212requirements; zero findings. Report jj-store-validation-3759ef51-8407-48e2-870b-048a79b28c67.
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
node scripts/validation.mjs run docs:check
```

**Notes**

144documentationfiles; links/anchors valid. Report jj-store-validation-a7e9fd4a-079a-4d2d-a52f-7374d4b05730.
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
node scripts/validation.mjs run spec:first
```

**Notes**

2protectedfiles/2canonicalchapters pass. Report jj-store-validation-987edf28-ca56-458e-bbb6-21d3690b3d7b.
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
node scripts/validation.mjs run spec:validate
```

**Notes**

Recording the final runs split the new receipt into two bounded pages; the new details page lacked its global SUMMARY entry. Added that one nested date-navigation link, without running global navigation generation. Report jj-store-validation-0e050ee2-8758-4f26-b251-7a00d8cb54fb.
<!-- /run -->
<!-- /evidence-field -->

<!-- evidence-field:result markdown -->
## Result

pending
<!-- /evidence-field -->

<!-- evidence-field:remaining list -->
## Remaining

- Green hosted required checks on the submitted new head before merge.
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

Design Core0.13.4 published from merged c1e19ba19fda5d608d7ab788694056b5425b4856; release43734446608 and all11 main CI checks pass. Registry tarballSHA256936d2bca9002998fbcc5d249ef5f953f01c3c3e09e23d6bbdd4f8d35e10439cb; npm signatures46verified/0invalid/0missing, including this package and SLSA provenance. Source candidate65621b... differs only in gzip OS header byte9; uncompressed tarSHA256aa637d6d8a37abdaab8054bf04c77809a0c0e7da8c6e93935743ed20911c94bb is identical across4518files. Node24.15.0 host,pnpm10.32.1, managedLinuxDocker.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

Only normal npm semver minimum and registry integrity change; native JJ/protocol/persistence formats and Rust source are unchanged.
<!-- /evidence-field -->

<!-- evidence-field:scope markdown -->
## Scope

User separately authorized Design Core0.13.4 publication after green CI. The existing protected npm environment was approved without changing protections. Registry-only frozen consumer intake replaces the previously blocked published0.13.3. No source alias, pnpm hook, archive overlay, node_modules edit, assertion waiver or visual baseline update is used. Historical receipts001/002 remain immutable. Complete Storybook is required because the foundational shell dependency is shared by application/catalog surfaces; relevant web/catalog types, units and cold browser builds qualify its consumer impact. Earlier unchanged Rust verification remains source evidence; this dependency slice does not qualify Sync, OS custody or installed applications. Generated Svelte Storybook case names are recorded in the exact run command and report, following existing E-APPLICATION-MENU-001/E-SHELL-LAYOUT-001 evidence conventions; the native/plain named-test list is empty. Excluded coverage remains explicit: existing Storybook Q-STORY-001/Q-STORY-002, cold-browser Q-BROWSER-001 geometry, and the opt-in working-tree observation validator without JJ_STORE_WORKING_OBSERVATION_CAPTURE. None is counted as passing; no quarantine, environment or assertion is changed.
<!-- /evidence-field -->

<!-- evidence-field:next markdown -->
## Next

Push the verified npm intake to PR86, retain its watch and merge only after all required checks pass.
<!-- /evidence-field -->


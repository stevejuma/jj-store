---
schema: 1
id: E-DEPENDENCY-AUTOMATION-006
---

# E-DEPENDENCY-AUTOMATION-006

<!-- evidence-field:id markdown -->
## Id

E-DEPENDENCY-AUTOMATION-006
<!-- /evidence-field -->

<!-- evidence-field:date markdown -->
## Date

2026-11-18
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M30
<!-- /evidence-field -->

<!-- evidence-field:requirements list -->
## Requirements

- JST-ARCH-005
- JST-DOC-002
- JST-GOV-002
- JST-GOV-011
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

verification
<!-- /evidence-field -->

<!-- evidence-field:revision markdown -->
## Revision

Enclosing Jujutsu change xxyqpvswwwrppwswzoxzpoknwvwxtozs
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- .github/dependabot.yml
- .github/workflows/publish-release.yml
- Cargo.lock
- crates/jj-store-jj/Cargo.toml
- crates/jj-store-read-index/Cargo.toml
- docs/maintenance/coverage.json
- docs/releases.md
- scripts/automation-governance.test.mjs
- spec/src/current/contracts/adapters/12-custom-data-read-indexes.md
- spec/src/current/contracts/architecture/22-custom-data-indexes.md
- spec/src/current/contracts/architecture/23-public-distribution.md
- spec/src/current/contracts/documentation/02-requirements.md
- spec/src/current/contracts/jj-integration/01-native-jj-integration.md
- spec/src/current/governance/spec-governance.md
- spec/src/current/progress/M30/index.md
- spec/src/history/2026/11/18/E-DEPENDENCY-AUTOMATION-006/SUMMARY.nav.md
- spec/src/history/2026/11/18/E-DEPENDENCY-AUTOMATION-006/index.md
- spec/src/history/2026/11/18/SUMMARY.nav.md
- spec/src/history/2026/11/18/index.md
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- scripts/automation-governance.test.mjs#Dependabot keeps the contracted
  Changesets action pin and version-independent index sources
- scripts/ci-workflows.test.mjs#Changesets CLI v2 keeps its compatible release
  action
<!-- /evidence-field -->

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
GitHub Actions runs for dependabot/github_actions/actions-eb11efd4e4 and dependabot/cargo/rust-c918501b34
```

**Notes**

PR 93 failed Tooling regression tests because Dependabot replaced the contracted
Changesets action v1.9.0 pin. PR 94 failed UI acceptance inputs with
Documentation review required: custom-data-indexes after the read-index
Cargo.toml pin entered the fingerprint map.
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
node --test --test-name-pattern='Dependabot keeps the contracted Changesets action pin|Changesets CLI v2 keeps its compatible release action' scripts/automation-governance.test.mjs scripts/ci-workflows.test.mjs
pnpm docs:review custom-data-indexes "Reviewed redb 4.6 against version-independent custom index docs; fingerprint implementation sources only so Dependabot Cargo pins no longer invalidate the acknowledgement."
```

**Notes**

The combined maintainer change keeps Changesets action v1.9.0, ignores further
Dependabot proposals for that action, bumps setup-python plus redb/prost, and
maps custom-data-indexes to implementation sources only.
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

actions/setup-python 7.1.0; changesets/action 1.9.0; redb 4.6.0; prost 0.14.6;
pnpm 10.32.1.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

No storage, API, route or publication format changes. Documentation review now
fingerprints `crates/jj-store-read-index/src` without the Cargo.toml pin.
<!-- /evidence-field -->

<!-- evidence-field:scope markdown -->
## Scope

Combine the open compatible Dependabot bumps, preserve the Changesets release
pin, and stop recurring bot CI failures from that pin and version-independent
index fingerprint mapping.
<!-- /evidence-field -->

<!-- evidence-field:next markdown -->
## Next

Open the combined pull request, close Dependabot PRs 93 and 94, and require fresh
hosted CI on the maintainer branch.
<!-- /evidence-field -->

<!-- evidence-field:contract_review yaml -->
## Contract review

```yaml
status: changed
reason: JST-ARCH-005 keeps Changesets action v1.9.0 and now states Dependabot
  ignores replacements. JST-DOC-002 still invalidates mapped source changes; the
  custom-data-indexes map now tracks implementation sources rather than
  dependency pins for version-independent operator docs.
```
<!-- /evidence-field -->

<!-- evidence-field:milestone_review yaml -->
## Milestone review

```yaml
status: unchanged
reason: M30 remains In progress. This dependency maintenance does not qualify
  publication, provider, platform or visual gates.
```
<!-- /evidence-field -->

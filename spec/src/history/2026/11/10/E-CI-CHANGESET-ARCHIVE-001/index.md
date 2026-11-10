---
schema: 1
id: E-CI-CHANGESET-ARCHIVE-001
---

# E-CI-CHANGESET-ARCHIVE-001

<!-- evidence-field:id markdown -->
## Id

E-CI-CHANGESET-ARCHIVE-001
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M30
<!-- /evidence-field -->

<!-- evidence-field:requirements list -->
## Requirements

- JST-GOV-004
<!-- /evidence-field -->

<!-- evidence-field:date markdown -->
## Date

2026-11-10
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

verification
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- spec/archive/sources.json
- spec/archive/2026-11-10/changesets/admitted-history-bridge.source
- spec/archive/2026-11-10/changesets/ci-cold-browser-freshness.source
- spec/archive/2026-11-10/changesets/core-ui-acceptance.source
- spec/archive/2026-11-10/changesets/crate-overviews.source
- spec/archive/2026-11-10/changesets/desktop-browser-development.source
- spec/archive/2026-11-10/changesets/headless-capture.source
- spec/archive/2026-11-10/changesets/history-wire-integers.source
- spec/archive/2026-11-10/changesets/injectable-collaboration-stores.source
- spec/archive/2026-11-10/changesets/injected-prompt-coordination.source
- spec/archive/2026-11-10/changesets/linux-vault-directory-sync.source
- spec/archive/2026-11-10/changesets/local-history-profile.source
- spec/archive/2026-11-10/changesets/main-local-qualification.source
- spec/archive/2026-11-10/changesets/native-agent-privacy.source
- spec/archive/2026-11-10/changesets/remote-test-audit-repairs.source
- spec/archive/2026-11-10/changesets/repository-removal.source
- spec/archive/2026-11-10/changesets/reusable-session-acquisition.source
- spec/archive/2026-11-10/changesets/reusable-workspace-discovery.source
- spec/archive/2026-11-10/changesets/search-save-notice.source
- spec/archive/2026-11-10/changesets/shared-conversation-stream.source
- spec/archive/2026-11-10/changesets/shared-read-admission.source
- spec/archive/2026-11-10/changesets/standalone-companion-provider.source
- spec/archive/2026-11-10/changesets/typed-conversation-client.source
- spec/archive/2026-11-10/changesets/typed-history-client.source
- spec/src/current/governance/02-requirements.md
- spec/src/current/progress/M30/index.md
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
sha256sum spec/archive/2026-11-10/changesets/*.source
node scripts/spec/progress-validation.mjs
pnpm spec:first --base a630e97f23a78572b3cc11619e1540f5fabf2ad8 --head @
```

**Notes**

Before the archive, the mapped spec-first gate rejected all 23 consumed
Changesets paths as missing evidence files. Each archive byte stream matches the
last pre-release source at revision 9ccd2e327e49, its recorded SHA-256 matches,
progress integrity passes and spec-first can read the immutable evidence without
reintroducing consumed release intent.
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

jj-store 0.2.0 release main at a630e97f23a7; archive reader schema unchanged.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

Existing `spec/archive/sources.json` archive entries with exact SHA-256. No
historical evidence payload, Changesets release intent or package version is
changed.
<!-- /evidence-field -->

<!-- evidence-field:scope markdown -->
## Scope

Archive the 23 Changesets source files consumed by the 0.2.0 version release so
immutable evidence remains verifiable after normal Changesets deletion. The
files stay absent from `.changeset` and cannot be released again.
<!-- /evidence-field -->

<!-- evidence-field:next markdown -->
## Next

Require future version automation to archive consumed evidence inputs before
deletion, then retain the ordinary M30 release gates.
<!-- /evidence-field -->

<!-- evidence-field:contract_review yaml -->
## Contract review

```yaml
status: changed
reason: JST-GOV-004 now explicitly requires byte-exact archival of consumed
  release inputs referenced by immutable evidence before their normal deletion.
```
<!-- /evidence-field -->

<!-- evidence-field:milestone_review yaml -->
## Milestone review

```yaml
status: unchanged
reason: M30 remains In progress at its existing Cargo, native bundle, Pages and
  authenticated GHCR release gates. Repairing evidence closure does not qualify
  or publish a release.
```
<!-- /evidence-field -->

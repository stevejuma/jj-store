---
schema: 1
id: E-PUBLIC-RELEASE-001
---

# E-PUBLIC-RELEASE-001 details 2

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
node --test scripts/release.test.mjs scripts/ci.test.mjs
```

**Notes**

Five release/CI tooling tests passed before the automation-governance extension.
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
node scripts/package-check.mjs
```

**Notes**

All 20 package metadata and contained-input checks passed; this is not Cargo package compilation or publication qualification.
<!-- /run -->

<!-- run -->
### Run 3

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

Initial release scaffold had no E-PUBLIC-RELEASE-001 record or fresh evidence coverage; this record repairs that bookkeeping. Diagnostics include same-change ledger failures and missing coverage, not product-test failures.
<!-- /run -->
<!-- /evidence-field -->

<!-- evidence-field:result markdown -->
## Result

pending
<!-- /evidence-field -->

<!-- evidence-field:remaining list -->
## Remaining

- Complete release qualification and fix artifact reuse, cold CI fixtures,
  package archive verification and immutable image promotion.
- Review retained history audit findings before public source publication.
- Create GitHub repository, enable Pages/security settings, publish and
  externally verify Cargo/native/GHCR artifacts only after qualification.
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

Coordinated 0.1.0; Rust 1.99.0; scanner 1.4.0; private npm coordination package.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

Native identities, manifest/WAL authority and read pins unchanged. Distribution naming and privacy scanner/cache versions are governed by JST-ARCH-005.
<!-- /evidence-field -->

<!-- evidence-field:scope markdown -->
## Scope

Public release infrastructure and portable package preparation remain in progress. Compiler lint fixes preserve assertions and authority. Earlier qualification logs show 68 governance tests, 26 spec tests, full all-target workspace lint and ten isolated nextest replay tests passed; these have not been rerun in this bookkeeping repair.
<!-- /evidence-field -->

<!-- evidence-field:next markdown -->
## Next

Implement bot governance exclusions, then complete all concrete release acceptance gaps before publication.
<!-- /evidence-field -->

<!-- evidence-field:contract_review yaml -->
## Contract review

```yaml
status: changed
reason: Owning architecture/public installation and affected integration topics
  describe the new public distribution contracts. Native persistence and
  authority remain unchanged.

```
<!-- /evidence-field -->

<!-- evidence-field:milestone_review yaml -->
## Milestone review

```yaml
status: unchanged
reason: M30 remains In progress. Partial local tooling and compiler evidence
  does not complete release, container or hosted CI gates; M29 status is
  retained.

```
<!-- /evidence-field -->


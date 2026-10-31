---
schema: 1
id: E-CONVERSATIONS-NPM-006
---

# E-CONVERSATIONS-NPM-006

<!-- evidence-field:id markdown -->
## Id

E-CONVERSATIONS-NPM-006
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M30
<!-- /evidence-field -->

<!-- evidence-field:requirements list -->
## Requirements

- JST-ARCH-005
<!-- /evidence-field -->

<!-- evidence-field:date markdown -->
## Date

2026-10-31
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

verification
<!-- /evidence-field -->

<!-- evidence-field:revision markdown -->
## Revision

Enclosing Jujutsu change lzwmsotsuxrmkynrumokxltwznqpmzmr
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- .github/workflows/npm-publish-jj-conversations.yml
- docs/maintenance/coverage.json
- docs/site/technical/conversations.md
- scripts/conversations-npm-artifact.mjs
- scripts/conversations-npm-plan.mjs
- scripts/conversations-npm.mjs
- scripts/conversations-npm.test.mjs
- spec/src/current/contracts/architecture/23-public-distribution.md
- spec/src/current/contracts/documentation/03-delivery-and-validation.md
- spec/src/current/governance/spec-governance.md
- spec/src/current/progress/M30/index.md
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- scripts/conversations-npm.test.mjs#downloaded npm package must match size, SHA-256 and integrity, with or without provenance
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
node --test scripts/conversations-npm.test.mjs
```

**Notes**

Seven tests passed on Node 24.10.0. A matching tarball succeeds with or without provenance. A different tarball fails.
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

@lapismd/jj-conversations 0.1.0 is already on npm. The registry tarball is 8993 bytes, shasum 99b0cd138a8f9fb2a2c4b422237dd2a99dde50d5, SHA-256 84080793c056519a8adbe5fdd4bc3d46b331c419719e269f3456114cc2c94ec8, and integrity sha512-MiuUS4z5+4FMNmE9F0wJFc2MNTbEyhclXBe1XCoquUqyzriqqnTK25m5ARAu8hjgEnMbvAgV5M8mjRtk4vO+Lg==. No provenance attestation. Trusted publisher 6fa3eb4e-4961-4267-825d-060fa2236189.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

npm package tarball, SHA-1 shasum, SHA-256, SHA-512 integrity, and optional provenance attestation.
<!-- /evidence-field -->

<!-- evidence-field:scope markdown -->
## Scope

The conversations npm workflow tests and packs only @lapismd/jj-conversations. It verifies the existing registry tarball by size, SHA-256, and integrity, succeeds when provenance is absent, and fails when the tarball differs. It does not rerun repository-wide heavy qualification or upload 0.1.0 again.
<!-- /evidence-field -->

<!-- evidence-field:next markdown -->
## Next

Keep repository-wide heavy qualification on its own M30 gate.
<!-- /evidence-field -->

<!-- evidence-field:contract_review yaml -->
## Contract review

```yaml
status: changed
reason: JST-ARCH-005 now limits the conversations npm workflow to package checks, exact pack, and idempotent registry verification. Provenance is optional. A different tarball still fails.

```
<!-- /evidence-field -->

<!-- evidence-field:milestone_review yaml -->
## Milestone review

```yaml
status: unchanged
reason: M30 stays In progress. This records the published 0.1.0 package and the narrower conversations workflow. The remaining repository qualification gates are unchanged.

```
<!-- /evidence-field -->

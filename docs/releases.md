# Releases and Changesets

Submit changes through a pull request to `main`. Its `required` GitHub Actions
check must pass against the current base, and review conversations must be
resolved before merging. The protection applies to administrators too. Force
pushes and branch deletion are blocked; published `v*` tags cannot be changed or
deleted. The generated Version Packages PR follows the same merge gate.

Changesets owns versions and notes through the private `@lapismd/jj-store` npm
package. It synchronizes the 19 workspace Cargo packages, separate embedded
RustFS package, four native bundles and Linux multi-architecture image. The npm
package is never published.

## Prepare

1. Run `pnpm changeset` for package, behavior or distribution changes. Select
   the coordination package. Documentation-only changes need no note.
2. The Changesets workflow creates **Version Packages**. `pnpm release:version`
   updates manifests, internal dependency versions, lockfiles and changelog.
   Merge after CI passes.
3. The orchestrator dispatches full qualification for the selected source and
   version. Publication stops on source mismatch, drift or failed qualification.

Initial 0.1.0 publication is the explicit bootstrap exception to pending notes.
Subsequent code changes require a Changesets note.

Generated `changeset-release/main` PRs and Dependabot-authored `dependabot/` PRs
do not need spec, milestone, evidence or additional Changesets edits. The
Changesets exemption follows lapis-notes; jj-store extends it to Dependabot.
Static spec validation, formatting, package checks and all impact-selected test
lanes still run. The release workflow explicitly dispatches all standard checks
for Version Packages; version automation does not generate spec evidence.
Ordinary PRs retain the spec and evidence requirements, and publication always
requires qualification.

## CI and manual qualification

PRs run the standard Rust cohort and affected web checks after formatting,
governance and tooling succeed. Server, CLI and capture package tests (including
unit tests), embedded RustFS, native platforms and browser acceptance run in the
manual qualification workflow. Main pushes always run all standard checks. Cargo
metadata selects changed standard packages plus reverse dependencies, including
development, optional, target-specific and build edges. Manifests, lockfiles, CI
configuration, fixtures and unknown inputs broaden selection. Clippy, doctests
and archives use the same selected packages. The plan summary lists excluded
manual packages and selection reasons.

Archives carry source/package/test inventories and a digest. Partition coverage
must be complete and disjoint; only verified empty partitions may run no tests.
Dependency caches remain separate by platform, toolchain, profile and cohort.
Successful PR builds populate their isolated PR cache; shared-main writes
require both a main workflow context and source verified on canonical main.
Arbitrary qualification refs restore only. Summaries record cache matches and
job/step timings; a restored cache never substitutes for tests.

Once these workflows are available on the default branch, run:

```sh
gh workflow run ci.yml --ref main -f all_checks=true
gh workflow run ci-heavy.yml --ref main -f ref=COMMIT_SHA
```

Omitting `ref` qualifies the workflow's selected revision. Qualification pins
that revision once, runs all standard checks, then every heavy suite.
Independent suites continue after failures, while missing build inputs block
their consumers. No scheduled heavy runs are enabled. The separate
`publish-release.yml` workflow calls this complete qualification at its exact
source SHA before any publication. Changesets release orchestration remains
disabled until initial qualification is ready; running qualification alone does
not publish anything.

Six workflows can also run independently: `ci-heavy-backend.yml`,
`ci-heavy-contracts.yml`, `ci-heavy-storage.yml`, `ci-heavy-ui.yml`,
`ci-heavy-cold.yml` and `ci-heavy-native.yml`. For each, provide the exact
source and the coordinator's producer run/attempt:

```sh
gh workflow run ci-heavy-ui.yml --ref main \
  -f source=COMMIT_SHA -f producer_run=RUN_ID -f producer_attempt=1
```

Storage defaults to both embedded and external acceptance; its internal `lane`
input also lets the coordinator start external checks before runtime
compilation. All standalone suites verify the successful standard gate and
relevant producer jobs. Artifacts use attempt-specific names and checksummed
source receipts. A failed unrelated suite does not prevent reusing successfully
produced inputs. Expired or mismatched artifacts fail; start fresh full
qualification to replace them. Standalone results do not replace the publication
gate.

Compare `qualification-results-*` artifacts and job summaries. Archive consumers
reuse compilation; API contracts, native checks and embedded-storage test
binaries still compile locally, so those reruns are not execution-only
measurements. Reports retain command logs, available JUnit/Playwright results,
source/runner identity and job/step timings. Missing queue/dependency timestamps
remain unknown. Runner minutes count parallel execution separately from elapsed
duration. Download completed-run job timestamps when measuring cache-save steps
that execute after a job's report snapshot.

During the explicitly authorized CI rollout, administrator enforcement may be
disabled only around each verified direct main push. Snapshot protection first,
restore it immediately even after push failure, and verify the restored state.
PR requirements, required checks and tag immutability remain configured.

Run 38700726811 passed standard checks at `199969c8` but failed heavy shard 5.
Run 38716420649 subsequently passed standard checks and built heavy artifacts in
17m15s, then failed `kingfisher_redacts_secrets_and_normalizes_workspace_paths`
on the obsolete `mongodb-kingfisher@2.7.0` identity expectation. Its other
shards were canceled and downstream qualification did not run. These remain
historical failures; restructuring does not qualify those gaps or promote M30 to
Done.

## Initial Cargo bootstrap

Initial crates.io creation requires a short-lived API token. Verify all
artifacts first, then authenticate outside the checkout. Publish the verified
archives in dependency order with `pnpm release:registry -- --publish`.

Configure every crate's trusted publisher for `stevejuma/jj-store`, workflow
`publish-release.yml` and environment `release`, then enforce trusted
publishing. Remove local bootstrap credentials and revoke the token. Subsequent
releases use OIDC temporary credentials; the jj-acp token is not reusable.

## Verify and recover

Crates, tags and release assets are immutable. Existing downloaded package bytes
must match candidate archives. Different contents and yanked versions stop
publication. After a failed or lost acknowledgement, inspect the original source
and registry before retrying; a timeout does not prove rejection.

Keep the original source and artifacts and resume the same version with an
explicit qualified-release workflow run. Publish only missing packages/assets.
Different source needs a new Changesets version. Never move a version tag or
overwrite an asset.

Verify archive checksums before extraction. Verify the image digest and
signature:

```sh
cosign verify ghcr.io/stevejuma/jj-store@sha256:DIGEST \
  --certificate-identity-regexp '^https://github.com/stevejuma/jj-store/.github/workflows/publish-release.yml@refs/heads/main$' \
  --certificate-oidc-issuer https://token.actions.githubusercontent.com
```

GHCR visibility must be public and anonymous pulls must succeed before release
completion. `latest` advances only after publication gates pass. Record
remaining platform, provider and desktop gates in release notes and the
milestone ledger.

## Bounded compilation-cache experiment

The standard workflow exposes an optional branch-only `cache_trial` dispatch. It
runs standard checks first, warms isolated source-specific dependency and
sccache caches, then runs three matched warm builds per backend on Ubuntu 22.04.
Measured caches are restore-only. Every sample runs strict Clippy, doctests and
archive creation; inventories must match and both backend builds pass their
ordinary tests. The result compares complete build-job medians, including setup,
cache and artifact overhead. Adopt sccache only at a 10% or better improvement.
The trial does not schedule heavy tests, publish releases or alter main caches.

```sh
gh workflow run ci.yml --ref codex/ci-portability-and-bootstrap \
  -f all_checks=true -f cache_trial=true
```

The `cache-trial-decision` artifact and job summary retain matched timings and
the decision; per-sample artifacts retain inventory and sccache statistics.
Normal checks continue to use rust-cache alone until that decision is verified.

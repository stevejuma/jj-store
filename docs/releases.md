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
manual qualification workflow. Main pushes always run all standard checks.

Once these workflows are available on the default branch, run:

```sh
gh workflow run ci.yml --ref main -f all_checks=true
gh workflow run ci-heavy.yml --ref main -f ref=COMMIT_SHA
```

Omitting `ref` qualifies the workflow's selected revision. Qualification pins
that revision once, runs all standard checks, then every heavy lane. No subset
switches or scheduled heavy runs are enabled. The separate `publish-release.yml`
workflow calls this complete qualification at its exact source SHA before any
publication. Changesets release orchestration remains disabled until initial
qualification is ready; running qualification alone does not publish anything.

Run 38700726811 passed standard checks at `199969c8` but failed heavy shard 5;
its siblings were canceled and later qualification lanes did not run. This
workflow split does not qualify those gaps or promote M30 to Done.

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

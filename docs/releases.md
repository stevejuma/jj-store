# Releases and Changesets

Changesets owns versions and notes through the private `@lapismd/jj-store` npm
package. It synchronizes the 19 workspace Cargo packages, separate embedded
RustFS package, four native bundles and Linux multi-architecture image. The npm
package is never published.

## Prepare

1. Run `pnpm changeset` for package, behavior or distribution changes. Select
   the coordination package. Documentation-only changes need no note.
2. The Changesets workflow creates **Version Packages**. `pnpm release:version`
   updates manifests, internal dependency versions, lockfiles, changelog and
   evidence together. Merge after CI passes.
3. The orchestrator dispatches full qualification for the selected source and
   version. Publication stops on source mismatch, drift or failed qualification.

Initial 0.1.0 publication is the explicit bootstrap exception to pending notes.
Subsequent code changes require a Changesets note.

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

Verify archive checksums before extraction. Verify the image digest and signature:

```sh
cosign verify ghcr.io/stevejuma/jj-store@sha256:DIGEST \
  --certificate-identity-regexp '^https://github.com/stevejuma/jj-store/.github/workflows/publish-release.yml@refs/heads/main$' \
  --certificate-oidc-issuer https://token.actions.githubusercontent.com
```

GHCR visibility must be public and anonymous pulls must succeed before release
completion. `latest` advances only after publication gates pass. Record remaining
platform, provider and desktop gates in release notes and the milestone ledger.

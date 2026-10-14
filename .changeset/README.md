# Release notes

Run `pnpm changeset` for changes to published Rust packages, server behavior or
distribution. Select `@lapismd/jj-store` and the appropriate semantic version
bump. This private npm package coordinates every Cargo package, native bundle
and container; it is never published to npm.

The Version Packages pull request runs `pnpm release:version`, applies the
Changesets changelog and synchronizes Cargo versions. Merge it after CI passes.
The release orchestrator selects the immutable main commit and dispatches full
qualification before publishing packages, assets and GHCR images.

Documentation-only changes need no release note. The initial 0.1.0 source
publication is the explicit bootstrap exception. Do not add credential values
to release notes or repository files.

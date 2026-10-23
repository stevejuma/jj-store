# Validation runners

Linux-compatible test and check commands run through Docker by default. The
ignored `.local/validation/config.json` selects a Docker context and optional
resource limits. For this workspace use `{"context":"omarchy"}`. No endpoint or
credentials belong in tracked configuration. An unreachable daemon fails without
falling back to the host.

```sh
pnpm validate:doctor
pnpm validate:images
pnpm test:storybook -- catalog/components/store-application/RepositoryApplication.stories.svelte
pnpm validate:docker -- test:spec
JJ_STORE_VALIDATION_RUNTIME=native pnpm web:check
pnpm validate:collect -- jj-store-validation-RUN_ID
pnpm validate:stop -- jj-store-validation-RUN_ID
pnpm validate:clean -- --dry-run
pnpm validate:clean -- --apply
```

An optional `"transport":"ssh-cli"` uses the SSH endpoint recorded in the Docker
context when the Docker client SSH bridge is unavailable. It streams
build/source archives and reports through SSH without remote path assumptions.

Images are built on the selected daemon when their fingerprint is missing.
`validate:images -- --rebuild` explicitly rebuilds both toolchains. Defaults are
8 CPUs, 24 GiB memory and 2 GiB shared memory. `cpus`, `memoryGiB`, and `shmGiB`
can be set in local configuration. Development services and native desktop
preparation remain local; hosted CI selects native execution explicitly.

Each run transfers an exact source snapshot with its original revision/parents.
Full-toolchain runs also transfer the exact tracked sibling pins from local Git
objects, with package identity and checksum verification; sibling working-copy
edits are excluded. No remote source credentials are needed. Host bind mounts,
credentials, development data and host node_modules are not used. Reports under
`.local/validation/runs` identify the source, image and package archive inputs.
Failure and cancellation never establish acceptance. The unfiltered Storybook
suite runs four sequential shards with fresh browser processes and separate
JUnit reports. Explicit story targets and caller-selected shards remain exact.
Every shard runs even when an earlier one fails. Collect a retained failed run
before cleanup. `--caches` additionally removes unused runner-owned cache
volumes; Docker refuses volumes held by containers.

Registry dependencies use the frozen lockfile. Temporary upstream validation can
opt into `archives` in local configuration, pointing to a JSON array with
`name`, `version`, `path` and `sha256` for each package archive. Paths are
relative to that manifest. Only declared, checksum- and identity-verified
archives are staged; the isolated install projection restores source manifests
before tests. Archive validation remains distinct from registry release
qualification.

Linux visual runs use the `container-linux` profile and its own PNG namespace.
Mac capture remains available with explicit native execution and the existing
native profile. Baseline updates require separate approval. Linux tests cannot
qualify native macOS desktop, live provider or hardware acceptance.

Production app and Help browser lanes build their preview server through managed
Cargo before Playwright starts its readiness deadline. They use the executable
from that registered target, including an explicit native target selection.
Build failure prevents browser startup; the server readiness deadline is
unchanged.

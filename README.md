# jj-store

Native Jujutsu repository storage, collaboration, workflows and a browser UI,
backed by RustFS. Immutable native objects and manifest-selected publication
remain authoritative; indexes and adapters are rebuildable.

The initial **0.1.0** distribution is experimental. The
[capability guide](https://stevejuma.github.io/jj-store/manual/reference/capabilities/)
and [milestone ledger](spec/src/current/progress/index.md) record qualification
and remaining platform, replication, provider and desktop gates.

- [Specification](https://stevejuma.github.io/jj-store/)
- [Application manual](https://stevejuma.github.io/jj-store/manual/)
- [Native downloads](https://github.com/stevejuma/jj-store/releases)
- [Server image](https://github.com/stevejuma/jj-store/pkgs/container/jj-store)
- [Changesets and release recovery](docs/releases.md)
- [Security reporting](SECURITY.md)

## Install

Download the GNU/Linux or macOS archive for ARM64 or x86-64, verify
`SHA256SUMS`, and extract it outside a checkout. Retain the bundle layout:
`bin/jj-store`, `bin/jj-store-jj`, `bin/jj-store-rustfs`, and `web/` with
matching Help assets. Add its `bin` directory to PATH or use explicit executable
paths. The existing upstream `jj` command stays available. macOS command-line
bundles are unsigned; desktop/DMG distribution has separate qualification.

Rust **1.99** or newer supports Cargo installation:

```sh
cargo install jj-store --locked
cargo install jj-store-embedded-rustfs --locked
jj-store --version
jj-store-jj store --help
```

Cargo installs executable tools. Complete downloads also supply the browser and
Help. The storage helper is separately compiled to preserve native allocator
boundaries. Kingfisher's native dependency uses checksum-verified portable
archives; consult that dependency for offline/source-build prerequisites.

## Local managed storage

Choose a private state directory outside Git/JJ checkouts:

```sh
jj-store init --local --state-dir /absolute/private/jj-store
jj-store config check --config /absolute/private/jj-store/server.toml
jj-store run --config /absolute/private/jj-store/server.toml
```

Set `web_root` to the matching bundle's `web/` directory. LocalTrusted uses
literal-loopback HTTP and an owner-only Unix socket. Follow the
[native tutorial](docs/site/getting-started/native.md) and
[project/task tutorial](docs/site/getting-started/work.md). Existing Git-backed
JJ folders use the separately governed companion profile; conversion support has
its own documented limits.

## Authenticated Docker server

`ghcr.io/stevejuma/jj-store` supports Linux AMD64 and ARM64. Prefer a versioned
digest. It contains the server, UI and Help and connects to external RustFS.

Prepare an authenticated format-2 configuration, private installation/catalog,
operator policy and TLS certificate/key. The installation directory must be
owned by UID/GID **65532**, with private directory/file permissions. Set
`web_root` to `/usr/share/jj-store/web`, use private TLS paths, and inject the
environment references named by your storage configuration.

```sh
docker run --detach --name jj-store \
  --read-only --cap-drop ALL --security-opt no-new-privileges \
  --tmpfs /tmp:rw,noexec,nosuid,size=64m \
  --mount type=bind,src=/absolute/private/jj-store,dst=/var/lib/jj-store \
  --publish 7443:7443 \
  --env JJ_STORE_S3_ACCESS_KEY --env JJ_STORE_S3_SECRET_KEY \
  ghcr.io/stevejuma/jj-store:v0.1.0
```

The image runs without root and requires explicit configuration. Shared
listeners require authenticated TLS. LocalTrusted cannot be exposed through a
shared listener. See [configuration](docs/site/reference/configuration.md) and
[access](docs/site/reference/security.md) for initialization, grants and
revocation. `docker stop` requests graceful shutdown. Preserve installation
state and authoritative RustFS data across upgrades; credentials stay outside
source checkouts.

## Optional agent execution

Install a matching
[jj-acp release](https://github.com/stevejuma/jj-acp/releases) and explicitly
configure paired executables and admitted agents. Provider tools and credentials
belong to the execution host. The default image includes no agent runner or
workflow executor. Live-provider acceptance remains distinct from deterministic
fixtures.

## Develop and verify

Use Rust 1.99.0, Node 22+, pnpm 10.32.1, Python 3.11+, uv, mdBook 0.5.4 and
Playwright Chromium:

```sh
pnpm install --frozen-lockfile --ignore-pnpmfile
uv sync --locked
pnpm exec playwright install chromium
pnpm dev:cargo -- build -p jj-store --bins --locked
pnpm build:embedded-rustfs
pnpm web:build
```

Follow [AGENTS.md](AGENTS.md) for focused verification. CI separates ordinary
and eight independent heavy shards, real-storage, browser and native-platform
lanes. Relevant PRs select affected heavy tests; nightly and release runs select
all qualification lanes. Ignored subprocess helpers run through their acceptance
parents.

Initialize the private development fixture once with `pnpm dev:restart`, then
use `pnpm dev:ui` for Vite HMR at `http://127.0.0.1:7460/`. After Rust edits run
`pnpm dev:backend:build` and `pnpm dev:backend:restart`. `pnpm dev:stop`
preserves state; `pnpm dev:reset` explicitly replaces the fixture.

Inspect build storage with `pnpm dev:disk` and `pnpm dev:clean -- --dry-run`.
Use managed Cargo commands; live targets, fixtures and recovery archives are
protected. The [documentation guide](docs/site/contributing/documentation.md)
explains generated references and source coverage.

## Contribute and release

Commit verified slices with Jujutsu and update owning requirements and evidence.
Run `pnpm changeset` for publishable changes, selecting `@lapismd/jj-store`.
Changesets creates a Version Packages PR and synchronizes Cargo, native-bundle
and container versions. The npm coordination package remains private.

Full qualification precedes immutable publication. Releases include checksums,
container SBOM/provenance and keyless image signatures. Read the
[release guide](docs/releases.md) before retrying a partial publication.

[Apache-2.0](LICENSE.md), with [third-party notices](docs/licenses/NOTICE.md).

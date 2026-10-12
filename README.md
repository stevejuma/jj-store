# jj-store

Native, centrally authoritative Jujutsu repository storage backed by RustFS. The
source repository is Git-colocated; served repositories have no Git, SQL, or
relay dependency. The first release remains under implementation; the progress
ledger records qualified native CLI/server capabilities and remaining gates. No
existing repository migration is provided.

- [Implementation plan](docs/implementation-plan.md)
- [Progress and next action](spec/src/milestones.md)
- [Canonical specification](spec/src/index.md)
- [Upstream pins](docs/upstream-lock.md)
- [Original reference](docs/reference/README.md)
- [Agent guide](AGENTS.md)
- [Repository browser](docs/browser.md)
- [Local Codex capture preview](docs/capture-preview.md)

## Development

Use Rust 1.98.1, Node 22 or newer, pnpm 10.32.1, and mdBook. Install tooling
with `pnpm install --frozen-lockfile`. Use the focused package and test lanes in
the [agent guide](AGENTS.md). The root `pnpm spec:check`, `pnpm check`,
`pnpm test`, and `pnpm build` commands are wider qualification lanes. The pinned
Rust toolchain includes rustfmt. `pnpm format` formats Rust, authored Markdown
and tooling; `pnpm format:check` checks them without making changes. Use
`pnpm format:rust` or `pnpm format:rust:check` for Rust only. Rust formatting
also runs in both `pnpm check` and `pnpm spec:check`. Docker is required for
isolated real RustFS conformance, not unit tests.

`pnpm contracts:generate` regenerates OpenAPI and browser types from the Rust
application contracts. `pnpm contracts:check` rejects drift and verifies actual
HTTP fixture responses; it runs in both normal and specification checks. See the
[contract guide](schemas/README.md).

The workspace builds `target/debug/jj` and `target/debug/jj-store`. Start a
local installation with `target/debug/jj-store init --local --state-dir <path>`
and the [server guide](docs/cli/server.md). Use
[jj store init](docs/cli/init.md) for a new native repository and
[attach](docs/cli/attach.md) for another physical workspace. Use explicit build
paths; development never replaces the installed `jj`.

For the browser UI, `pnpm dev:restart` rebuilds the UI, embedded RustFS helper
and development binaries, then starts the private LocalTrusted installation in
`.local/dev-demo`. It initializes and seeds the native demo repository only when
that ignored directory is empty. `pnpm dev:stop` stops the owned processes while
preserving the repository, installation identity and embedded RustFS volume.

For daily UI development, initialize once with `pnpm dev:restart`, then run
`pnpm dev:ui`. Vite serves the current workspace at `http://127.0.0.1:7460/`
with HMR and proxies `/v1` to the compiled backend on port 7462. Svelte,
TypeScript and CSS edits need no build or backend restart. The UI command
requires existing binaries and a complete fixture; it never builds or seeds.
After changing Rust, use `pnpm dev:backend:build` followed by
`pnpm dev:backend:restart`, which keeps Vite running. `pnpm dev:restart` still
rebuilds/restarts the whole stack and returns to built UI serving;
`pnpm dev:stop` stops either mode without deleting data. To reuse a fixture from
another workspace, set `JJ_STORE_DEV_ROOT` to its absolute `.local/dev-demo`
path for every command. `JJ_STORE_DEV_SERVER` selects an existing server binary;
the UI and backend ports can be selected with `JJ_STORE_DEV_PORT` and
`JJ_STORE_DEV_BACKEND_PORT`. Production CSP verification uses the built UI; Vite
has a separate loopback-only HMR document policy.

Build caches are maintained by repository commands. `pnpm dev:disk` reports
Cargo caches across registered JJ workspaces and shows protected state
separately. `pnpm dev:clean -- --dry-run` previews cleanup; `--apply` removes
eligible idle artifacts. Use `pnpm dev:cargo -- test -p <package> --locked` for
focused Rust verification. The defaults are 40 GiB per target, 80 GiB across
managed targets, and 30 GiB minimum free space. Positive GiB overrides are
`JJ_STORE_BUILD_TARGET_GIB`, `JJ_STORE_BUILD_TOTAL_GIB` and
`JJ_STORE_BUILD_FREE_GIB`. A compiler can exceed a threshold while running;
unresolved pressure stops subsequent managed work with cleanup guidance.

Temporary qualification targets use `.local/build-targets/NAME`. Register them
with `pnpm dev:target:register -- --target PATH --reason TEXT`, then select that
path with `CARGO_TARGET_DIR` or Cargo's `--target-dir`. Successful qualification
runs `pnpm dev:target:retire -- --target PATH --apply`; intentionally retained
outputs remain protected until `dev:target:complete`. Clean idle workspace build
roots before retiring the workspace. Fixtures, recovery archives, staged runtime
resources and source repositories are preserved. Direct Cargo bypasses the
budget preflight; use managed commands and avoid concurrent manual cleanup.
Python 3, POSIX locks and `lsof` are required for guarded cleanup.

`pnpm dev:reset` replaces the demo state with the deterministic sample fixture.
The reset publishes representative Codex, Cursor and OpenCode sessions and
creates open and resolved reviews across all seven repository, operation,
revision, code and agent anchor types. No repository token or Docker service is
required. `pnpm web:test:real` uses its own OS-temporary fixture, publishes a
change through a separate native CLI process, verifies SSE notification and
explicit browser refresh through the built UI, then removes that isolated
fixture. The local demo configures its search worker and reports ready only
after the complete seeded repository position is indexed.

On macOS with Xcode license prompts, prefix checks with
`DEVELOPER_DIR=/Library/Developer/CommandLineTools` to use the installed Command
Line Tools SDK; this does not change the system developer selection.

Use `pnpm cli:install` to build release binaries and install `jj-store` (server
administration) and `jj-store-jj` (custom JJ and project commands) in
`~/.local/bin`. Add that directory to `PATH`. `--bin-dir <path>` selects a
different destination; `--debug --no-build` installs existing debug artifacts.
The installer never replaces the installed `jj` or starts a service.

The demo now includes the existing native local checkout, a server-only native
repository and an ordinary Git-backed JJ folder at
`.local/dev-demo/git-repository`. The three sample projects retain their native
metadata home and link one repository each. The Git-backed folder uses real Git
storage and companion metadata; it is available through the local CLI, while
browser serving remains pending. Restart adds these variants to an existing
fixture and changes only untouched sample links and task targets. Edited records
and retired links remain intact. Projects and Repositories sidebar disclosure
choices persist in browser local storage.

## Application documentation

The [application manual](docs/site/index.md) explains repository contexts,
workflows, configuration, app-specific CLI/API reference and architecture. It is
separate from the canonical [specification](spec/src/SUMMARY.md). Run
`pnpm docs:serve` for a local Material for MkDocs preview at
`http://127.0.0.1:7480/`. `pnpm web:build` bundles that same self-contained
site; **Help** opens it in a full-screen overlay in the browser and desktop app.

`pnpm docs:check` validates generated references, acknowledged source coverage,
Mermaid, navigation and links. Read
[documentation maintenance](docs/site/contributing/documentation.md) for the
locked build dependencies, review acknowledgements and focused tests.

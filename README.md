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

## Development

Use Rust 1.98.1, Node 22 or newer, pnpm 10.32.1, and mdBook. Install tooling
with `pnpm install --frozen-lockfile`. Run `pnpm spec:check`, `pnpm check`,
`pnpm test`, and `pnpm build`. The pinned Rust toolchain includes rustfmt.
`pnpm format` formats Rust, authored Markdown and tooling; `pnpm format:check`
checks them without making changes. Use `pnpm format:rust` or
`pnpm format:rust:check` for Rust only. Rust formatting also runs in both
`pnpm check` and `pnpm spec:check`. Docker is required for isolated real RustFS
conformance, not unit tests.

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
preserving the repository, installation identity and embedded RustFS volume;
`pnpm dev:reset` replaces them with the deterministic sample fixture. No
repository token or Docker service is required. `pnpm web:test:real` uses its
own OS-temporary fixture, publishes a change through a separate native CLI
process, verifies SSE notification and explicit browser refresh through the
built UI, then removes that isolated fixture. The local demo configures its
search worker and reports ready only after the seeded repository is indexed.

On macOS with Xcode license prompts, prefix checks with
`DEVELOPER_DIR=/Library/Developer/CommandLineTools` to use the installed Command
Line Tools SDK; this does not change the system developer selection.

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

Use Rust 1.97.1, Node 22 or newer, pnpm 10.32.1, and mdBook. Install tooling
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

The workspace builds `target/debug/jj` and `target/debug/jj-store`. Start with
`target/debug/jj-store config example` and the
[server guide](docs/cli/server.md). Use [jj store init](docs/cli/init.md) for a
new native repository and [attach](docs/cli/attach.md) for another physical
workspace. Use explicit build paths; development never replaces the installed
`jj`.

On macOS with Xcode license prompts, prefix checks with
`DEVELOPER_DIR=/Library/Developer/CommandLineTools` to use the installed Command
Line Tools SDK; this does not change the system developer selection.

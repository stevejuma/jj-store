# jj-store-desktop

Native desktop host for the jj-store browser application. It manages a private
LocalTrusted installation, paired server/storage processes, native window
integration and repository-bound terminal sessions.

From the repository root, stage development resources with
`pnpm desktop:prepare` and launch with `pnpm desktop:run`. The independent
desktop Cargo workspace isolates Tauri and terminal dependencies from upstream
JJ and RustFS. Initial qualification targets macOS ARM64; signed distribution
and other platform gates remain separately tracked.

See [desktop development](../docs/desktop-development.md) for staging, tests,
lifecycle and terminal details, and
[process lifecycle](../docs/site/technical/lifecycle.md) for the application
host model.

The desktop stages a verified Lapis AI helper bundle built from the sibling
checkout. Run `pnpm ai:prepare` or let `pnpm desktop:prepare` build it. Native
resource verification uses the locally linked shared Rust crate. Agent and
compute requests continue through the owned server API. See
[local compute](../docs/local-compute.md) for bundle selection and lifecycle.

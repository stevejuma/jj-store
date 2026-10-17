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

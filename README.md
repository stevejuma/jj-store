# jj-store

`jj-store` is a native object store and collaboration service for
[Jujutsu](https://github.com/jj-vcs/jj). It keeps immutable JJ objects in
RustFS, publishes repository state through manifest compare-and-swap, and adds a
custom CLI, structured APIs, search, projects, reviews, agent history,
workflows, and a browser application.

The project is experimental. Native storage and the LocalTrusted application are
implemented, but some replication, provider, desktop, platform, visual, and
public-release gates are still open. The
[capability guide](https://stevejuma.github.io/jj-store/manual/reference/capabilities/)
and [milestone ledger](spec/src/current/progress/index.md) are the source of
truth for what is qualified.

## Explore

- [Application manual](https://stevejuma.github.io/jj-store/manual/) — install,
  operate, and use jj-store
- [Storybook](https://stevejuma.github.io/jj-store/storybook/) — maintained UI
  components and interaction states
- [Specification](https://stevejuma.github.io/jj-store/) — contracts,
  requirements, milestones, and evidence
- [GitHub releases](https://github.com/stevejuma/jj-store/releases) — native
  bundles, standalone server executables, and checksums
- [Container image](https://github.com/stevejuma/jj-store/pkgs/container/jj-store)

## What it provides

- Native JJ object, operation, view, manifest, WAL, and receipt storage
- LocalTrusted and authenticated server profiles
- A collision-safe `jj-store-jj` client alongside the upstream `jj` command
- Browser views for code, search, projects, tasks, reviews, agents, and
  workflows
- Repository-native WorkStore, AgentStore, ReviewStore, and WorkflowStore data
- Rebuildable search and adapter projections that never become repository
  authority

The manifest is the only publication authority. Uploaded objects and WAL
candidates remain invisible until selected by committed references, and a
timeout is treated as an unknown outcome rather than a proven conflict.

## Install a native bundle

Releases target GNU/Linux and macOS on ARM64 and x86-64. Download the archive
for your platform, verify it against `SHA256SUMS`, and extract it outside a
repository checkout. Keep the bundle together:

```text
bin/jj-store
bin/jj-store-jj
bin/jj-store-rustfs
web/
```

The release also exposes `jj-store-<version>-<target>` as a standalone server
download. After checksum verification, make it executable with `chmod +x`. The
full archive is recommended because it includes the matching browser, Help,
custom client, and embedded-storage helper.

macOS command-line bundles are currently unsigned. Windows and desktop/DMG
distribution have separate qualification gates.

## Run locally

Choose a private state directory outside Git and JJ checkouts, then initialize
the LocalTrusted profile:

```sh
jj-store init --local --state-dir /absolute/private/jj-store
jj-store config check --config /absolute/private/jj-store/server.toml
jj-store run --config /absolute/private/jj-store/server.toml
```

Set `web_root` to the extracted bundle's `web/` directory. LocalTrusted listens
only on literal loopback and uses an owner-only Unix socket. Start with the
[native tutorial](https://stevejuma.github.io/jj-store/manual/getting-started/native/)
and the
[projects tutorial](https://stevejuma.github.io/jj-store/manual/getting-started/work/).

For a shared deployment, use the authenticated TLS profile with external RustFS.
The container runs without root and contains the server, browser, and Help
assets. Successful canonical `main` builds publish immutable `sha-<12-hex>`
images plus the rolling `main` tag; qualified releases publish the matching
`v<version>` tag and advance `latest`. Prefer immutable tags or digests outside
development. Configuration, installation policy, credentials, and TLS material
must live outside the checkout; see the
[configuration](https://stevejuma.github.io/jj-store/manual/reference/configuration/)
and [security](https://stevejuma.github.io/jj-store/manual/reference/security/)
guides.

## Develop

Read [AGENTS.md](AGENTS.md) before changing implementation or specifications.
The repository uses Jujutsu for version control and requires focused validation
plus same-change Markdown evidence for protected behavior.

The primary toolchain is Rust 1.99.0, Node 22+, pnpm 10.32.1, Python 3.11+, uv,
mdBook 0.5.4, and Playwright Chromium. Install locked dependencies, then
initialize the private development fixture once:

```sh
npx --yes pnpm@10.32.1 install --frozen-lockfile --ignore-pnpmfile
uv sync --locked
npx --yes pnpm@10.32.1 exec playwright install chromium
npx --yes pnpm@10.32.1 dev:restart
```

Use `pnpm dev:ui` for UI work with Vite HMR. After Rust changes, use
`pnpm dev:backend:build` and `pnpm dev:backend:restart`; `pnpm dev:reset` is the
explicit destructive fixture reset. Managed Cargo entry points enforce disk and
target ownership rules.

Pull requests run impact-selected standard checks. Daily and manual complete
qualification additionally covers the heavy Rust cohort, native platforms, real
storage, and cold application suites at one immutable source revision. Focused
success is not release qualification. The [release guide](docs/releases.md)
explains Changesets, qualification, publication, and recovery.

## Contributing

Land narrow, verified Jujutsu changes. Update the owning requirement before or
with protected behavior, record actual validation in `spec/src/history`, and add
a Changeset for publishable changes. Do not put credentials or server state
inside a repository checkout.

Security issues should follow [SECURITY.md](SECURITY.md). The project is
licensed under [Apache-2.0](LICENSE.md); third-party notices are in
[docs/licenses/NOTICE.md](docs/licenses/NOTICE.md).

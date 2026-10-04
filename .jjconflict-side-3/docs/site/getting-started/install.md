# Installation and setup

## Components

A development installation has paired `jj` and `jj-store` executables, a built
browser bundle, and an embedded RustFS helper for native storage. The custom
`jj` retains upstream command handling and adds jj-store's command families. The
standalone `jj-store` process owns server startup and shutdown. The optional
macOS desktop host supervises those same services and loads the browser UI.

The qualified native Git profile uses JJ 0.45.1 and Git 2.55.0 on macOS ARM64.
Desktop packaging has additional qualification limits; see
[capabilities](../reference/capabilities.md). Do not assume a development app
bundle is a qualified distributable release for every platform.

## Build explicit development executables

From the source checkout, install the pinned Rust toolchain and pnpm version
named by the repository. The documentation toolchain also needs `uv`, Python
3.11 or later, and Playwright Chromium for build-time diagram rendering.

```sh
pnpm install --ignore-pnpmfile --frozen-lockfile
uv sync --locked
pnpm exec playwright install chromium
cargo build --locked -p jj-store-cli --bins
pnpm build:embedded-rustfs
pnpm web:build
```

`--ignore-pnpmfile` uses the committed registry dependencies instead of the
workspace's optional local Design Core substitution. Registry-only acceptance
has its own current limitations; inspect the command results rather than
assuming that installing dependencies proves the UI is qualified.

Use absolute paths to `target/debug/jj`, `target/debug/jj-store`, and
`runtime/embedded-rustfs/target/debug/jj-store-rustfs`. These commands do not
replace an installed `jj`. Keep the paired CLI and server from the same build.
The browser build includes the matching manual under its `help` directory.

## Choose where state lives

Put normal installation state outside Git and JJ checkouts, for example in a
private application-data directory. The server rejects unsafe control paths. The
checkout's ignored `.local` development fixtures are an explicit harness
exception, not a production configuration template.

Use [native setup](native.md) to create managed storage or
[folder setup](local.md) for an existing Git-backed JJ repository. Neither
choice requires an upstream server. A project metadata home can later link
several separately authorized code repositories.

## Development fixture and desktop

`pnpm dev:restart` builds and initializes the complete development fixture.
After that, `pnpm dev:ui` runs the current UI with Vite against the existing
backend. Ordinary UI edits need no RustFS rebuild or data reset. See
[development](../contributing/development.md) for the exact lifecycle.

`pnpm desktop:prepare` stages paired development resources. Desktop setup and
packaging remain distinct from running the browser; preserve the reported
platform and release limits when distributing an app.

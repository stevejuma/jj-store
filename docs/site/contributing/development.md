# Development

Read the repository's `AGENTS.md` and owning canonical specification before
changing behavior. The specification owns requirements and acceptance evidence;
this manual explains the supported behavior to users.

## Dependencies and local execution

The repository uses the pinned Cargo lockfile, Node 22 or later and pnpm. Python
3.11 or later and `uv` build documentation; `uv sync --locked` installs its
pinned dependencies. Playwright Chromium renders Mermaid diagrams at build time.
Install it with `pnpm exec playwright install chromium` when absent.

For an existing complete fixture, `pnpm dev:ui` serves Vite at the configured
loopback origin and reuses the existing native executable. Run
`pnpm dev:restart` once on a fresh workspace to build and initialize the
disposable fixture. `pnpm dev:stop` preserves its state. `pnpm dev:reset` is an
explicit destructive reset, not an ordinary iteration command.

After backend edits, `pnpm dev:backend:build` and `pnpm dev:backend:restart`
replace the backend while Vite remains running. Explicit development paths avoid
replacing the installed `jj` or restarting unrelated services. Never put real
credentials in fixtures.

## Verification by affected boundary

| Change                             | Focused starting lane                                            |
| ---------------------------------- | ---------------------------------------------------------------- |
| Manual or references               | `pnpm docs:check`, `pnpm docs:test`                              |
| Browser components                 | `pnpm web:check`, exact component interactions and accessibility |
| Documentation delivery             | `pnpm docs:test`, HTTP asset tests, `pnpm docs:test:browser`     |
| Rust provider/service              | Owning package check and exact regression test                   |
| Canonical requirements or evidence | `pnpm spec:validate`, `pnpm spec:first`, `pnpm check:progress`   |

Generated-asset and lifecycle changes require a cold run. Source dependency
validation is distinct from registry-resolved release acceptance. Visual
baseline updates require their own review; they are not substitutes for keyboard
or functional checks.

Production browser components live under `web/src/lib`. Catalog fixtures may
import production components, while production never imports fixture state.
Community is a source/design reference; Design Core and Mira remain foundational
package dependencies. See the repository guide for ownership and attribution.

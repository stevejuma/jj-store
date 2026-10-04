# Local Codex capture preview

Run these commands from the preview workspace. The preview owns its isolated
`.local/codex-preview` fixture and loopback ports 7470/7472. It preserves
captured history across stop and restart.

```sh
pnpm dev:capture:setup
```

Setup checks for Codex CLI **0.156.1**, builds the development binaries and
seeds a new native repository using the existing development workflow. If an
embedded RustFS executable is already built, set `JJ_STORE_DEV_RUSTFS_RUNTIME`
to its absolute path. Otherwise build it with `pnpm build:embedded-rustfs`
first.

Open <http://127.0.0.1:7470/>. Start Codex in `.local/codex-preview/repository`:

```sh
codex -C "$PWD/.local/codex-preview/repository"
```

Open Settings → Agents → Capture to see the configured profile and actual root
directories selected at startup. The CLI default resolves to
`CODEX_HOME/sessions` or `HOME/.codex/sessions`; explicit roots override that
selection. Sources shows observed sessions; Imports shows history requests;
Diagnostics contains worker and queue health. Let discovery observe the session
before submitting a prompt. Capture requires a fresh supported task start after
monitoring has armed; a start missed during discovery waits for the next turn or
an explicit **Import history**. Source growth alone does not authorize importing
old history. Import controls show body-free source ordinals; a source with no
recorded repository binding may be unlinked. An import that cannot establish
current directory admission remains withheld and may return an unknown
acknowledgement. Preserve its original request ID; a missing receipt does not
prove rejection. Clearer source selection and an explicit pre-acceptance refusal
are deferred usability work.

The debug build can spend more than two minutes preparing the native sanitizer
on a cold capture/import. During this work, Capture inspection may temporarily
report busy; the original import continues and its receipt can be inspected.
Read-only inspection joins the management queue for at most one second rather
than losing every race to continuing import work. The source table retains
accepted rows and shows a footer spinner while refreshing. The source endpoint
reads the worker's current cached catalog, without scanning transcript folders;
source identities and cursors are durable private state, while runtime coverage
is reconstructed after restart. Cold sanitizer optimization remains deferred.

Captured sessions and transcript updates appear automatically in Agents. The
sample fixture also contains seeded Codex, Cursor and OpenCode sessions; only
Codex CLI automatic capture is enabled.

Try a distinctive harmless prompt, add another turn, pause/resume in Capture,
then restart the backend and inspect the same session:

```sh
pnpm dev:capture:restart
```

After editing Rust, run `pnpm dev:backend:build` before restarting. Vite stays
running. To stop or start the existing preview:

```sh
pnpm dev:capture:stop
pnpm dev:capture
```

Configuration lives in `.local/codex-preview/local/server.toml`. Setup enables
this exact CLI profile; normal `CODEX_HOME/sessions` or `~/.codex/sessions`
locations are used:

```toml
[agent_capture]
enabled = true
max_readers = 4
rescan_seconds = 5
max_sources = 1024
max_pending_bytes = 67108864

[[agent_capture.harnesses]]
profile = "codex-cli-jsonl-v1"
product_version = "0.156.1"
```

The server verifies the native repository and its exact server connection before
ingestion. Another physical workspace of the same registered repository can
qualify; unrelated folders remain unavailable. Explicit transcript roots can be
configured instead of the normal locations. Configuration changes require a
backend restart; runtime Resume cannot override disabled configuration.

Optional environment overrides are `JJ_STORE_CAPTURE_PREVIEW_ROOT`,
`JJ_STORE_CAPTURE_PREVIEW_PORT`, `JJ_STORE_CAPTURE_PREVIEW_BACKEND_PORT` and
`JJ_STORE_CAPTURE_CODEX`. Use the same overrides for every preview command.

Codex Desktop needs independent actual-version qualification. OpenCode, Cursor,
advanced conversation/review features and broader release qualification remain
deferred in the [implementation plan](implementation-plan.md). This preview
supports current AgentStore only. Visual baselines remain pending. The inherited
registry Design Core package lacks APIs used by current main; local validation
uses main's documented dependency setup with tracked registry declarations
preserved. Registry-only UI release qualification remains open.

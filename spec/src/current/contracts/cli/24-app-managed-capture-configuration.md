# App-managed capture configuration

The local desktop and browser configuration editor saves only typed
`agent_capture` settings in the selected external server configuration. It uses
the existing bounds and qualified Codex versions, shows resolved defaults and
requires the next backend startup to activate changes. Restart opens retained
capture state without resetting policy, source generations or import receipts.
The original preview's manual TOML setup is now optional for these settings.

Matching HMR backend restarts reuse the exact fixture's saved `ui-server.toml`
configuration, retaining app-edited capture and search settings. Only explicit
loopback port, UI mode and development binary overrides are reapplied. A
missing, linked or oversized retained configuration fails before stopping the
backend; startup MUST NOT silently restore defaults. Switching to another
configuration or fixture remains an explicit development mode selection.

UI development validates the built manual before stopping services. Run
`pnpm docs:build` when the bundle is missing or invalid. A successful subsequent
documentation build publishes its manifest and updates Vite's Help assets
without restarting the backend or reloading the application.

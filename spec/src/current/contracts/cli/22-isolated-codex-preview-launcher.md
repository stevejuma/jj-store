# Isolated Codex preview launcher

`pnpm dev:capture:setup` builds an isolated native development fixture, links
its physical repository and enables only qualified Codex CLI capture at the
normal known locations. It uses the existing dev UI/backend ownership checks,
ports 7470/7472 and ignored `.local/codex-preview` state by default.
`dev:capture` reuses it, `dev:capture:restart` replaces only the preview
backend, and `dev:capture:stop` stops the owned preview processes while
preserving history. The main fixture, installed binaries and unrelated services
are untouched. CLI version is checked before startup; Desktop is a separate
qualification gate.

The local preview exposes capture management through Settings → Agents, with the
shared divided Settings rows and header refresh. Local service retains service
and device-folder management.

The capture runtime retains the same resolved directory roots given to discovery
at startup. LocalTrusted Admin Settings → Agents displays those paths, including
Codex home defaults and explicit configured overrides; changing configuration
requires backend restart. Count-only coverage is insufficient for directory
configuration inspection.

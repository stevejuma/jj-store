---
schema: 1
id: E-M17-011
---

# E-M17-011 details 3

<!-- evidence-field:runs runs -->
## Validation

<!-- run -->
### Run 1

```yaml
result: pass
failures: 0
hasNote: true
noteEncoding: markdown

```

```shell
pnpm exec prettier --check spec/src/api.md spec/src/cli.md spec/src/collaboration.md spec/src/compatibility.md spec/src/milestones.md
```

**Notes**

All changed canonical documents pass formatting.
<!-- /run -->

<!-- run -->
### Run 2

```yaml
result: pass
failures: 0
hasNote: true
noteEncoding: markdown

```

```shell
pnpm check:progress
```

**Notes**

129 tasks validate with 266 evidence entries and unchanged parent evidence; M17.2 and aggregate M17 remain In progress.
<!-- /run -->
<!-- /evidence-field -->

<!-- evidence-field:result markdown -->
## Result

pass
<!-- /evidence-field -->

<!-- evidence-field:remaining list -->
## Remaining

[]
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

Rust 1.98.1; JJ 0.45.1; Codex CLI 0.156.1 and Desktop 0.159.2 location profiles. Disposable filesystem and synthetic metadata fixtures on macOS ARM64; no real transcript body read or harness edit qualification.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

No persistence, configuration-schema, prepared request, receipt, object or wire changes. Explicit roots precede CODEX_HOME/sessions and server-user ~/.codex/sessions. Device-only approved roots and candidates retain canonical physical identity without Serialize/Debug. Metadata scans retain at most nine directory handles, examine at most 4096 entries and return at most 256 candidates per step; depth limit is eight. Profile/header, repository and activity admission remain independent.
<!-- /evidence-field -->

<!-- evidence-field:revision markdown -->
## Revision

enclosing Jujutsu change mmsuynku
<!-- /evidence-field -->

<!-- evidence-field:scope markdown -->
## Scope

New read-only capture-runtime capability plus pure server harness root resolution require the complete direct discovery and configuration binaries, affected package all-target checks and strict lint. Default roots do not create paths; the incremental walk reads no bodies, withholds nested symlinks, special files, changed candidates and rebound roots, and exposes incomplete/depth/unavailable counts. It grants no source lease, model activity, repository binding, sanitization, checkpoint or publication authority. Strict server-consumer lint additionally required a mechanical repair of three inherited needless borrows in local_browser; no browser behavior/lifecycle, handler signature, schema or route changed, so contracts/cold UI/product browser lanes are out of scope. Spool, projection, RustFS, real harness edits, installation, daemon restart and root aggregates are out of scope. Remaining M17.2 gates include common native aliases, per-record source contexts, bound-reader preparation and source leases, active admission/backfill, other harness locations/readers, supervised worker and controls. The scanner is an incremental inventory, not a running watcher. Enabled startup remains guarded; M17.5 and M17.6/M17.7 gates remain required.
<!-- /evidence-field -->


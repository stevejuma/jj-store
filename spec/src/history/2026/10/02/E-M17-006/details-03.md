---
schema: 1
id: E-M17-006
---

# E-M17-006 details 3

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
pnpm check:boundaries
```

**Notes**

All 17-package ownership boundaries pass; upstream discovery stays CLI-owned.
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
cargo fmt --all -- --check
```

**Notes**

Final Rust formatting passes.
<!-- /run -->

<!-- run -->
### Run 3

```yaml
result: pass
failures: 0
hasNote: true
noteEncoding: markdown

```

```shell
pnpm exec prettier --check spec/src/cli.md spec/src/collaboration.md spec/src/mutations.md spec/src/milestones.md
```

**Notes**

Final changed canonical chapter and ledger formatting passes.
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

JJ 0.45.1; Rust/Cargo 1.98.1; device directory identity qualified on macOS ARM64 Unix.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

New internal VerifiedCaptureWorkspace proof retains existing VerifiedWorkspace bytes and stores a device-only SHA-256 binding witness. No repository objects, operation heads, native publication, receipts, transport schema or collaboration IDs change. The proof retains the original session directory privately so source aliases and later inner boundaries are rechecked.
<!-- /evidence-field -->

<!-- evidence-field:revision markdown -->
## Revision

enclosing Jujutsu change qrpnkzmk
<!-- /evidence-field -->

<!-- evidence-field:scope markdown -->
## Scope

New capture-only discovery capability plus published installation registry membership and exact physical/connection revalidation. Existing explicit root-only registration remains unchanged and root-only implementations default to unavailable for capture. Shared verifier consumers require complete direct CLI discovery and server workspace/admission binaries, affected library checks and strict clippy, and the actual root-inspection HTTP acceptance regression. No watcher startup/lifecycle, UI, RustFS or root aggregate change; source readers, active-session admission, sanitized cursor durability and worker composition remain subsequent M17.2 gates. This proof is not write authorization or a code checkpoint.
<!-- /evidence-field -->


---
schema: 1
id: E-M17-005
---

# E-M17-005 details 4

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
pnpm exec prettier --check scripts/check-boundaries.mjs scripts/governance.test.mjs spec-validator.config.mjs spec/src/architecture.md spec/src/cli.md spec/src/collaboration.md spec/src/security.md spec/src/spec-governance.md spec/src/milestones.md
```

**Notes**

Final changed configuration/governance/canonical formatting passes.
<!-- /run -->

<!-- run -->
### Run 2

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
pnpm check:progress
```

**Notes**

Final bookkeeping rejected future milestone gates in a passing slice remaining list. Moved these future gates to scope; the slice has no unresolved verification gaps.
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
pnpm check:progress
```

**Notes**

Corrected final entry passes for all 129 tasks; M17.2 remains In progress with future gates in scope and task next action.
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

Rust/Cargo 1.98.1; JJ 0.45.1; Kingfisher 2.7.0 at 63dccba231b6b01990c84319d25f92f49428a431.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

Existing spool V1 entries, sanitizer policy, prepared native/local requests and receipt encodings unchanged. Cargo.lock adds only local capture package and moves pinned Kingfisher direct dependency ownership.
<!-- /evidence-field -->

<!-- evidence-field:revision markdown -->
## Revision

enclosing Jujutsu change qptmnynw
<!-- /evidence-field -->

<!-- evidence-field:scope markdown -->
## Scope

Runtime extraction changes shared spool/sanitizer/drainer/private path infrastructure used by CLI hooks, local/native recovery, credential leases and server policy. Run the complete directly affected CLI capture/hook/setup/recovery binaries, moved runtime lease test, server configuration/local installation and governance suite; affected libraries require checks and strict clippy. No model/publication/wire contract changes; unrelated Rust, UI, real RustFS and root aggregate lanes are out of scope. This slice does not enable automatic readers or workers. The additive strict format-2 capture config validates explicit product surfaces/versions, bounds, approved roots and LocalTrusted native write enablement. Enabled preparation fails closed until workers are wired. Shared startup now needs the exact cold executable gate then full directly affected server executable suite; this escalation is recorded before running it. This is config/runtime foundation, not qualified source or enabled worker acceptance. Remaining milestone gates: qualified source readers, verified matching, active backfill, supervision, controls, alias enrichment and privacy; enabled startup stays intentionally unavailable until composition. These are subsequent slices, not gaps in this foundation verification.
<!-- /evidence-field -->


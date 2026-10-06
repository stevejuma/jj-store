---
schema: 1
id: E-M17-014
---

# E-M17-014 details 3

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
pnpm spec:validate > /tmp/jj-store-agent-capture-m172-registry-spec.log 2>&1
```

**Notes**

Five validators, 21 chapters and 160 requirements pass.
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
pnpm spec:first > /tmp/jj-store-agent-capture-m172-registry-first.log 2>&1
```

**Notes**

Spec-first passes for ten protected source/test files and three canonical chapters.
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
pnpm exec prettier --check spec/src/collaboration.md spec/src/architecture.md spec/src/milestones.md > /tmp/jj-store-agent-capture-m172-registry-doc-format.log 2>&1
```

**Notes**

All changed canonical documents pass formatting.
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

Rust 1.98.1; JJ 0.45.1; macOS ARM64; synthetic Codex CLI 0.156.1 and Desktop 0.159.2 transcript headers. Real OS subprocess lease exclusion and killed-process recovery; no real harness execution or RustFS qualification.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

New private source registry JSON version 1, bound to supplied existing installation identity. Opaque IDs derive from installation plus monotonic ordinal. Only source IDs, ordinals and SHA-256 location, exact physical/header witness and physical-lease digests persist. Sources, aliases and file/byte budgets fail closed without history eviction. Existing AgentStore, spool, cursor journal, prepared request and receipt shapes/bytes are unchanged.
<!-- /evidence-field -->

<!-- evidence-field:revision markdown -->
## Revision

enclosing Jujutsu change rqxmzltp
<!-- /evidence-field -->

<!-- evidence-field:scope markdown -->
## Scope

One new capture-runtime registration and leased-reader boundary with private filesystem authority; no existing spool, publication or server consumer is changed. Exact new registry regression precedes complete direct registry/policy, bound/discovery/transcript readers, two exact unit targets, affected all-target check and strict lint, formatting and canonical checkers. Header qualification has no registry side effect. Caller must independently prove repository binding and active-session admission before acquisition; this API does not implement or substitute for those proofs. Known locations retain stable channels across replacement; exact physical/header proof joins overlapping roots/hardlinks; product profile selection does not duplicate channels. Unknown generations of a known physical file await known-location rediscovery. Conflicting mappings, retained inode reuse, exhausted bounds, foreign installations, missing/corrupt catalogs, unsafe ownership/permissions, symlinks and rebound state fail closed. Both source and physical leases remain owned by the actual reader and are checked before/after polling; subprocess death and inherited-description drop release are qualified. Source progress is still supplied from the independent sanitized journal. Complete private-state loss/backup, registry retention/purge and other platforms remain separate gates. Registration is not repository authorization, activity, publication, checkpoint or worker composition. Future M17.2 gates remain per-record repository contexts, active-only backfill and offline tails, Cursor/OpenCode readers, installation principal, fair bounded supervised worker and typed controls. Enabled server preparation remains guarded. M17.5 real harness edits and the full M17.3-M17.7 roadmap remain required; both M17.2 and aggregate M17 remain In progress. No CLI/server startup, model/wire schema, UI lifecycle, real RustFS, installed daemon, publication or root aggregate changed; those lanes are out of scope.
<!-- /evidence-field -->


---
schema: 1
id: E-M13-008
---

# E-M13-008 details 3

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
CARGO_TARGET_DIR=/Users/stevejuma/code/lapismd/jj-store/target cargo test -p jj-store-cli --locked --test local_agent_provenance_cli
```

**Notes**

Final four cold executable cases pass after page-kind guard.
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

102 tasks pass evidence bookkeeping.
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
pnpm exec prettier --check docs/cli/agent.md spec/src/cli.md spec/src/mutations.md spec/src/local-jj.md spec/src/milestones.md
```

**Notes**

Changed documents pass formatting.
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

<!-- evidence-field:deferred list -->
## Deferred

- Automatic hook code causality, local ReviewStore and M13.3 completion remain
  separate gates.
- Native portable capsule carrier, filesystem server/UI, conversion and
  unrelated root aggregates remain unimplemented/out of scope.
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

Rust 1.98.1; JJ/jj-lib/jj-cli 0.45.1; Git 2.55.0; macOS ARM64; explicit freshly built development binary and disposable local repositories.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

Private provenance journal V1 binds repository, actor, original request and canonical intent digest before publication. Local provenance pages V2 bind catalog, selected metadata commit, limit and after key. Native codecs/manifests/WAL unchanged. Local capabilities now advertise explicit agent code anchors.
<!-- /evidence-field -->

<!-- evidence-field:next markdown -->
## Next

Commit qualified provenance CLI, then implement shared ReviewStore with retained portable anchors.
<!-- /evidence-field -->


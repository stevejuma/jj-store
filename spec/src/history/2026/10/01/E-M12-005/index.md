---
schema: 1
id: E-M12-005
---

# E-M12-005

<!-- evidence-field:id markdown -->
## Id

E-M12-005
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M12
- M12.4
<!-- /evidence-field -->

<!-- evidence-field:date markdown -->
## Date

2026-10-01
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

native-git-push-attempts
<!-- /evidence-field -->

<!-- evidence-field:revision markdown -->
## Revision

Enclosing Jujutsu change ylyqvktz.
<!-- /evidence-field -->

<!-- evidence-field:before markdown -->
## Before

GB2 local fetch de1c275c: push/status/recovery unavailable. Exact acceptance target: native selected push with exact original parent, stale lease rejection, dry-run and cache-loss recovery without retransmission. Review found a staged source operation could authorize a push; the exact service regression failed once before the accepted-ancestry and frozen-view proof.
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

JJ 0.45.1;gix 0.87.1;Git 2.55.0;Rust 1.98.1
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

Manifest 4;Git immutable record V1;typed acceptance V1.
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- spec/src/git-interop.md
- crates/jj-store-cli/tests/executable_native.rs
- crates/jj-store-wal/src/publication.rs
- crates/jj-store-wal/src/publication/git_interop.rs
- crates/jj-store-wal/src/publication/git_interop/attempts.rs
- crates/jj-store-service/src/git_bridge.rs
- crates/jj-store-cli/src/git_bridge.rs
- crates/jj-store-cli/src/git_bridge/push.rs
- crates/jj-store-cli/src/git_bridge/transport.rs
- crates/jj-store-cli/src/presentation.rs
- crates/jj-store-cli/src/store_commands.rs
- crates/jj-store-cli/src/commands.rs
- crates/jj-store-wal/src/publication/git_interop/facts.rs
- crates/jj-store-service/tests/git_bridge.rs
- spec/src/cli.md
- spec/src/mutations.md
- spec/src/jj-integration.md
- spec/src/publication.md
- docs/cli/commands.md
- crates/jj-store-jj/src/git_bridge/mod.rs
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages

- [E-M12-005 details 2](details-02.md)
- [E-M12-005 details 3](details-03.md)

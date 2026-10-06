---
schema: 1
id: E-M12-010
---

# E-M12-010

<!-- evidence-field:id markdown -->
## Id

E-M12-010
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M12
- M12.7
<!-- /evidence-field -->

<!-- evidence-field:date markdown -->
## Date

2026-10-01
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

native-git-credentials-and-real-rustfs-cycle
<!-- /evidence-field -->

<!-- evidence-field:revision markdown -->
## Revision

Enclosing Jujutsu change rkqwtxnl.
<!-- /evidence-field -->

<!-- evidence-field:before markdown -->
## Before

GB0-GB5 qualified. Real-storage external Git merge/browser gate and approved CA/SSH/helper boundary were not qualified. New authenticated HTTPS exact target fails because Git environment strips explicit CA trust.
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

macOS ARM64 (Darwin arm64); JJ 0.45.1; gix 0.87.1; Git 2.55.0; Rust 1.98.1; rustix 1.1.5; owned embedded RustFS binary; generated fixture-only HTTPS and SSH credentials
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

No native authority/contract change. Qualified jj-git-sha1-0.45.1-v1 bookmark profile. Device credential-helper allowlist and absolute approved executable/CA paths remain invocation-local; native Git facts still use manifest-selected format-4 authority and reducer-2 data-only code origins.
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- AGENTS.md
- crates/jj-store-cli/src/commands.rs
- crates/jj-store-cli/src/git_bridge/push.rs
- crates/jj-store-cli/src/git_bridge/transport.rs
- crates/jj-store-cli/src/git_bridge/transport/credentials.rs
- crates/jj-store-cli/tests/executable_native.rs
- crates/jj-store-cli/tests/git_bridge/https.py
- crates/jj-store-cli/tests/server_executable.rs
- crates/jj-store-cli/tests/server_executable/git_bridge.rs
- docs/cli/commands.md
- spec/src/api.md
- spec/src/cli.md
- spec/src/git-interop.md
- spec/src/milestones.md
- spec/src/security.md
- spec/src/spec-governance.md
- web/tests/real/git-bridge.spec.ts
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages

- [E-M12-010 details 2](details-02.md)
- [E-M12-010 details 3](details-03.md)

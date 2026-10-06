---
schema: 1
id: E-M9-033
---

# E-M9-033 details 2

<!-- evidence-field:versions markdown -->
## Versions

Rust 1.98.1; Codex CLI 0.156.1; OpenCode 1.18.30; Cursor 2026.09.26-dd393fe explicitly unsupported against 3.22.7; MongoDB Kingfisher 2.7.0 at 63dccba231b6b01990c84319d25f92f49428a431 with scanner default features disabled.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

No native object, manifest, WAL, receipt, AgentStore, ReviewStore or wire encoding changes. Native copy is a process-local bounded API over the existing ObjectStore and DependencySource contracts; it conditionally copies exact immutable bytes and installs the existing canonical manifest last. Remote export, backup, migration, failover and promotion remain deferred.
<!-- /evidence-field -->


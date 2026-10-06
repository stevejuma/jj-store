---
schema: 1
id: E-M15-009
---

# E-M15-009

<!-- evidence-field:id markdown -->
## Id

E-M15-009
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M15
- M15.3
<!-- /evidence-field -->

<!-- evidence-field:date markdown -->
## Date

2026-10-01
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

native-unix-rpc
<!-- /evidence-field -->

<!-- evidence-field:revision markdown -->
## Revision

enclosing Jujutsu change vrzlqqpx
<!-- /evidence-field -->

<!-- evidence-field:before markdown -->
## Before

Clean isolated workspace at 133341fc. Exact RPC collab target absent. Native service commands and signed pagination exist without registered protobuf methods or typed client access.
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

Rust 1.98.1; generated prost/tonic native V1 handlers and typed client; actual owner-admitted Unix gRPC over disposable MemoryStore; pnpm 10.32.1. No installed binary, runner, service or credential files changed.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

Additive native V1 protobuf methods for command/status/page/detail/message/assets. Service-model reexport avoids an upstream JJ or new external dependency. Four-MiB typed JSON; separately bounded two-MiB binary image field with empty inline JSON bytes; image response verifies metadata, content identity and repository. Persistence formats unchanged.
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- crates/jj-store-rpc/proto/native.proto
- crates/jj-store-rpc/src/collab.rs
- crates/jj-store-rpc/src/collab/codec_tests.rs
- crates/jj-store-rpc/src/client.rs
- crates/jj-store-rpc/src/server.rs
- crates/jj-store-rpc/src/lib.rs
- crates/jj-store-rpc/tests/collab.rs
- crates/jj-store-service/src/collab_reads.rs
- crates/jj-store-service/src/lib.rs
- spec/src/api.md
- spec/src/architecture.md
- spec/src/cli.md
- spec/src/collaboration.md
- spec/src/jj-integration.md
- spec/src/mutations.md
- spec/src/milestones.md
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages

- [E-M15-009 details 2](details-02.md)
- [E-M15-009 details 3](details-03.md)

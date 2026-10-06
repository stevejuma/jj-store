---
schema: 1
id: E-READ-044
---

# E-READ-044

<!-- evidence-field:id markdown -->
## Id

E-READ-044
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M13.4
<!-- /evidence-field -->

<!-- evidence-field:date markdown -->
## Date

2026-10-04
<!-- /evidence-field -->

<!-- evidence-field:revision markdown -->
## Revision

enclosing Jujutsu change tkttkswl
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

Prepared native visible reference windows with selected-target hydration and exact-snapshot HTTP recovery
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

Rust1.98.1 (48a229cea 2026-09-01); jj-lib0.45.1; JJ0.45.1; macOS ARM64.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

No native/index/WAL/companion/receipt, public DTO/schema, cursor or CLI format change. Native resident visibility now includes sorted textual change keys and sorted visible target IDs from the index commit/change stream. Conservatively reserves512bytes/commit plus existing heads/labels/base allowance before projection growth; resident weight and global idle eviction include it. Reference selection and target/cache copies have request-local memory leases; existing content work/read/output/deadline/page/group bounds retained. Selected pages borrow the freshly verified request-scoped native view; only chosen bookmark terms are converted, with no duplicate operation/view IO or whole-view DTO copy.
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- crates/jj-store-jj/src/query/mod.rs
- crates/jj-store-jj/src/query/projection.rs
- crates/jj-store-jj/src/query/workers.rs
- crates/jj-store-jj/src/query/content.rs
- crates/jj-store-jj/src/query/content/references.rs
- crates/jj-store-jj/src/query/content/references/prepared.rs
- crates/jj-store-service/src/pagination/browsing.rs
- crates/jj-store-cli/tests/native_lifecycle/reference_windows.rs
- crates/jj-store-cli/tests/native_lifecycle/http_reads.rs
- crates/jj-store-cli/tests/native_lifecycle/query_workers.rs
- docs/implementation-plan.md
- spec/src/api.md
- spec/src/architecture.md
- spec/src/cli.md
- spec/src/compatibility.md
- spec/src/jj-integration.md
- spec/src/milestones.md
- spec/src/mutations.md
<!-- /evidence-field -->

<!-- evidence-field:result markdown -->
## Result

pending
<!-- /evidence-field -->

<!-- evidence-field:remaining list -->
## Remaining

- 34 unique focused Rust tests pass, including final exact reference/HTTP/worker
  reruns. Native unfiltered Changes/All performance delivery complete without
  full qualification or milestone promotion. Pages select indexed keys before
  target hydration and borrow fresh verified operation/view data; prepared
  visibility never grants membership or authority.
- Nonempty native/local description filtering still needs indexed
  projection/query keys. Native standalone metadata readers and a differently
  embedded reference operation keep their previous separately bounded reader;
  the serving composition uses the new exact-operation prepared path.
- Indexed operation child neighbors, deep operation/predecessor/copy/search
  continuations, retained oversized difference/change-group discovery, paged
  native RPC and remaining local application search/index delivery remain open.
- Complete combined integration/contracts, Linux/native2million/near-capacity
  fixtures, measured RSS/release latency, full cold production browser,
  fault/revocation/restart/publication/real-RustFS, registry/release and
  Help/source-review refresh remain deferred. Public schemas and UI production
  code unchanged; UI suites/build and contract regeneration are out of scope for
  this delivery. Main/Linux source and original dev services untouched; retained
  workspace remains open. Visual baselines unapproved; inherited __eh_frame
  linker warning and earlier strict-lint gaps separately deferred.
<!-- /evidence-field -->

<!-- evidence-field:next markdown -->
## Next

Continue indexed native/local description filtering, operation child lookup and retained deep/difference/search/RPC continuations in this workspace; qualify the complete combined implementation separately.
<!-- /evidence-field -->

<!-- evidence-field:scope markdown -->
## Scope

Native prepared visibility/content composition and signed browser recovery. Focused graph/cache/worker and existing native transport consumers run because execution and resident derivation are shared. No full Rust/UI/repository aggregate or combined qualification; no installed executable replacement, source mutation or original server restart.
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages

- [E-READ-044 details 2](details-02.md)
- [E-READ-044 details 3](details-03.md)
- [E-READ-044 details 4](details-04.md)

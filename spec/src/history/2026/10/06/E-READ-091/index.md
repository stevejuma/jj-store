---
schema: 1
id: E-READ-091
---

# E-READ-091

<!-- evidence-field:id markdown -->
## Id

E-READ-091
<!-- /evidence-field -->

<!-- evidence-field:date markdown -->
## Date

2026-10-06
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M4.6
- M13.4
<!-- /evidence-field -->

<!-- evidence-field:requirements list -->
## Requirements

- JST-JJ-003
- JST-LOCAL-004
- JST-GOV-007
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

Progressive Blame feasibility stopped at upstream synchronous comparison gate
<!-- /evidence-field -->

<!-- evidence-field:revision markdown -->
## Revision

enclosing Jujutsu change wuyoqxpo
<!-- /evidence-field -->

<!-- evidence-field:before markdown -->
## Before

No Blame route, controller or UI exists. The agreed first slice must qualify incremental JJ annotation and bounded execution before exposing contracts. Main was clean at e7483f29; retained bounded-reads workspace was clean and was advanced to that parent using jj new. The development servers were not restarted.
<!-- /evidence-field -->

<!-- evidence-field:scope markdown -->
## Scope

Test-only FileAnnotator domain/correctness probes and an opt-in Linux probe through existing scoped local readers and privately prepared core index. No production runtime, API, DTO, generated contract, UI, cache or installed binary change. Stop at the explicit feasibility gate after measuring a single synchronous comparison that exceeds the cooperative deadline and warm target.
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

macOS ARM64; pinned jj-lib/jj-core 0.45.1; cargo development test profile (unoptimized, debug=0); current repository toolchain and dependencies.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

Persisted state, identities, HTTP/RPC contracts and generated consumers unchanged. New probes compile only under cfg(test). No upstream source, dependency patch or annotation fork.
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- crates/jj-store-jj/src/query/mod.rs
- crates/jj-store-jj/src/query/blame_probe.rs
- crates/jj-store-jj/src/local_browser/code_index.rs
- crates/jj-store-jj/src/local_browser/code_index/blame_probe.rs
- spec/src/jj-integration.md
- spec/src/local-jj.md
- spec/src/spec-governance.md
- spec/src/milestones.md
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- crates/jj-store-jj/src/query/blame_probe.rs#incremental_domains_match_complete_annotation_for_edits_and_merges
- crates/jj-store-jj/src/query/blame_probe.rs#synchronous_comparison_honors_progress_deadline
- crates/jj-store-jj/src/local_browser/code_index/blame_probe.rs#linux_first_window_uses_guarded_pinned_reader
<!-- /evidence-field -->


<!-- evidence-relations -->
## Resolution and supersession

The original result remains **blocked**. Milestone gates select their own evidence.

- [E-READ-094](../E-READ-094/index.md) supersedes this entry. Accepted cooperative cancellation replaces the proposed hard comparison deadline; the original failed probe remains a failure.
<!-- /evidence-relations -->
<!-- generated-navigation -->
## Pages

- [E-READ-091 details 2](details-02.md)
- [E-READ-091 details 3](details-03.md)

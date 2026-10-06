---
schema: 1
id: E-DESK-001
---

# E-DESK-001

<!-- evidence-field:id markdown -->
## Id

E-DESK-001
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M17
- M17.1
- M17.2
<!-- /evidence-field -->

<!-- evidence-field:date markdown -->
## Date

2026-10-02
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

desktop-foundation
<!-- /evidence-field -->

<!-- evidence-field:revision markdown -->
## Revision

ykmzsnwm (enclosing change)
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

macOS ARM64; Rust 1.98.1; JJ 0.45.1; Tauri 2.11.5 with Buzz-pinned macros/codegen/plugin 2.6.3, runtime 2.11.3, runtime-wry 2.11.4 and utils 2.9.3; existing pinned real RustFS helper reused. Existing ignored node_modules development tree reused; inherited Design Core substitution is not registry-only foundation qualification.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

New independent desktop lock and version-1 private device port settings. Format-2 installation, native object/WAL/receipt bytes, HTTP schema and browser CSP unchanged. Opt-in supervised run flags override bundle paths and bind only in memory. Parent stdin is a nonblocking Unix pipe; EOF uses existing graceful server shutdown.
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- .gitignore
- Cargo.toml
- crates/jj-store-cli/src/lib.rs
- crates/jj-store-cli/src/server_commands.rs
- crates/jj-store-cli/src/server_desktop.rs
- crates/jj-store-cli/src/server_process.rs
- crates/jj-store-cli/tests/desktop_supervision.rs
- desktop/Cargo.lock
- desktop/Cargo.toml
- desktop/README.md
- desktop/build.rs
- desktop/icons/icon.png
- desktop/src/installation.rs
- desktop/src/lib.rs
- desktop/src/main.rs
- desktop/src/native.rs
- desktop/src/process.rs
- desktop/src/readiness.rs
- desktop/src/runtime.rs
- desktop/tauri.conf.json
- desktop/tests/managed_runtime.rs
- desktop/tests/owned_process.rs
- desktop/ui/index.html
- docs/cli/server-run.md
- package.json
- scripts/check-boundaries.mjs
- scripts/check-progress.mjs
- scripts/desktop.mjs
- scripts/governance.test.mjs
- spec/src/SUMMARY.md
- spec/src/architecture.md
- spec/src/cli.md
- spec/src/desktop.md
- spec/src/milestones.md
- spec/src/spec-governance.md
- spec/src/verification.md
- spec-validator.config.mjs
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- crates/jj-store-cli/tests/desktop_supervision.rs#desktop_flags_require_supervision_and_a_pipe_before_startup
- crates/jj-store-cli/tests/desktop_supervision.rs#desktop_owner_eof_stops_real_service_and_retains_installation
- desktop/src/readiness.rs#readiness_rejects_redirects_foreign_modes_and_unready_storage
- desktop/src/readiness.rs#navigation_preserves_only_exact_managed_origin
- desktop/tests/managed_runtime.rs#real_managed_runtime_restarts_preserves_origin_and_rejects_occupied_port
- desktop/tests/owned_process.rs#owner_finish_closes_stdin_and_waits_for_clean_child_exit
- desktop/tests/owned_process.rs#oversized_readiness_is_rejected_without_unbounded_capture
- desktop/tests/owned_process.rs#stderr_capture_remains_bounded_while_child_output_is_drained
- scripts/governance.test.mjs#service_owned_review_reader_is_within_layer_boundary
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages

- [E-DESK-001 details 2](details-02.md)
- [E-DESK-001 details 3](details-03.md)
- [E-DESK-001 details 4](details-04.md)

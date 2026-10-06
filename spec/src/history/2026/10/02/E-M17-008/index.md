---
schema: 1
id: E-M17-008
---

# E-M17-008

<!-- evidence-field:id markdown -->
## Id

E-M17-008
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M17
- M17.2
<!-- /evidence-field -->

<!-- evidence-field:date markdown -->
## Date

2026-10-02
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

bounded-codex-file-source
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- crates/jj-store-capture/src/codex_transcript/file.rs
- crates/jj-store-capture/src/codex_transcript.rs
- crates/jj-store-capture/src/lib.rs
- crates/jj-store-capture/src/profile.rs
- crates/jj-store-capture/tests/codex_transcript.rs
- crates/jj-store-server/tests/configuration.rs
- spec/src/collaboration.md
- spec/src/compatibility.md
- spec/src/milestones.md
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- crates/jj-store-capture/tests/codex_transcript.rs#codex_transcript_reader_accepts_only_exact_bound_product_surface
- crates/jj-store-capture/tests/codex_transcript.rs#complete_tails_retry_exactly_and_resume_only_after_sanitized_durability
- crates/jj-store-capture/tests/codex_transcript.rs#partial_utf8_and_invalid_complete_records_preserve_exact_byte_positions
- crates/jj-store-capture/tests/codex_transcript.rs#oversized_records_are_bounded_omissions_and_later_records_remain_readable
- crates/jj-store-capture/tests/codex_transcript.rs#replacements_truncation_and_same_size_rewrites_create_new_generations
- crates/jj-store-capture/tests/codex_transcript.rs#resume_verifies_large_prefix_in_bounded_steps_and_does_not_reemit_it
- crates/jj-store-capture/tests/codex_transcript.rs#missing_headers_unsafe_paths_and_changed_resume_identity_fail_closed
- crates/jj-store-capture/tests/codex_transcript.rs#rewrite_between_prefix_verification_steps_cannot_skip_changed_history
- crates/jj-store-capture/tests/codex_transcript.rs#batch_record_limit_preserves_every_later_record
- crates/jj-store-server/tests/configuration.rs#automatic_capture_configuration_is_explicit_local_writable_and_bounded
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages

- [E-M17-008 details 2](details-02.md)
- [E-M17-008 details 3](details-03.md)

# Search configuration identity recovery

Two regressions initially failed. Native runners accepted an extractor with a
mismatched configuration digest. Reopening an existing index with newly excluded
source paths activated the old generation before rebuilding; forcing the rebuild
to fail left that incompatible generation searchable.

The projection contract now validates identity before runner ownership or I/O.
Native extraction binds all identity fields to its canonical configuration.
Recovery checks compatibility before activating readers; an incompatible
artifact requires rebuilding. A failed replacement build leaves no newly served
generation.

The native fixtures prove rejection for configuration, implementation, schema
and adapter-name mismatches; restart at the same repository position; failed
rebuild without old source exposure; and successful retry with the new
exclusions. Tests use actual local Tantivy generations over native memory-backed
repository data. Repository formats and identities remain unchanged. E-M6-007 in
the [single ledger](../../spec/src/milestones.md) records exact runs and
remaining server, snapshot and deployment gaps.

Final validation passed both original regressions, all 22 native projection
fixtures, ten adapter delivery fixtures and ten Tantivy durability fixtures.
Normal checks, build, all 13 specification lanes and formatting pass. The full
test lane passed 346 Rust tests with zero failures and six external tests
ignored, plus governance, harness, contract and existing web unit checks.

# Third-party references

Jujutsu (The Jujutsu Authors), jj-cli and jj-lib 0.45.1, Apache-2.0. No upstream
implementation source is copied in the bootstrap slice. Commit Cloud and WalGit
are design references; audit exact revision and license before copying or
adapting source. Runtime/build dependencies retain their own licenses. RustFS is
a separately distributed deployment dependency.

The unmodified upstream custom-backend example is copied under
`crates/jj-store-cli/examples/upstream_custom_backend.rs` from JJ revision
`7c41cdeb16b6b321c64e789a966b6adf723816a5`; it retains its Apache-2.0 header. It
is a Git-backed extension proof, not the native production backend.

Typed native abandonment in `crates/jj-store-jj/src/commands/abandon.rs` adapts
the transaction algorithm from `cli/src/commands/abandon.rs` at that same JJ
revision (Copyright 2020 The Jujutsu Authors, Apache-2.0). It uses native
`transform_descendants_with_options` and `RewriteRefsOptions`; server admission,
authorization, report and publication boundaries are project-owned.

Tantivy 0.26.2, MIT (Quickwit contributors), is consumed as a released crates.io
dependency with checksum in Cargo.lock. Its source license remains available at
https://github.com/quickwit-oss/tantivy/blob/72d1ef9a6468aa68bbc69dcc80cdf60aaf64364d/LICENSE.
No upstream Tantivy implementation source is copied.

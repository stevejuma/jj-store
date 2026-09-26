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

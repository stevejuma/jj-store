# M0 extension qualification

Native compatibility probes are normal Rust test targets. Their source/fixture
updates require canonical chapter changes and same-change ledger evidence.

The external compiled-storage lane is explicitly opt-in with the isolated RustFS
harness. Its immutable report records the command and actual test output; normal
test output must report this deployment test as ignored, not passed.

The opt-in `pnpm test:local-release` lane is distinct from captured-response
browser tests. It must cold-build, initialize a persistent format-2 LocalTrusted
installation, start checkout-owned embedded RustFS and the native server,
auto-open the sole catalog repository without credentials, publish from another
native CLI process over the owner-only Unix socket, observe token-free SSE
without automatic snapshot replacement, complete an explicit refresh, and record
cold/restart startup time plus server/RustFS RSS against the macOS ARM64
ceilings. `JJ_STORE_REAL_KEEP=1` may retain the owned stack for review;
otherwise the lane cleans it up. Runtime metadata and Playwright artifacts
record no secret. The development stack additionally configures token-free
LocalTrusted search and waits for its first current Tantivy generation before it
reports readiness. Authenticated/shared deployment, registry-only M4
finalization and shared deployment acceptance are deferred and do not gate this
lane.

Publication faults are exercised in the normal WAL tests. The opt-in deployment
lane records both the initial compiled native publication run and a separate
post-kill recovery process; neither is inferred from normal ignored tests.

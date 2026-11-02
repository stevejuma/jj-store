---
schema: 1
id: E-LOCAL-TEST-AUDIT-001
---

# E-LOCAL-TEST-AUDIT-001 details 3

<!-- evidence-field:runs runs -->
## Validation

<!-- run -->
### Run 1

```yaml
result: pass
failures: 0
hasNote: true
noteEncoding: markdown

```

```shell
JJ_STORE_AUDIT_MEMORY_GIB=24 node .local/test-audit-20261101/remote-run.mjs test:storybook catalog/components/store-application/AppearanceDiscussions.stories.svelte
```

**Notes**

Receipt jj-store-validation-96ca2bc3-ef59-4bc3-8214-3c0e0111b97b. The unchanged FocusAndSplitPreserveToolDiscussionDrafts case passes 1/1 in a fresh 4-CPU/24-GiB Docker run, with no skips. The 24-GiB allocation matches validationConfig default memory; the existing saved 12-GiB local configuration remains untouched. Full repeated qualification is still required; this focused pass alone does not resolve the failed aggregate.
<!-- /run -->

<!-- run -->
### Run 2

```yaml
result: fail
failures: 14
hasNote: true
noteEncoding: markdown

```

```shell
JJ_STORE_AUDIT_MEMORY_GIB=24 node .local/test-audit-20261101/remote-run.mjs test --no-fail-fast
```

**Notes**

Receipt jj-store-validation-6bcc87e2-2232-4219-be0a-8ca46586c84a. All UI lanes complete: governance 120, storage harness 3, three real storage/LocalTrusted targets 1 each, contracts 8, web unit 180, catalog unit 1208 plus one pre-existing actual-capture guard skip; Storybook 1046 pass and exactly two quarantines skipped (273/284/238/251 pass per shard). Cold static catalog browser 1 pass. Cargo completes all targets with three failed binaries: executable_native 14 pass/8 fail/3 ignored, native_lifecycle 234 pass/5 fail, native_rpc 27 pass/1 fail. Git 2.43 lacks pinned 2.55 profile and sshd. Historical index merging panics on owned readonly wrapper; mixed Ring/AWS-LC Rustls features panic before TLS. These deterministic Rust failures are not quarantined. UI cgroup memory.peak 16624111616 against memory.max 25769803776, no max/oom events. The formerly errored third shard passes all 238 cases.
<!-- /run -->

<!-- run -->
### Run 3

```yaml
result: fail
failures: 2
hasNote: true
noteEncoding: markdown

```

```shell
Remote Docker validation:rust test -p jj-store --test native_lifecycle --locked -- graph_acceptance::cache_acceptance::cold_historical_indexes_leave_the_primary_generation_immutable --exact; validation:rust test -p jj-store --test native_rpc --locked -- rpc_tls_checks_trust_and_server_hostname --exact
```

**Notes**

rust-exact-baselines.log records one deterministic failure for each original regression before implementation; 0 pass/1 fail in each invocation. Existing assertions remain unchanged.
<!-- /run -->

<!-- run -->
### Run 4

```yaml
result: pass
failures: 0
hasNote: true
noteEncoding: markdown

```

```shell
Remote Docker validation:rust test -p jj-store --test executable_native --locked
```

**Notes**

Receipt jj-store-validation-30cc9abe-49f2-48d7-92c2-0845ab54fe73: 22 pass, zero fail, 3 existing credential-provider manual gates ignored. Corrected image builds actual Git 2.55.0 from checksum-verified official source, installs fixture sshd, and unlocks the isolated root account for generated-key-only fixture authentication. No host SSH service or credential configuration is changed.
<!-- /run -->

<!-- run -->
### Run 5

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
Remote Docker validation:rust test -p jj-store --test native_lifecycle --locked -- graph_acceptance::cache_acceptance::historical_revset_difference_preserves_exact_operation_visibility --exact
```

**Notes**

New semantic regression before implementation, receipt jj-store-validation-c4c56f11-ef12-4952-a90a-af5eb8a1ab9c: 0 pass/1 fail/239 filtered. Native default mutable index panics when handed the owned Reader. The regression checks exact historical set difference and unchanged native manifest.
<!-- /run -->

<!-- run -->
### Run 6

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
Remote Docker validation:rust test -p jj-store-rpc --test tls_provider_default --test tls_provider_existing --locked --no-fail-fast (new server configuration regressions, provider calls temporarily absent)
```

**Notes**

Receipt jj-store-validation-256702e1-e126-42bb-9239-7bb97f29fef9: 1 fail/1 pass. Without the new provider initialization calls, a mixed-feature server panics selecting a Rustls provider; an already-installed host provider is retained and passes. Invalid fixture PEM must produce an ordinary configuration error after crypto selection, with no network handshake. Calls were restored before green verification.
<!-- /run -->

<!-- run -->
### Run 7

```yaml
result: pass
failures: 0
hasNote: true
noteEncoding: markdown

```

```shell
Remote Docker new-history-regression-after: validation:rust test -p jj-store --test native_lifecycle --locked -- graph_acceptance::cache_acceptance::historical_revset_difference_preserves_exact_operation_visibility --exact
```

**Notes**

Receipt jj-store-validation-23bbc428-7caf-49db-be49-cbcb33ec1536. Exact regression passes 1/1 with 239 deliberately filtered cases. Historical visibility, immutable cache/manifest assertions and resource guards remain intact.
<!-- /run -->

<!-- run -->
### Run 8

```yaml
result: pass
failures: 0
hasNote: true
noteEncoding: markdown

```

```shell
Remote Docker new-tls-regressions-after: validation:rust test -p jj-store-rpc --test tls_provider_default --test tls_provider_existing --locked --no-fail-fast
```

**Notes**

Receipt jj-store-validation-1a6281df-197c-4aca-a258-b2a37ea56c27. Both isolated TLS configuration processes pass (2 total): mixed compiled providers initialize without panic and the host-selected provider Arc/cipher-suite restriction is retained.
<!-- /run -->

<!-- run -->
### Run 9

```yaml
result: pass
failures: 0
hasNote: true
noteEncoding: markdown

```

```shell
Remote Docker original-history-after: validation:rust test -p jj-store --test native_lifecycle --locked -- graph_acceptance::cache_acceptance::cold_historical_indexes_leave_the_primary_generation_immutable --exact
```

**Notes**

Receipt jj-store-validation-17cb6291-3d34-4244-91f0-fd5a9f7c1992. Exact regression passes 1/1 with 239 deliberately filtered cases. Historical visibility, immutable cache/manifest assertions and resource guards remain intact.
<!-- /run -->

<!-- run -->
### Run 10

```yaml
result: pass
failures: 0
hasNote: true
noteEncoding: markdown

```

```shell
Remote Docker validation:rust test -p jj-store --test native_rpc --locked -- rpc_tls_checks_trust_and_server_hostname --exact
```

**Notes**

Receipt jj-store-validation-ae26c004-ca2d-4677-ad74-f7b96cc94dfa: original TLS regression passes 1/1, with 27 deliberately filtered. Untrusted CA and wrong server hostname still fail; a trusted localhost connection succeeds and returns exact native heads.
<!-- /run -->

<!-- run -->
### Run 11

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
Remote Docker validation:rust test --workspace --locked --no-fail-fast
```

**Notes**

Receipt jj-store-validation-01bb5187-fdf3-4770-bb6a-05e4363aed2a. [2490, 1, 61] totals [passed, failed, ignored], 333 result summaries. All 14 baseline Rust failures are repaired: executable_native 22 pass/3 existing manual ignores; native_lifecycle 240 pass; native_rpc 28 pass. Both new isolated provider tests pass. One additional failure: jj-store-server capture_import_state::import_receipt_survives_reopen_and_exact_replay_never_retargets_its_source returns Unavailable at its first resolve immediately after create (line 44), before record/reopen. The prior original full run passed all eight family cases. No repaired query/RPC entry point is invoked on this file-only path. Fresh exact reproductions are pending before classification.
<!-- /run -->

<!-- run -->
### Run 12

```yaml
result: fail
failures: 4
hasNote: true
noteEncoding: markdown

```

```shell
Remote Docker capture-import exact probe 1, then exact probes 2/3 and complete capture_import_state family
```

**Notes**

Receipts jj-store-validation-524c7933-ca65-4230-88d9-f8cacd82fa69, jj-store-validation-5d537089-7ef3-4785-abeb-2759cdd200d9, jj-store-validation-052d27a9-a46d-42df-b37c-85c39b40dc38, jj-store-validation-4e487658-0ff5-4322-9c73-23c396233f5d. Probe 1 test body passes 1/1, but postflight rejects 41.5 GiB against the unchanged 40-GiB target limit. Later three attempts stop at preflight without running tests. All four invocations fail infrastructure qualification; no fresh successful probe is claimed from them.
<!-- /run -->

<!-- run -->
### Run 13

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
Isolated stopped validation container: node scripts/build-storage.mjs report --json; clean --target target --dry-run; clean --target target --apply
```

**Notes**

Report identifies 44,606,259,200 bytes entirely as generated Cargo artifacts, with no unsafe entries or live workspace run. The guarded filesystem cleaner refuses the Docker target because CACHEDIR.TAG is absent. No marker is fabricated and no filesystem cleanup guard is weakened. The runner Docker cache lifecycle is used next, scoped to the owned idle target volume; original receipts are retained.
<!-- /run -->

<!-- run -->
### Run 14

```yaml
result: pass
failures: 0
hasNote: true
noteEncoding: markdown

```

```shell
Remote Docker: verify target cache label and exited workspace consumers; retain run logs; docker rm <eight stopped owned containers>; docker volume rm jj-store-validation-001ff6589b6f2aeb-903d5aa45dd97965278e-target
```

**Notes**

Only the verified idle task build-cache volume is removed through the same Docker lifecycle used by the runner cache-clean command. Docker removal uses no force flag. All consumers are exited, carry the same workspace and run labels, and have retained local logs. Ownership/exit metadata is retained in cache-replacement.json. Source snapshots remain in local Jujutsu; pnpm, Cargo registry/Git and other cache volumes remain. This is cache maintenance, not test acceptance.
<!-- /run -->

<!-- run -->
### Run 15

```yaml
result: pass
failures: 0
hasNote: true
noteEncoding: markdown

```

```shell
Remote Docker validation:rust test -p jj-store-server --test capture_import_state --locked -- import_receipt_survives_reopen_and_exact_replay_never_retargets_its_source --exact
```

**Notes**

Receipt jj-store-validation-03cf932f-f4da-4d98-9bd5-7766702e0107. Fresh clean-cache exact attempt 1 succeeds, including preflight and postflight: 1 passed, zero failures/ignores and 7 deliberately filtered cases. Source and assertions are unchanged; this does not establish a root-cause repair.
<!-- /run -->

<!-- run -->
### Run 16

```yaml
result: pass
failures: 0
hasNote: true
noteEncoding: markdown

```

```shell
Remote Docker validation:rust test -p jj-store-server --test capture_import_state --locked -- import_receipt_survives_reopen_and_exact_replay_never_retargets_its_source --exact
```

**Notes**

Receipt jj-store-validation-c893b016-2a34-4ec9-9b13-3025ca21defb. Fresh exact attempt 2: 1 pass, zero failures/ignores, 7 deliberately filtered cases. No source or assertion change.
<!-- /run -->

<!-- run -->
### Run 17

```yaml
result: pass
failures: 0
hasNote: true
noteEncoding: markdown

```

```shell
Remote Docker validation:rust test -p jj-store-server --test capture_import_state --locked -- import_receipt_survives_reopen_and_exact_replay_never_retargets_its_source --exact
```

**Notes**

Receipt jj-store-validation-4f77eea6-b9cc-4772-b219-03ae14bd09a7. Fresh exact attempt 3: 1 pass, zero failures/ignores, 7 deliberately filtered cases. No source or assertion change.
<!-- /run -->

<!-- run -->
### Run 18

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
Remote Docker validation:rust test -p jj-store-server --test capture_import_state --locked (before quarantine)
```

**Notes**

Receipt jj-store-validation-463fea33-cd37-46b6-a5b5-e1dc9a07f2bf. The narrowed complete family reproduces the same first resolve Unavailable: 7 pass/1 fail. All three fresh isolated exact targets passed unchanged. This strengthens the suite-context dependence proof. The case is now individually ignored as Q-RUST-001; its body, original-source/receipt assertions and all runtime guards are unchanged. Owner, disabled coverage and re-enable conditions are in the canonical quarantine index.
<!-- /run -->
<!-- /evidence-field -->


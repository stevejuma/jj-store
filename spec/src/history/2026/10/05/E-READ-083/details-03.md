---
schema: 1
id: E-READ-083
---

# E-READ-083 details 3

<!-- evidence-field:notes list -->
## Notes

- Original Linux registration 0204aecea848ac87410ad8a606239422 remains
  generation 2 and operation
  c29523a65dc707ab052cd66095169427e17ca54eebefc2ee497fb2fe6d78d5772a91ae9f071aa78e0565b585f2ab8b9dff734e3d47e93899683ee7070f66283e.
  Source /Users/stevejuma/code/linux/.jj/repo is read-only. Local master remains
  062871f1371b2e02a272ff5279c6479aff0a37ef; all() correctly includes newer
  remote-origin commits in this captured operation.
- Actual browser loaded root and lib file lists with independent initially
  Loading row metadata, then 50-row Commits. Selected
  a74306e2e676f9775457366fc047a660fbf02f26 exposes two changed files and the
  drivers/clk/ti/composite.c text diff. Final cold DOM has text hunks, no
  rebuild notice and no Loading selected comparison; screenshot
  linux-cold-final.png. A 15-second settled Network.requestWillBeSent
  observation has no /v1 data requests (linux-idle-network.json); this is not
  the five-minute renewal/release gate.
- "Initial Linux API proof: eleven 200 responses, pinned
  root/lib/content/metadata/bookmarks/commit/diff reads and three 50-row history
  pages with 150 distinct commits and 54 metadata nodes each. Debug timings
  roughly 1-1.3s point reads and 2-2.6s history pages. Final seven post-restart
  probes all pass: readiness 0.310s, 50-row log 1.970s, selected text diff
  1.149s; native two-row bookmark window 0.229s and exact conflict/demo lookup
  0.285s. These are observations, not a latency/RSS benchmark."
- Two retained Linux standard-index segments keep identical checksums, lengths
  and mtimes across both restarts. All 18 source index/op-head/extra-head files
  and 89,135,722 bytes remain checksum-identical. Private warm reuse is proven;
  native Linux-sized cold construction is still not qualified and may hit owned
  builder allocation admission.
- "Actual cold acceptance exposed two product defects now fixed in E081/E082:
  redundant exact change lookup blocked selected graph navigation; background
  Busy invented unavailable repository witnesses and a read/refresh feedback
  loop. Failed UI/monitor regressions and before-live traces are preserved.
  Direct startup probes also saw bounded Busy while the browser was loading;
  final settled corrected probes all succeed."
- "Preview prerequisite repair retained lockfile bytes and tracked clean status:
  refreshed stale node_modules through existing source-validation hooks after
  frozen pnpmfileChecksum mismatch, then restored pnpm-lock.yaml. Help assets
  were copied from main; accidentally snapshotted ignored docs/build files were
  untracked after advancing source. No registry-only consumer qualification is
  claimed."
- "Initial API harness errors are retained separately: bodyless GET Content-Type
  rejected, flattened snapshot shape misread, native references accidentally
  sent as POST (405), and source scan initially omitted store/extra/heads.
  Corrected harness verifies the actual contract; no admission or assertions
  were weakened."
- "Evidence copied into main byte-identically: landing-evidence-copy.json
  retains 1612 source artifacts; final-source-evidence-copy.json retains 28
  correction artifacts. Existing differing files were preserved in
  retained-source-evidence. Earlier 523 ledger entries remain unchanged.
  Inherited macOS linker unwind-table warning remains."
- No Linux source write/repair, fixture reset/migration, installed CLI
  replacement, package publication, push, unrelated service restart or
  visual-baseline change. Independent 7460/7462 listener PIDs 53716/36259 remain
  unchanged. Current preview remains LocalTrusted HMR; production built-bundle
  and released-registry acceptance stay separate.
<!-- /evidence-field -->

<!-- evidence-field:remaining list -->
## Remaining

- Advanced search/evolution/copy/deep-query optimization and authenticated
  automatic maintenance remain deferred.
- Full construction scale/RSS/latency/fault/RustFS/release/visual qualification
  remains deferred. Native Linux-sized cold construction and other platforms are
  not qualified. M13 milestone status is unchanged.
<!-- /evidence-field -->

<!-- evidence-field:next markdown -->
## Next

Narrow core-browsing plan accepted and landed; keep the clean temporary workspace available for separately scoped follow-up.
<!-- /evidence-field -->

<!-- evidence-field:result markdown -->
## Result

pending
<!-- /evidence-field -->


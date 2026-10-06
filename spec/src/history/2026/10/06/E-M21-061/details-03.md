---
schema: 1
id: E-M21-061
---

# E-M21-061 details 3

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
pnpm spec:first; pnpm spec:validate; pnpm check:progress
```

**Notes**

spec-first-verified.log:61 protected files/8 canonical chapters before ledger append; spec-validate.log:5 validators/23 chapters/173 requirements. Initial spec-first mapping omitted JJ/compatibility then CLI/mutations chapters; all updated. Final governance/progress evidence recorded in governance-final.log and progress-final.log after this entry.
<!-- /run -->

<!-- run -->
### Run 2

```yaml
result: pass
failures: 0
hasNote: true
noteEncoding: markdown

```

```shell
pnpm dev:backend:build; JJ_STORE_DEV_ROOT=/Users/stevejuma/code/lapismd/jj-store-stack-workspace-ui/.local/dev-demo JJ_STORE_DEV_PORT=7476 JJ_STORE_DEV_BACKEND_PORT=7478 pnpm dev:backend:restart
```

**Notes**

backend-build-final.log/backend-restart-final.log:rebuilt explicit development executable and restarted only owned backend;Main Vite on7476,backend7478,fixture preserved. No reset,installed CLI change or unrelated service restart.
<!-- /run -->

<!-- run -->
### Run 3

```yaml
result: pass
failures: 0
hasNote: true
noteEncoding: markdown

```

```shell
CUA cold reload existing live tab; inspect Workspace, Commits, Op Log and history-mode menu; jj --ignore-working-copy log/op log
```

**Notes**

live-verification.txt:original7db78 operation rendered three expected columns; exposed/fixed managed-index omission. Sample independently changed through six Unapply operations at~08:13 to cloud-runner only,operation737ef4c0. Left that state intact. Cold rebuilt server loads real commits; focused histories follow737ef4c0 and checked Follow current operation. Native log confirms WIP8c47d335 above merge61c16099 with trunk1b5f0cb8 and cloud-runner9492c959 parents. No agent-triggered apply/unapply/reconcile/cleanup or file edits to sample; seven obsolete heads remain pending explicit guarded cleanup.
<!-- /run -->

<!-- run -->
### Run 4

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
pnpm check:progress
```

**Notes**

Initial evidence classified excluded work and explicit implementation bounds as remaining gaps, incompatible with pass. Moved them to out_of_scope, retained unchanged milestone statuses and reran progress-final.log.
<!-- /run -->

<!-- run -->
### Run 5

```yaml
result: pass
failures: 0
hasNote: true
noteEncoding: markdown

```

```shell
JJ_STORE_LOCAL_BROWSER_CONTRACT_CAPTURE=$PWD/.local/stack-topology-fix/http-contracts-verified.json cargo test --locked -p jj-store-cli --test local_browser filesystem_runtime_registers_and_reads_without_native_storage -- --exact
```

**Notes**

http-verified.log:second successful exact real HTTP run; no admission relaxation.55 response captures revalidated against generated schemas.
<!-- /run -->
<!-- /evidence-field -->

<!-- evidence-field:result markdown -->
## Result

pass
<!-- /evidence-field -->

<!-- evidence-field:next markdown -->
## Next

Use explicit Reapply saved branches preview to inspect any eligible legacy cleanup; ordinary refresh and reads do not repair graphs.
<!-- /evidence-field -->

<!-- evidence-field:verification_scope markdown -->
## Verification scope

Focused native stack binary,managed direct consumers,real local HTTP capture,controller/navigation tests,cold affected Chromium stories,types,source boundaries and canonical/documentation/progress checks. Wider unrelated suites excluded by targeted policy.
<!-- /evidence-field -->

<!-- evidence-field:out_of_scope list -->
## Out of scope

- Real sample legacy cleanup remains explicit and was not performed. Membership
  changes during verification were left intact.
- Topology traversal is bounded at50000 total steps and stops at merges;
  ambiguous shared applied bases retain distinct action ownership. Cleanup walks
  at most256 linear operations and excludes references/descendants.
- Full root suites,external Linux scale
  qualification,RustFS/release/backend-provider expansion,screenshot
  comparisons/baselines and package publication are outside this slice; managed
  external Linux fixture remains ignored. No milestone promotion.
<!-- /evidence-field -->

<!-- evidence-field:remaining list -->
## Remaining

[]
<!-- /evidence-field -->


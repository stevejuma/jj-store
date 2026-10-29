---
schema: 1
id: E-REPOSITORY-DELETION-001
---

# E-REPOSITORY-DELETION-001 details 3

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
pnpm dev:cargo -- test --locked -p jj-store-service --test authorization --test administration; pnpm dev:cargo -- test --locked -p jj-store-server --test local_installation --test configuration
```

**Notes**

18 native authorization/token-administration tests and 25 installation/configuration tests pass. These retain existing policies, revocation, receipt replay, private catalogs, atomic initialization and profile admission.
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
pnpm test:storybook catalog/components/store-application/RepositoryRemoval.stories.svelte catalog/components/store-application/RepositoryDeletionRecovery.stories.svelte catalog/components/store-application/StoreRepositorySettings.stories.svelte catalog/components/store-application/SettingsNavigation.stories.svelte; pnpm catalog:test web/tests/vite/local-repository-removal.test.ts web/tests/vite/native-repository-deletion.test.ts; pnpm catalog:check
```

**Notes**

After the uncertainty guard: 31 fresh-browser interaction/accessibility cases, 6 intent/retry units and catalog type checking (0 errors/warnings) pass. The new restart-required failure is asserted as a toast and preserves its original deletion request.
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
pnpm dev:cargo -- test --locked -p jj-store-object; JJ_STORE_RUSTFS_RUNTIME=/Users/stevejuma/code/lapismd/jj-store/runtime/embedded-rustfs/target/debug/jj-store-rustfs pnpm dev:cargo -- test --locked -p jj-store-server --test repository_deletion -- --ignored --nocapture
```

**Notes**

Final uncertainty/cold-runtime qualification: 5 object units plus 5 contract tests pass. Unknown PUT outcomes block maintenance before any LIST/DELETE; they are not inferred from timeout. Managed-only admission allows a cold owned storage restart to end old handlers before continuing. The isolated real erasure test passes after actually stopping/restarting RustFS between original admission and recovery, while neighbour objects remain readable.
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
pnpm dev:backend:build
```

**Notes**

An overlapping build began before the final uncertainty enum/source update and used a superseded service artifact. Its mixed-source compile was rejected. The final stable-source backend rebuild is required below; no failed build is accepted.
<!-- /run -->

<!-- run -->
### Run 5

```yaml
result: fail
failures: 2
hasNote: true
noteEncoding: markdown

```

```shell
pnpm spec:first
```

**Notes**

The new Changeset and real-storage fixture require explicit public-distribution and compatibility canonical updates. Added those owning paragraphs; a subsequent check required their fresh evidence coverage, now included.
<!-- /run -->

<!-- run -->
### Run 6

```yaml
result: pass
failures: 0
hasNote: true
noteEncoding: markdown

```

```shell
pnpm dev:backend:build
```

**Notes**

Stable-source backend rebuild passed. Darwin linker reported the known large __eh_frame compact-unwind warning for the development binaries; no warning-free build is claimed.
<!-- /run -->

<!-- run -->
### Run 7

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
pnpm contracts:check
```

**Notes**

The native fixture assertion passed, but managed Cargo postflight refused the target at 40.3 GiB above its 40 GiB budget. This was a build-storage gate failure, not a contract assertion failure. The final full contract lane is rerun after eligible maintenance.
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
pnpm dev:clean -- --dry-run; pnpm dev:stop; pnpm dev:clean -- --apply; pnpm dev:backend:build; pnpm dev:ui; pnpm dev:disk
```

**Notes**

Stopped the review service before cleaning eligible idle workspace build outputs. Reclaimed about 18.3 GiB; no fixtures, archives, staged resources or retained targets removed. Rebuilt stable source and cold-started UI/backend on 7460/7462 with fixture preserved and search ready. Post-restart disk report: root target 21.9 GiB, free 66.2 GiB, below limits. Darwin development linker warning remains as described above.
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
CUA fresh hidden-tab review: local and native Settings > General; open confirmation; Cancel; repeat native after cold restart; close temporary tabs
```

**Notes**

Local JJ Sample Repo and native demo/server-only expose red actions with their correct preserved-folder/permanent-object wording. Both confirmations were cancelled; no existing repository was removed. Final cold native General page visibly shows Delete and permanent object-erasure text. Screenshots: .local/repository-removal-confirmation.png and .local/repository-deletion-confirmation.png. Existing user tabs preserved. Real deletion only ran against isolated test storage.
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
pnpm contracts:check
```

**Notes**

Final post-maintenance lane passes: generated OpenAPI/TypeScript match and 300 actual HTTP/SSE responses conform. Contract validator tests and TypeScript fixture typing pass. The earlier disk-budget failure is resolved without bypassing its gate.
<!-- /run -->

<!-- run -->
### Run 11

```yaml
result: pass
failures: 0
hasNote: true
noteEncoding: markdown

```

```shell
pnpm exec prettier --write <changed UI, generated contract and Markdown files>; pnpm exec prettier --check <same changed files>; pnpm check:progress; pnpm spec:validate; pnpm spec:first
```

**Notes**

Changed-file formatting and format check pass. Final post-milestone progress validation passes for 157 tasks and 847 records; seven specification validators pass (4068 chapters, 205 requirements), and spec-first passes 40 protected files/11 canonical chapters. Full root aggregate checks, full Rust workspace, release/platform qualification and visual baselines are out of scope under the targeted policy. An exploratory jj diff --check was unsupported by the installed CLI; required Rust and Prettier formatting checks passed instead.
<!-- /run -->
<!-- /evidence-field -->

<!-- evidence-field:result markdown -->
## Result

pass
<!-- /evidence-field -->

<!-- evidence-field:remaining list -->
## Remaining

[]
<!-- /evidence-field -->

<!-- evidence-field:next markdown -->
## Next

Retain focused repository lifecycle regressions. External/versioned S3, authenticated/shared erasure and visual baseline approval require separate qualification; no publication or push performed.
<!-- /evidence-field -->

<!-- evidence-field:contract_review yaml -->
## Contract review

```yaml
status: changed
reason: JST-OBJ-004 receives only explicit confirmed LocalTrusted Admin
  namespace erasure; ordinary ObjectStore and automatic GC remain unchanged.
  Owning UI/API/security/CLI/object-storage/architecture/mutation chapters
  govern original intent, installation fencing, bounded erasure and local folder
  preservation.

```
<!-- /evidence-field -->

<!-- evidence-field:milestone_review yaml -->
## Milestone review

```yaml
status: updated
reason: "M8a.9 is Done for the verified macOS ARM64 managed LocalTrusted Admin
  profile: real RustFS erasure/restart/fencing, exact-intent recovery,
  native/HTTP authorization, local removal, cold browser
  interaction/accessibility and public contracts pass. Parent M8a remains
  Deferred; M4.18 remains Done with new evidence, M13.4 remains In progress, and
  M4.31 retains its unrelated Design Core release gate.
  External/versioned/shared profiles are excluded from this gate. Visual
  baselines remain unapproved and untouched."

```
<!-- /evidence-field -->


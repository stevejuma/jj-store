---
schema: 1
id: E-INJECTABLE-STORES-001
---

# E-INJECTABLE-STORES-001 details 3

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
pnpm check:boundaries
```

**Notes**

21 workspace packages, with the new core dependency allowlist.
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
node scripts/package-check.mjs
```

**Notes**

22 coordinated Cargo packages including the embedded helper: registry version metadata and contained compilation inputs passed.
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
node --test scripts/spec/progress-regression.test.mjs scripts/spec/navigation-regression.test.mjs
```

**Notes**

10 helper tests passed. The navigation generator also normalized unrelated historical whitespace; that unrelated churn was restored before committing.
<!-- /run -->

<!-- run -->
### Run 4

```yaml
result: fail
failures: 2
hasNote: true
noteEncoding: markdown

```

```shell
pnpm exec prettier --write Cargo.toml crates/jj-store-core/Cargo.toml spec/src/current/contracts/architecture/03-requirements.md spec/src/current/contracts/architecture/25-injectable-collaboration-stores.md spec/src/current/progress/M32/index.md spec/src/current/verification/arch.md spec/src/current/contracts/publication/07-selected-agentstore-reads.md spec/src/current/contracts/mutations/18-m18-shared-capture-service-semantics.md scripts/check-boundaries.mjs .changeset/injectable-collaboration-stores.md
```

**Notes**

Prettier has no TOML parser. The Markdown/JS targets formatted successfully; later formatting selected only changed Markdown/JS files.
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
pnpm dev:cargo -- test -p jj-store-object --locked --message-format=short
```

**Notes**

10 object tests passed, including conditional publication and unknown-before/after-apply contracts. No new real-RustFS or topology qualification is claimed.
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
python3 - <<'PY'
from pathlib import Path
import subprocess
paths=subprocess.check_output(['jj','--no-pager','diff','--name-only'],text=True).splitlines()
files=[p for p in paths if p.endswith('.rs') and Path(p).is_file()]
subprocess.run(['rustfmt','--edition','2024','--config','skip_children=true',*files],check=True)
subprocess.run(['rustfmt','--check','--edition','2024','--config','skip_children=true',*files],check=True)
print('Changed Rust files formatted and checked:',len(files))
PY
```

**Notes**

81 changed Rust files formatted and checked after the verified semantic changes. A preceding check reported formatting differences in adapted fixture signatures; these were corrected.
<!-- /run -->

<!-- run -->
### Run 7

```yaml
result: fail
failures: 30
hasNote: true
noteEncoding: markdown

```

```shell
pnpm check:progress
```

**Notes**

Historical evidence named moved WAL source paths; added explicit removed-files declarations from the parent tree, preserving historical records.
<!-- /run -->

<!-- run -->
### Run 8

```yaml
result: fail
failures: 3
hasNote: true
noteEncoding: markdown

```

```shell
pnpm spec:validate
```

**Notes**

The governance chapter exceeded its 400-line limit and two newly generated evidence detail pages lacked navigation. Kept the new ownership mapping in its existing table row and regenerated scoped navigation.
<!-- /run -->

<!-- run -->
### Run 9

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
node .local/injectable-stores/refresh-navigation.mjs
```

**Notes**

A concurrent read-side JJ snapshot hit a transient Git index lock before unrelated navigation normalization could be restored. No lock was deleted; restored the unrelated paths after the commands completed and kept subsequent JJ/spec snapshot commands sequential.
<!-- /run -->

<!-- run -->
### Run 10

```yaml
result: fail
failures: 414
hasNote: true
noteEncoding: markdown

```

```shell
pnpm check:progress
```

**Notes**

Unrelated global navigation churn was not covered by this slice; restored those paths.
<!-- /run -->

<!-- run -->
### Run 11

```yaml
result: fail
failures: 12
hasNote: true
noteEncoding: markdown

```

```shell
pnpm check:progress
```

**Notes**

Included the newly generated M32 task-history files in the fresh evidence file inventory.
<!-- /run -->

<!-- run -->
### Run 12

```yaml
result: pass
failures: 0
hasNote: true
noteEncoding: markdown

```

```shell
pnpm spec:validate
```

**Notes**

Seven validators and 206 requirements passed after document limits and evidence navigation were corrected.
<!-- /run -->

<!-- run -->
### Run 13

```yaml
result: pass
failures: 0
hasNote: true
noteEncoding: markdown

```

```shell
pnpm check:progress
```

**Notes**

162 tasks and 850 evidence records passed; moved source paths and new task-history paths are explicitly covered.
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
pnpm spec:first
```

**Notes**

122 protected files and seven canonical chapters passed their owning-topic gate.
<!-- /run -->

<!-- run -->
### Run 15

```yaml
result: fail
failures: 2
hasNote: true
noteEncoding: markdown

```

```shell
python3 - <<'PY'
from pathlib import Path
import subprocess
paths=subprocess.check_output(['jj','--no-pager','diff','--name-only'],text=True).splitlines()
files=[p for p in paths if Path(p).is_file() and p.endswith(('.md','.mjs'))]
r=subprocess.run(['pnpm','exec','prettier','--check',*files],capture_output=True,text=True)
Path('.local/injectable-stores/foundation-format-check.log').write_text(r.stdout+r.stderr)
print('Markdown/JavaScript formatting exit:',r.returncode)
raise SystemExit(r.returncode)
PY
```

**Notes**

Only the two generated architecture/progress indexes required formatting; corrected them with Prettier.
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
pnpm dev:cargo -- test -p jj-store-core --locked
```

**Notes**

Final core suite: six passed, including rejecting oversized untrusted control bytes before JSON decoding. Native callers were already bounded to the same existing 16 MiB limit.
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
pnpm dev:cargo -- clippy -p jj-store-core --all-targets --locked --message-format=short -- -D warnings
```

**Notes**

Final public core byte-admission guard and all core targets passed strict lint.
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


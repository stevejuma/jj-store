---
schema: 1
id: E-READ-086
---

# E-READ-086 details 3

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
JJ_STORE_DEV_ROOT=/Users/stevejuma/code/lapismd/jj-store-codex-preview/.local/codex-preview JJ_STORE_DEV_SERVER=/Users/stevejuma/code/lapismd/jj-store-bounded-reads/target/debug/jj-store JJ_STORE_DEV_PORT=7470 JJ_STORE_DEV_BACKEND_PORT=7472 pnpm dev:backend:restart
```

**Notes**

backend-restart.log: restarted only preview backend after build completed; Vite HMR and existing fixture preserved.
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
cua_repl: cold original Linux master/arch load, observed HTTP responses and folder navigation under temporary network latency
```

**Notes**

linux-before-summary.json: 41 root and 23 arch rows all merge-unavailable. linux-after-summary.json: 19/41 root and 5/23 arch rows available; remaining 22/18 are explicit Budget under unchanged 64 visits. Charged work 312154/311343 within the existing one-million cap; metadata bytes about 3.34MiB. Description and Updated visibly render; linux-metadata-rendered.png records the page. linux-folder-skeleton.png captures pending arch/arm64 navigation with retained snapshot. Temporary latency restored; fresh-page console had zero errors.
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
jj rebase -r yolnqxty -o pkkupzrv; jj rebase -r mwovxnyo -o yolnqxty
```

**Notes**

Started temporary work from clean main and advanced the clean preview to the verified production tree for original-page checks. No unrelated workspace or source-repository edits.
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
pnpm check:progress; pnpm exec spec-validator first; pnpm exec spec-validator validate --only governance,summary,verification,book
```

**Notes**

Final: 139 tasks, eight protected files with six canonical chapters, four canonical validators. Task status and all 528 prior evidence entries remain unchanged.
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
pnpm exec prettier spec/src/milestones.md; pnpm exec prettier --write spec/src/jj-integration.md; pnpm exec prettier --check (affected presentation and six canonical chapters)
```

**Notes**

Final formatting passed; only the new ledger entry was replaced from formatter output, preserving the historical prefix and task rows.
<!-- /run -->
<!-- /evidence-field -->

<!-- evidence-field:result markdown -->
## Result

pending
<!-- /evidence-field -->

<!-- evidence-field:remaining list -->
## Remaining

- Older row metadata remains explicitly unavailable when no first-parent change
  is found within the existing 64-visit cap. Original Linux acceptance retains
  22 root and 18 arch Budget rows; no exhaustive history claim or automatic
  unbounded retry.
- Full release/RustFS/scale/RSS, registry-resolved foundational packages and
  visual baselines remain outside this narrow slice. Existing reproduced package
  Clippy diagnostics from E-READ-084 are unchanged and not rerun.
<!-- /evidence-field -->

<!-- evidence-field:next markdown -->
## Next

Commit this verified narrow slice and advance clean main with Jujutsu; preserve the temporary workspace and fixed 7470 preview. Deeper metadata coverage requires a separately scoped bounded-history policy.
<!-- /evidence-field -->


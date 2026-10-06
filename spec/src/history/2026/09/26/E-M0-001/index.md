---
schema: 1
id: E-M0-001
---

# E-M0-001

<!-- evidence-field:id markdown -->
## Id

E-M0-001
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M0
- M0.1
- M0.3
<!-- /evidence-field -->

<!-- evidence-field:date markdown -->
## Date

2026-09-26
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

real-rustfs
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- scripts/rustfs-contract.py
- docs/codec-coverage.md
- docs/upstream-lock.md
- docs/evidence/m0-rustfs-single-node.json
- package.json
- spec-validator.config.mjs
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- scripts/rustfs-contract.py#test_signatures_bind_conditions_and_bytes
- scripts/rustfs-contract.py#test_rejects_more_than_one_winner
- scripts/rustfs-contract.py#test_unknown_is_not_a_precondition_failure
<!-- /evidence-field -->

<!-- evidence-field:runs runs -->
## Validation

<!-- run -->
### Run 1

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
python3 scripts/rustfs-contract.py --self-test
```

**Notes**

Before implementation: script absent.
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
python3 scripts/rustfs-contract.py --self-test
```

**Notes**

3 tests
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
python3 scripts/rustfs-contract.py --run --output .scratch/rustfs-initial.json
```

**Notes**

7 probes; 20 rounds x 16 contenders for create and CAS; owned container and volume removed.
<!-- /run -->

<!-- run -->
### Run 4

```yaml
result: pass
failures: 0
hasNote: false
noteEncoding: markdown

```

```shell
DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm spec:check
```
<!-- /run -->

<!-- run -->
### Run 5

```yaml
result: pass
failures: 0
hasNote: false
noteEncoding: markdown

```

```shell
DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm check
```
<!-- /run -->

<!-- run -->
### Run 6

```yaml
result: pass
failures: 0
hasNote: false
noteEncoding: markdown

```

```shell
DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm test
```
<!-- /run -->

<!-- run -->
### Run 7

```yaml
result: pass
failures: 0
hasNote: false
noteEncoding: markdown

```

```shell
DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm build
```
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

<!-- evidence-field:versions markdown -->
## Versions

jj 0.45.1; RustFS sha256:b7014e0ce2bc703c1316b3ef760e29dfae61fe4a50d1a66fa89638e0f8ea211f; reference SHAs in upstream-lock
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

Profile defined; no production persistence implemented.
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages



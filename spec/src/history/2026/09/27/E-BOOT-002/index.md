---
schema: 1
id: E-BOOT-002
---

# E-BOOT-002

<!-- evidence-field:id markdown -->
## Id

E-BOOT-002
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- Bootstrap
- Bootstrap.1
<!-- /evidence-field -->

<!-- evidence-field:date markdown -->
## Date

2026-09-27
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

rustfmt-governance
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- README.md
- package.json
- rustfmt.toml
- scripts/check-progress.mjs
- spec/src/architecture.md
- spec/src/spec-governance.md
- spec/src/verification.md
- spec-validator.config.mjs
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

[]
<!-- /evidence-field -->

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
rustup component list --installed
```

**Notes**

rustfmt was already installed by the pinned 1.97.1 toolchain; pnpm check already invoked cargo fmt directly.
<!-- /run -->

<!-- run -->
### Run 2

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
pnpm format:rust:check
```

**Notes**

Before change: dedicated Rust formatting check command absent; spec:check and shared format commands did not invoke rustfmt.
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
pnpm format:rust:check
```

**Notes**

All workspace Rust files already conform to tracked edition/style edition 2024 and Unix newline settings; no Rust source rewrites.
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
pnpm format
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
pnpm format:check
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm check
```
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm spec:check
```

**Notes**

9 lanes, including explicit rustfmt; 57 canonical requirements.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm test
```

**Notes**

59 Rust tests, 14 governance tests and 3 harness self-tests passed; external RustFS tests remain opt-in and are unrelated to formatting.
<!-- /run -->

<!-- run -->
### Run 9

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

Rust 1.97.1; rustfmt 1.9.0-stable (8bab26f4f6 2026-07-14)
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

No storage/API changes. New tracked rustfmt.toml; Rust formatter aliases and normal/specification/shared-format integration.
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages



---
schema: 1
id: E-M17-006
---

# E-M17-006 details 2

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
cargo test -p jj-store-cli --test workspace_management capture_discovery_resolves_nested_and_linked_workspaces_without_mutation --locked -- --exact > /tmp/jj-store-agent-capture-m172-binding-before.log 2>&1
```

**Notes**

Before implementation: capture-specific verifier defaults to unavailable and nested/linked session directories cannot be admitted.
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
cargo test -p jj-store-cli --test workspace_management capture_discovery_resolves_nested_and_linked_workspaces_without_mutation --locked -- --exact > /tmp/jj-store-agent-capture-m172-binding.log 2>&1
```

**Notes**

One exact regression passes: nested directories select the canonical native root, another physical JJ workspace shares the repository with a distinct device proof, ordinary dirty-file edits leave the binding unchanged, foreign sockets fail and operation heads remain absent.
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
cargo test -p jj-store-cli --test workspace_management --locked > /tmp/jj-store-agent-capture-m172-binding-all.log 2>&1
```

**Notes**

All six direct discovery/admission tests pass: nearest inner boundary, depth/metadata bounds, physical/connection rebind, published versus reserved registry membership, unrelated physical link admission and current unregister revalidation.
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
cargo fmt --all
```

**Notes**

Rust source/tests formatted.
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
cargo test -p jj-store-server --test capture_admission --test workspace_links --locked > /tmp/jj-store-agent-capture-m172-admission.log 2>&1
```

**Notes**

Initial direct admission and existing link registry qualification: four tests pass.
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
cargo check -p jj-store-service -p jj-store-server -p jj-store-cli --locked > /tmp/jj-store-agent-capture-m172-binding-check.log 2>&1
```

**Notes**

Affected service, server and CLI packages check.
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
cargo clippy --no-deps -p jj-store-service -p jj-store-server -p jj-store-cli --lib --test workspace_management --test capture_admission --test workspace_links --locked -- -D warnings > /tmp/jj-store-agent-capture-m172-binding-clippy.log 2>&1
```

**Notes**

Initial strict affected libraries and direct discovery/link/admission tests pass; untouched HTTP dependency lint baseline excluded.
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
cargo test -p jj-store-cli --test native_lifecycle content_acceptance::http_acceptance::local_management::local_workspace_management_requires_local_admin_and_captures_contracts --locked -- --exact > /tmp/jj-store-agent-capture-m172-binding-http.log 2>&1
```

**Notes**

Existing actual root-only HTTP management regression passes; one test executed.
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
cargo test -p jj-store-cli --test workspace_management server_capture_admits_only_published_registry_bindings_and_revalidates_them --locked -- --exact > /tmp/jj-store-agent-capture-m172-nested-before.log 2>&1
```

**Notes**

Strengthened before state found root-only revalidation misses an inner Git boundary introduced after session admission. Retain and re-resolve the original source directory.
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
cargo test -p jj-store-cli --test workspace_management server_capture_admits_only_published_registry_bindings_and_revalidates_them --locked -- --exact > /tmp/jj-store-agent-capture-m172-nested.log 2>&1
```

**Notes**

Original-directory discovery now rejects the introduced inner repository rather than checking only the old root.
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
cargo test -p jj-store-cli --test workspace_management --locked > /tmp/jj-store-agent-capture-m172-binding-final.log 2>&1
```

**Notes**

All six final tests pass, including original-directory alias redirection to another physical workspace and newly introduced inner Git boundaries.
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
cargo test -p jj-store-server --test capture_admission --test workspace_links --locked > /tmp/jj-store-agent-capture-m172-admission-final.log 2>&1
```

**Notes**

All four final direct server admission/link tests pass.
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
pnpm spec:validate
```

**Notes**

Five validators, 21 chapters and 160 requirements pass.
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

Six protected source/test files and four owning chapters have same-change evidence.
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
pnpm check:progress
```

**Notes**

129 tasks validate, with M17.2 and aggregate M17 still In progress.
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
cargo clippy --no-deps -p jj-store-service -p jj-store-server -p jj-store-cli --lib --test workspace_management --test capture_admission --test workspace_links --locked -- -D warnings > /tmp/jj-store-agent-capture-m172-binding-clippy-final.log 2>&1
```

**Notes**

Final strict affected libraries and direct source-discovery/link/admission tests pass.
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
cargo test -p jj-store-cli --test native_lifecycle content_acceptance::http_acceptance::local_management::local_workspace_management_requires_local_admin_and_captures_contracts --locked -- --exact > /tmp/jj-store-agent-capture-m172-binding-http-final.log 2>&1
```

**Notes**

Final actual root-only HTTP management regression passes after retaining original-directory revalidation; one test executed.
<!-- /run -->

<!-- run -->
### Run 18

```yaml
result: pass
failures: 0
hasNote: true
noteEncoding: markdown

```

```shell
cargo check -p jj-store-service -p jj-store-server -p jj-store-cli --locked > /tmp/jj-store-agent-capture-m172-binding-check-final.log 2>&1
```

**Notes**

Final affected package check passes after directory context and alias validation changes.
<!-- /run -->
<!-- /evidence-field -->


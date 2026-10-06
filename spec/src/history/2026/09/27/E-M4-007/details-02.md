---
schema: 1
id: E-M4-007
---

# E-M4-007 details 2

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
DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-cli --test native_lifecycle content_diff_
```

**Notes**

Before implementation: zero matching cases; not acceptance evidence.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-cli --test native_lifecycle conflict_descriptor_
```

**Notes**

Before implementation: zero matching cases; not acceptance evidence.
<!-- /run -->

<!-- run -->
### Run 3

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo check -p jj-store-service
```

**Notes**

Label mapping returned Option&lt;&amp;str&gt; instead of owned Option&lt;String&gt;; fixed explicit dereference and owned conversion.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo check -p jj-store-service
```
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
cargo fmt --all
```

**Notes**

New fixture module declared before its file was created. Created the file before rerunning.
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
cargo fmt --all
```
<!-- /run -->

<!-- run -->
### Run 7

```yaml
result: fail
failures: 5
hasNote: true
noteEncoding: markdown

```

```shell
DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-cli --test native_lifecycle content_diff_
```

**Notes**

Fixture compile diagnostics: generic collect and ambiguous BStr::as_ref; fixed explicit Vec and byte-slice types.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-cli --test native_lifecycle content_diff_
```

**Notes**

First executable run: 0 passed / 3 failed with Unavailable.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-cli --test native_lifecycle content_diff_native_lines -- --nocapture
```

**Notes**

Pinned experimental copy-history diff helper rejects valid placeholder copy IDs. Replaced with raw native tree deltas plus bounded owned lineage annotations; no synthetic history objects.
<!-- /run -->

<!-- run -->
### Run 10

```yaml
result: fail
failures: 2
hasNote: true
noteEncoding: markdown

```

```shell
DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-jj --test native_backend copy_placeholder_parents_are_boundaries_not_stored_history
```

**Notes**

Initial fixture compile diagnostics: missing helper label argument.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-jj --test native_backend copy_placeholder_parents_are_boundaries_not_stored_history
```

**Notes**

Backend tried to read a placeholder parent as a stored object. Keep placeholder parents in returned histories while stopping traversal at these boundaries.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-jj --test native_backend copy_placeholder_parents_are_boundaries_not_stored_history
```

**Notes**

1 case passes; direct placeholder object lookup still explicitly errors.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-cli --test native_lifecycle conflict_descriptor_
```

**Notes**

Initial 3 descriptor cases pass.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-cli --test native_lifecycle content_diff_
```

**Notes**

Initial 3 diff cases pass after placeholder handling and lineage lookup changes.
<!-- /run -->

<!-- run -->
### Run 15

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-cli --test native_lifecycle content_diff_canceled_copy_terms_
```

**Notes**

Canceled positive tree term incorrectly reappeared as a copy source. Lookup now simplifies signed terms before testing source membership and charges quadratic comparison work.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-cli --test native_lifecycle content_diff_canceled_copy_terms_
```

**Notes**

Exact regression passes: canceled signed term cannot become a source.
<!-- /run -->

<!-- run -->
### Run 17

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-cli --test native_lifecycle content_diff_
```

**Notes**

Expanded cases: 4 passed / 1 failed because expected sorted path list omitted the newly added non-UTF8 fixture. Added its expected path; preserved Binary assertion.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-cli --test native_lifecycle content_diff_
```

**Notes**

All 5 cases pass: native line parity, Unicode/CRLF/no-final-newline, empty/non-UTF8/binary, modes/symlinks/submodules, copy/rename and reverse parity, multiple lineages, canceled terms, limits, membership, corruption and revocation.
<!-- /run -->
<!-- /evidence-field -->


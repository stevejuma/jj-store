---
schema: 1
id: E-PROFILE-AVATAR-002
---

# E-PROFILE-AVATAR-002 details 2

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
Manual reproduction in the development profile picker with the user JPEG
```

**Notes**

The 861015-byte, 2500 × 2500 JPEG failed invalid_directory_input under the previous retained-media dimension admission.
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
cargo test -p jj-store-server --lib directory:: --locked
```

**Notes**

Final shared normalizer and directory authority: 15 tests pass. Includes JPEG aspect ratio/orientation, transparent PNG/GIF/WebP, source byte/dimension/decoded-memory limits, already-bounded byte preservation and receipts. Linker emits an unwind-size warning. /tmp/jj-image-directory-tests-final.log
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
cargo test -p jj-store-server --test directory_api --locked
```

**Notes**

3 pass: normalized binary source admission, stable replay, identity checked before body consumption, independent repository grants, local and authenticated profiles. /tmp/jj-image-api-tests.log
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
cargo test -p jj-store-http --lib contract::tests --locked
```

**Notes**

15 pass; additive image-upload endpoint and strict existing contract inventory. /tmp/jj-image-http-tests.log
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
cargo check -p jj-store-cli --all-targets --locked
```

**Notes**

Final CLI/server direct consumer check passes. /tmp/jj-image-cli-check-final.log
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
pnpm contracts:generate
```

**Notes**

Generated OpenAPI and TypeScript include upload_image and the binary image-uploads route. /tmp/jj-image-contracts-generate.log
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
pnpm contracts:check
```

**Notes**

Generated contracts match; 300 actual responses conform, 8 contract tests and contract TypeScript check pass. /tmp/jj-image-contracts-check.log
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
pnpm dev:backend:build
```

**Notes**

Built explicit development executables after Rust changes. /tmp/jj-image-backend-build.log
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
pnpm dev:backend:restart
```

**Notes**

Replaced backend on port 7462, retaining Vite 7460, paired v3 ACP/JJ resources and .local/dev-demo. No reset. /tmp/jj-image-backend-restart.log
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
Manual upload through the profile and emoji pickers after backend restart
```

**Notes**

The actual user JPEG is accepted and normalizes to 512 × 512 PNG. Initial stale binary/media-admission and development blob CSP errors were corrected. Later deferred selection yields the original 2500 × 2500 local preview with no upload; Cancel discards it. The user-saved :steve: remains intact.
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
node --test scripts/dev-ui.test.mjs
```

**Notes**

7 dev UI tests pass, including blob preview CSP. /tmp/jj-avatar-dev-ui-unit.log
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
pnpm test:dev:ui
```

**Notes**

Cold HMR startup/proxy/asset acceptance passes independently of built-production CSP. /tmp/jj-avatar-dev-ui-cold.log
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
pnpm exec vitest run --config catalog/vitest.config.ts --project catalog-unit web/tests/vite/directory.test.ts
```

**Notes**

10 exact directory client tests pass: selection/refresh/Cancel send zero POSTs; unknown upload and definition outcomes retain the original request and shortcode. Large source admission and small replay scenarios are separately bounded after an earlier full-suite pressure timeout. /tmp/jj-emoji-deferred-unit.log
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
pnpm web:test
```

**Notes**

174 web unit tests pass. /tmp/jj-emoji-deferred-web-unit.log
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
pnpm catalog:test
```

**Notes**

982 catalog unit tests pass; 1 existing skip. /tmp/jj-emoji-deferred-catalog-unit.log
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/StoreProfileSettings.stories.svelte catalog/components/store-application/SettingsNavigation.stories.svelte
```

**Notes**

13 interaction/accessibility stories pass on a fresh process. Avatar actions, identity row, Settings groups/version, shortcode adornments, local image selection and responsive geometry. Initial warnings and dependency-optimizer reload were repaired; no visual baseline approval. /tmp/jj-emoji-deferred-stories.log
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
env PATH="/Users/stevejuma/.local/bin:$PATH" node scripts/profile-acceptance.mjs
```

**Notes**

Cold built-production UI on isolated real RustFS: 1 local/enrolled-user flow passes, including large JPEG avatar and emoji, zero selection POSTs, normalization after Save, privacy, revocation and restart. Temporary service/store cleaned up. /tmp/jj-emoji-deferred-cold.log
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
pnpm check:ui-ownership
```

**Notes**

Ownership validation passes; Community remains an owned source reference, with no checkout or catalog dependency. /tmp/jj-emoji-deferred-ownership.log
<!-- /run -->
<!-- /evidence-field -->


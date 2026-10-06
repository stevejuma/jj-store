---
schema: 1
id: E-M11-005
---

# E-M11-005 details 11

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
cargo test -p jj-store-service --test review_threads --locked review_threads_are_root_pinned_and_details_resolve_selected_objects -- --exact
```

**Notes**

jsh-directory-review-after.log: 1/1: root-pinned review summary includes immutable creation actor.
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
cargo test -p jj-store-service --test review_threads --locked
```

**Notes**

jsh-directory-review-suite1.log: Compilation fails: fixture supplies ReviewObjectId for CommentRevisionId; fixed with same-byte typed conversion.
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
cargo test -p jj-store-service --test review_threads --locked
```

**Notes**

jsh-directory-review-suite2.log: 4/4: selected edited excerpt, 240 Unicode character bound, resumed root stability, hidden/tombstoned text exclusion and existing native detail bounds.
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
pnpm contracts:generate
```

**Notes**

jsh-directory-generate.log: Focused command completed.
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
pnpm contracts:check
```

**Notes**

jsh-directory-contracts1.log: Generated contracts match; 80 actual HTTP/SSE responses validate; six contract tests and TypeScript contract check pass.
<!-- /run -->

<!-- run -->
### Run 6

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
cargo test -p jj-store-cli --test native_lifecycle --locked local_trusted_http_pages_root_pinned_review_threads_and_resolves_detail -- --exact
```

**Notes**

jsh-directory-http1.log: Zero tests selected because --exact requires module qualification; no acceptance claim.
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
JJ_STORE_REVIEW_CONTRACT_CAPTURE=/tmp/jsh-directory-review-responses.json cargo test -p jj-store-cli --test native_lifecycle --locked http_collaboration_review_acceptance::local_trusted_http_pages_root_pinned_review_threads_and_resolves_detail -- --exact
```

**Notes**

jsh-directory-http2.log: 1/1: actual HTTP page/detail expose review presentation; pinned paging, permissions and durable receipt assertions retained.
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
cargo clippy -p jj-store-service -p jj-store-http --all-targets --locked -- -D warnings
```

**Notes**

jsh-directory-clippy1.log: Focused command completed.
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
cargo clippy -p jj-store-cli --test native_lifecycle --locked -- -D warnings
```

**Notes**

jsh-directory-cli-clippy1.log: Focused command completed.
<!-- /run -->

<!-- run -->
### Run 10

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
cargo fmt --all -- --check
```

**Notes**

jsh-directory-rustformat1.log: New HTTP assertion requires rustfmt; cargo fmt --all applied.
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
pnpm exec prettier --write web/src/lib/controller/projects-repository-types.ts web/src/lib/controller/http-repository-directory.ts web/src/lib/controller/projects-repositories.svelte.ts web/src/lib/controller/projects-hub-types.ts web/src/lib/controller/http-work-data-source.ts web/src/lib/controller/production-projects-hub.svelte.ts web/src/lib/ui/store-work/StoreRepositoryActivity.svelte web/src/lib/ui/store-work/repository-directory-presentation.ts web/src/lib/ui/store-work/StoreRepositoryDirectoryRow.svelte web/src/lib/ui/store-work/community-work-directory.css web/src/lib/ui/store-work/StoreProjectsRepositories.svelte web/src/lib/ui/store-work/StoreWorkRepositories.svelte web/src/lib/ui/store-work/StoreProjects.svelte web/src/lib/ui/store-work/StoreWorkReviewRow.svelte web/src/lib/ui/store-work/community-work-reviews.css web/src/lib/ui/store-work/StoreProjectsReviews.svelte catalog/fixtures/collaboration-review.ts catalog/fixtures/projects-hub-fixture.ts catalog/fixtures/projects-hub-layout.ts .storybook/vite.config.ts spec/src/projects.md spec/src/collaboration.md spec/src/api.md
```

**Notes**

jsh-directory-format1.log: changed-path formatting applied.
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
pnpm exec prettier --write web/src/lib/controller/http-repository-directory.ts web/tests/vite/projects-repositories.test.ts catalog/components/store-work/ProjectsDirectory.stories.svelte
```

**Notes**

jsh-directory-format2.log: changed-path formatting applied.
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
pnpm exec prettier --write web/tests/vite/projects-repositories.test.ts web/src/lib/ui/store-work/community-work-directory.css web/src/lib/ui/store-work/StoreWorkRepositories.svelte
```

**Notes**

jsh-directory-format3.log: changed-path formatting applied.
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
pnpm exec prettier --write catalog/components/store-work/ProjectsDirectory.stories.svelte web/src/lib/ui/store-work/community-work-directory.css web/src/lib/ui/store-work/community-work-reviews.css
```

**Notes**

jsh-directory-format4.log: changed-path formatting applied.
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
pnpm exec prettier --write web/tests/real/work-store.spec.ts
```

**Notes**

jsh-directory-format5.log: changed-path formatting applied.
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
pnpm exec prettier --write web/src/lib/ui/store-work/community-work-directory.css web/src/lib/ui/store-work/community-work-reviews.css
```

**Notes**

jsh-directory-format6.log: changed-path formatting applied.
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
pnpm exec prettier --write spec/src/ui.md spec/src/verification.md
```

**Notes**

jsh-directory-format7.log: changed-path formatting applied.
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
pnpm exec prettier --write web/src/lib/ui/store-work/community-work-reviews.css catalog/components/store-work/ProjectsDirectory.stories.svelte
```

**Notes**

jsh-directory-format8.log: changed-path formatting applied.
<!-- /run -->
<!-- /evidence-field -->


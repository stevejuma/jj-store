---
schema: 1
id: E-M11-005
---

# E-M11-005 details 9

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
pnpm build
```

**Notes**

lapis-community jsh-preview-source-build1.log: svelte-package, Vite, tools build and publint pass.
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
pnpm build-storybook
```

**Notes**

lapis-community jsh-preview-source-catalogbuild1.log: Static source Storybook build passes.
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
pnpm test:ui:consumer
```

**Notes**

lapis-community jsh-preview-source-consumer1.log: Six packaged viewport/theme dialog, confirmation, identity and avatar contracts pass.
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
pnpm spec:first
```

**Notes**

lapis-community jsh-preview-source-first1.log: Mapped chapters initially missing; restored unrelated model comment and added registry source-composition note.
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
pnpm spec:first
```

**Notes**

lapis-community jsh-preview-source-first2.log: 4 protected files, 3 canonical chapters.
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
pnpm spec:first
```

**Notes**

lapis-community jsh-preview-source-first3.log: Final verification evidence gate passes.
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
pnpm smoke:local
```

**Notes**

lapis-community jsh-preview-source-smoke1.log: Docker materializer fetch ETIMEDOUT/ENETUNREACH; owned compose lapis-registry-smoke-76586 removed and no containers remain.
<!-- /run -->

<!-- run -->
### Run 8

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
pnpm exec prettier --check scripts/architecture-baseline.json spec/src/architecture.md spec/src/components.md spec/src/registry-pipeline.md spec/src/verification.md src/components/community-chat-composer/CommunityChatComposer.svelte src/components/community-chat-composer/CommunityChatComposerEditContext.stories.svelte src/components/community-chat-composer/CommunityComposerEditContext.svelte
```

**Notes**

lapis-community jsh-preview-source-format3.log: New edit-context story formatting corrected.
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
pnpm exec prettier --write src/components/community-chat-composer/CommunityChatComposerEditContext.stories.svelte
```

**Notes**

lapis-community jsh-preview-source-format4.log: Story formatting applied.
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
pnpm exec prettier --check scripts/architecture-baseline.json spec/src/architecture.md spec/src/components.md spec/src/registry-pipeline.md spec/src/verification.md src/components/community-chat-composer/CommunityChatComposer.svelte src/components/community-chat-composer/CommunityChatComposerEditContext.stories.svelte src/components/community-chat-composer/CommunityComposerEditContext.svelte
```

**Notes**

lapis-community jsh-preview-source-format5.log: Final eight source paths pass formatting.
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
pnpm exec vitest run --project storybook src/components/community-chat-composer/CommunityChatComposerEditContext.stories.svelte src/components/community-chat-composer/CommunityChatComposer.stories.svelte src/components/community-patterns/Composer.stories.svelte
```

**Notes**

lapis-community jsh-preview-source-stories1.log: 14 pass, 1 whitespace-title assertion failure; retained exact regression.
<!-- /run -->

<!-- run -->
### Run 12

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
pnpm exec vitest run --project storybook src/components/community-chat-composer/CommunityChatComposerEditContext.stories.svelte src/components/community-chat-composer/CommunityChatComposer.stories.svelte src/components/community-patterns/Composer.stories.svelte
```

**Notes**

lapis-community jsh-preview-source-stories2.log: 14 pass, 1 whitespace-title assertion failure; retained exact regression.
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
pnpm exec vitest run --project storybook src/components/community-chat-composer/CommunityChatComposerEditContext.stories.svelte src/components/community-chat-composer/CommunityChatComposer.stories.svelte src/components/community-patterns/Composer.stories.svelte
```

**Notes**

lapis-community jsh-preview-source-stories3.log: 15/15 pass after immutable preview and whitespace-title correction; Svelte autofixer reports no issues.
<!-- /run -->

<!-- run -->
### Run 14

```yaml
result: fail
failures: 3
hasNote: true
noteEncoding: markdown

```

```shell
pnpm web:check
```

**Notes**

jsh-directory-web1.log: Three toSorted errors under configured TypeScript lib; use copied array sort.
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
pnpm web:check
```

**Notes**

jsh-directory-web2.log: svelte-check found 0 errors and 0 warnings
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
pnpm web:check
```

**Notes**

jsh-directory-web3.log: svelte-check found 0 errors and 0 warnings
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
pnpm catalog:check
```

**Notes**

jsh-directory-catalog1.log: svelte-check found 0 errors and 0 warnings; catalog boundaries ok (86 modules)
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
pnpm catalog:check
```

**Notes**

jsh-directory-catalog2.log: svelte-check found 0 errors and 0 warnings; catalog boundaries ok (87 modules)
<!-- /run -->
<!-- /evidence-field -->


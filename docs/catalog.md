# Storybook catalog

Run `pnpm storybook` and open http://127.0.0.1:6046/. Override the port with
`pnpm storybook --port <port>` when occupied; never stop an unrelated listener.
`pnpm catalog:test`, `pnpm test:storybook` and `pnpm build-storybook` provide
unit, Chromium interaction/accessibility and static-build acceptance. Production
UI integration stays deferred. All stories remain visual-pending.

## Provenance

Community source revision: `d859f33a484a9238bb11f974f4ead8e9be729ec6`. Source
repository: https://github.com/lapismd/lapis-community. Directly adapted source
is copyright its original contributors and licensed Apache-2.0, as is jj-store.
Design Core primitives are consumed through released
`@lapismd/design-core@^0.2.0`; their source is not copied. Community-local
tarballs are not runtime dependencies.

| Community source                                     | jj-store destination                                     | Adaptation                                       |
| ---------------------------------------------------- | -------------------------------------------------------- | ------------------------------------------------ |
| `src/components/community-chip/CommunityChip.svelte` | `catalog/components/community-chip/CommunityChip.svelte` | Direct copy; catalog-local imports where needed. |
| `src/components/community-chip/props.ts`             | `catalog/components/community-chip/props.ts`             | Direct copy; catalog-local imports where needed. |
| `src/components/community-typography.css`            | `catalog/components/community-typography.css`            | Direct copy; catalog-local imports where needed. |
| `.storybook/portable-canvas.ts`                      | `.storybook/portable-canvas.ts`                          | Direct copy; catalog-local imports where needed. |
| `.storybook/svelte-runtime-diagnostics.ts`           | `.storybook/svelte-runtime-diagnostics.ts`               | Direct copy; catalog-local imports where needed. |
| `.storybook/vitest.setup.ts`                         | `.storybook/vitest.setup.ts`                             | Direct copy; catalog-local imports where needed. |
| `src/components/community-chip/model.ts`             | `catalog/components/community-chip/model.ts`             | Direct copy.                                     |

# Contextual PR detail design comparison

This record supplements E-FORGE-DETAILS-001. Sources were inspected as design
references; their application source is not imported or copied.

| Reference | Observed pattern | Application decision |
| --- | --- | --- |
| jjbutler `a9763eebc79c17c75edc9c903c6cf6ff68003d5a`, `apps/desktop/src/components/PullRequestCard.svelte` and `packages/ui/src/lib/utils/getForgeLogo.ts` | Forge logo beside PR number and status; Markdown body; editing and secondary controls are explicit. | Pair the configured forge icon with the change's PR badge and detail identity. Open rendered title/body first and show the shared composer through Edit details. |
| jjbutler, `apps/desktop/src/components/ChecksPolling.svelte` | Deduplicated progressive check observation with explicit unavailable/fork/merged conditions. | Keep the existing bounded repository polling and capability model. Load the selected PR's checks through the existing read action, discarding late responses after navigation. |
| Changeyard `4b90e180fd9f7f16aeeb9cb24f17b31ad9cb849a`, `packages/vcs/src/components/vcs-pull-request-actions.tsx` | Separate details/check observations, explicit editor state, check refresh and PR menus. | Retain controller ownership and typed requests. Put connection/session controls and inventory in the column menu; keep the detail body focused on the selected PR. |
| Changeyard, `packages/web-ui/src/pull-request.tsx` | Rendered description by default, title and branch metadata, explicit edit/save and check rollup. | Show the title, state, repository and branch relationship before the description; render checks underneath and merge controls after the checks. |
| GitHub check presentation pattern | Aggregate outcome above individually labelled check rows and secondary detail/run controls. | Group revision-bound checks with a summary, counts, shape-and-text status rows, external details, jobs/logs and a capability-owned run menu. This is a visual pattern, not a GitHub-only contract. |

GitHub, GitLab and Forgejo share the same presentation model. Their monochrome
brand paths are bundled locally from Simple Icons 16.0.0 at
`28241d80f8422eaed317ef968ad1e40b6d0d9163` under CC0-1.0. Unknown provider names
fall back to the generic PR icon. No icon or credential lookup occurs at runtime.

Unlike jjbutler's compact card excerpt, the adjacent column keeps the full
rendered description and managed stack links. Editing and preview retain the
controller's draft. Failures use the application toast owner; missing or stale
check data remains explicitly unavailable rather than implying success.

The current owned PR contract does not expose author/reviewer, approvals or
conversation records. This slice does not invent those fields. Live provider,
packaged application and visual approval remain M31 gates; the catalog transport
is in-memory and performs no remote effects.

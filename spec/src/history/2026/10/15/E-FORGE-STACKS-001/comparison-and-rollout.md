# Forge stack comparison and rollout

Reviewed for change `xltkslny` on 2026-10-15. This is the implemented design and
remaining qualification plan. The initial profile is ordinary Git-backed JJ;
native storage submission remains separate.

## Findings and decisions

| Source | Useful pattern | Adopted here |
| --- | --- | --- |
| [jj-gh](https://github.com/mrjones2014/jj-gh/tree/41adfd32bf97344bb63dc4bc2a1560b97380bc99) | Derive PR bases from the JJ graph; preview the intended shape; unstack before retargeting; exact ordered stack equality. | Explicit bookmark segments and ancestors; immutable submit preview; independent push/create/retarget/description steps; optional exact native membership. |
| JJButler `a9763eebc79c17c75edc9c903c6cf6ff68003d5a` | Forge interfaces, optional capabilities, contextual PR cards/creation, per-revision checks monitoring, owned footer. | Owned Rust contracts, capability-gated controls, shared Stack/Workspace PR column, retained editing and selected CI revision. |
| Changeyard `4b90e180fd9f7f16aeeb9cb24f17b31ad9cb849a` | Provider adapters and JJ stack submission coordinated with PR presentation. | CLI and UI use the same orchestration. A regression proves rewritten branches need both push and retarget; these cannot be mutually exclusive effects. |
| [GitHub native stacks](https://docs.github.com/en/pull-requests/reference/stacked-pull-requests-apis-and-webhooks) | Native ordered PR membership and asynchronous prefix merging. | Optional same-repository capability; exact membership, explicit reshaping and durable asynchronous merge receipts. Portable chains remain the common model. |

JJButler paths inspected include `apps/desktop/src/lib/forge/interface/forge.ts`,
`forgeBranch.ts`, `forgeChecksMonitor.ts`, GitHub/GitLab services,
`components/ReviewCreation.svelte` and `shared/prFooter.ts`. Current FSL-1.1-MIT
source is a design reference; none is copied into this Apache-2.0 repository.
Changeyard paths include `src/vcs/jj/stack-submit.ts`,
`src/providers/ChangeProvider.ts` and its three adapters. jj-gh MIT source paths
include `src/gh/stack_create.rs`, `stack_detect.rs`, `real.rs` and
`src/commands/pr/stack.rs`. There is no runtime or checkout dependency on these
projects. UI source/design references are recorded in `docs/ui-provenance.json`.

## User flow and architecture

1. Approve a device-local connection: forge API root, source/fork project,
   upstream target project, push/target remotes and target branch. Supply a
   session token separately. Source and target remote identities are checked.
2. Select Submit on a stack/bookmark. The shared planner derives the required
   ancestor chain, pins the JJ operation, exact commit IDs and remote leases,
   and shows the branches, PR bases and independent effects. Edit proposed
   titles/descriptions; new PRs default to Draft.
3. Push only planned branch heads; create missing PRs, retarget existing PRs
   when allowed, and maintain a bounded marker-owned navigation block. Authored
   text remains intact. Shared ancestors are submitted once. Fork-only parents
   produce explicitly labelled cumulative upstream PRs.
4. Stack and Workspace bookmark cards show PR identity and current CI summary.
   The adjacent PR column owns editing, draft/ready, checks/runs/jobs, bounded
   logs, available rerun/cancel controls and merge confirmation.
5. Status polling observes landed/closed PRs and updates managed links. Local
   code is unchanged until explicit Restack previews and applies the descendant
   rebase, selected push and PR base updates. Restack preserves JJ change IDs and
   working-copy drafts; conflicts stop publication and retain the local result.
6. Portable merges require every recorded predecessor to have merged and the
   selected PR to target trunk. GitHub native stacks may merge a prefix through
   the selected PR. Provider protection and expected SHA remain required.

`jj-forge` owns provider HTTP, presentation-neutral types, deterministic plans
and the external-effect journal. `jj-store-jj` owns upstream JJ and guarded Git
transport. CLI and LocalTrusted HTTP consume these owned interfaces. The UI
controller owns session credentials, polling, drafts and pending request IDs.
No forge runtime enters native model, object or WAL authority.

## Provider differences

| Capability | GitHub | GitLab | Forgejo |
| --- | --- | --- | --- |
| Portable stacked PRs | Branch chain | Branch chain/MRs | Branch chain |
| Native GitHub membership | Explicit opt-in, same repository | Not used | Not used |
| Fork submission | Upstream PR, cumulative fallback | Create at source project with target project ID | Upstream PR, cumulative fallback |
| Draft/ready | GraphQL mutations | Draft title prefix | WIP title prefix; response must confirm |
| CI | Actions runs/jobs plus check-runs and legacy statuses | MR pipelines/jobs | Actions runs/jobs and statuses |
| Rerun | All or failed jobs | Retry failed jobs | Unavailable in inspected v16 API |
| Logs/cancel | Supported | Supported | Advertised only after v16+ version probe |
| Merge | Async merge/prefix receipt | Guarded merge/squash | Guarded merge/squash/rebase |

API references: [GitHub stacks](https://docs.github.com/en/rest/pulls/stacks),
[GitHub PR/async merge](https://docs.github.com/en/rest/pulls/pulls),
[GitLab merge requests](https://docs.gitlab.com/api/merge_requests/),
[GitLab pipelines](https://docs.gitlab.com/api/pipelines/), and
[Forgejo v16 schema](https://code.forgejo.org/forgejo/forgejo/raw/tag/v16.0.0/templates/swagger/v1_json.tmpl).
GitHub queue acceptance is not merge completion. Its async result availability
is bounded by the provider (currently 24 hours); later PR state can still prove
landing. GitLab's newer stack detection/retargeting is helpful observed provider
behavior, not a substitute for explicit local Restack or a portable guarantee.

## Qualification sequence and open work

The implemented slice has focused planner/journal/provider tests, a real local
Git/JJ submit-and-squash-Restack scenario, CLI configuration acceptance,
LocalTrusted HTTP admission, controller/lifecycle tests and cold PR-column
interaction/toast coverage. The evidence index records exact commands/outcomes.

Next, use disposable credentialed projects on each forge to verify API shape,
permissions, forks, drafts, running/failed CI, logs, cancellation/retry and merge
protection. GitHub additionally needs native stack reshape and async prefix
merge through actual queue completion. Test authentication expiry, rate limits,
uncertain network outcomes and a provider result older than its retention window.
Then cold-test the packaged application from connection through Restack across
restart. Platform, native-storage transport and larger-stack qualification are
separate gates. No visual baseline is updated by this slice.

Broader follow-ups are additional forge adapters, optional service-side
maintenance/webhooks, journal archival/migration, shared-root/cross-combination
Restack, OAuth/account integration, richer reviews/comments and provider merge
queues beyond this profile. These are not silently claimed as implemented.
Wider repository test fixes remain deferred per the user's instruction.

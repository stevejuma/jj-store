---
schema: 1
id: E-WORKFLOW-001
---

# Diagnostics, limits and remaining gates

This evidence belongs to enclosing Jujutsu change **xnsuxmul**. The initial
working copy was clean; all tracked changes implement this approved workflow
slice. No release, push, installed custom-CLI replacement, repository migration,
destructive fixture reset or visual baseline update was performed.

The initial implementation adds native WorkflowStore records selected only by
manifest CAS. The optional execution host uses private operator configuration,
source attachments and named command/existing AgentManager admission. Repository
Write alone does not grant execution or secret authority. Command workspaces
export an exact committed revision; conversation-only runs retain immutable
references and use an empty command workspace. The server stores coordination
and evidence without executing workflow code in its publication path.

New source matching qualifies bookmark `*`/`?`, native JJ revsets at the pinned
operation, recorded code-author emails and commit descriptions. Thread messages
use authenticated principal IDs and case-sensitive text conditions. Thread and
message scope plus emoji/Like/Dislike/custom reaction values support added
reactions. Raw message bodies remain in source ThreadStore. Selected thread
edits, tombstones and reaction retractions are excluded. Conditions are trusted
observations submitted only by explicitly admitted hosts. A pinned catalog
snapshot governs each first evaluation; duplicates return that decision even if
another observer has a different filter set or sees a later definition.

## Corrected failures

- UI TypeScript ran before the generated API completed and reported missing
  conditions/trigger variants. Regeneration followed by the exact web check
  passed with zero errors/warnings.
- Story assertions initially used the wrong select/search accessibility roles,
  an exact option name that also had an ID description, and HTML input value
  assertions for CodeMirror. Failed modal tests then left the following story
  blocked. Assertions were corrected to the actual roles and editor text;
  successful stories close their dialogs. Final four-file lane: 24 passed.
- Catalog fixture snapshot spreads produced an invalid union; the final fixture
  narrows its owned native snapshot before assigning WorkStore root. A temporary
  syntax error during that edit was fixed. Final catalog: zero errors/warnings.
- Architecture initially rejected an oversized overview/inspector. Owned
  feedback, home validation, author selection and trigger selector extractions
  keep production components under the existing limit. Architecture passed.
- Package boundary validation initially rejected the new neutral workflow model
  dependency on neutral collaboration identity/reference types. The specific
  edge is now declared and governed in the owning architecture chapter; no UI,
  storage provider or executor runtime dependency was added. All 18 packages
  pass the boundary checker.
- Strict dependency-wide Clippy hits four existing `jj-collab` warnings (large
  enum variants and collapsible conditions), plus earlier shared-WAL warnings.
  They were not weakened or edited. Strict `jj-workflow --no-deps` Clippy passed;
  earlier changed Rust consumer Clippy with warnings allowed passed. macOS linker
  warns about a large `__eh_frame` section; builds/tests complete successfully.
- The first real-storage invocation omitted `--ignored`; no test ran. The
  corrected command explicitly runs the ignored RustFS acceptance and passes.
- Documentation review initially required refreshed source fingerprints; eight
  topics were reviewed against final owning pages/contracts. `uv` was absent
  from the inherited PATH, so documentation Node tests were rerun with its
  existing absolute directory added. Final docs generation/check/test pass.
- Build/doc/contract lanes initially hit the 40 GiB root target cap while the
  development backend held that target live. Managed cleanup skipped it. Paired
  binaries were staged under ignored `.local/dev-runtime/workflow`, then only
  the development backend was restarted. First preflight lacked the paired `jj`
  and stopped before changing the running service; staging the pair resolved it.
  Managed dry-run/apply cleaned the now-idle root and recovered roughly 27 GiB.
  The final backend was rebuilt and restarted with Vite and `.local/dev-demo`
  preserved. Logs: `/tmp/jj-store-workflow-clean-apply.log`,
  `/tmp/jj-workflow-trigger-backend-{build,restart}.log`.

## Qualification boundaries

The full affected WAL package and native lifecycle/configuration lanes were
required by shared manifest/record dispatch and startup admission changes. The
public contract checker consumes generated API documents and real capture
fixtures, so its relevant wider lane is required by the changed shared schemas.
These are distinct from the forbidden default root aggregates, which were not
run (`pnpm check`, `pnpm test`, `pnpm build`, `pnpm spec:check`). UI-only follow-up
work uses exact web/catalog/Storybook/accessibility lanes and no Rust rebuild.

The CUA screenshot documents a fixture interaction, not approved visual parity.
Cold desktop/touch application stories pass Axe and layout/hover acceptance.
The real RustFS two-host lane qualifies commands, source export and durable
reports; it does not substitute for live external agent providers or an actual
Forgejo runner binary. M28 stays In progress. Unchanged Forgejo ConnectRPC
registration/job polling, complete GitHub Actions YAML/expression semantics,
DAGs, cron/webhooks, automatic failover/reassignment and Thread/Reaction origin
reduction remain deferred. The CLI guide owns the current supported profile.

Final generated-contract check passed, including its actual response capture
fixtures, 8 pure contract tests and TypeScript consumers. Initial spec-first
rejected unnamed test references, a freeform result and arbitrary child names
that were not loaded as evidence pages. The record now uses named Rust tests,
`pending` for unqualified provider/visual gates, and standard details pages.
Final spec/progress results are recorded after completing those checks.

Full spec validation also exposed an existing duplicate generated navigation
self-reference in E-APP-RELOAD-001/SUMMARY.nav.md. Its parent already selects
that evidence index; the duplicate generated link was removed without editing
committed evidence content. Broad navigation regeneration initially rewrote
52 unrelated historical/index pages; those changes were restored with Jujutsu,
retaining only workflow navigation and this necessary one-line duplicate fix.
The workflow requirements table now uses the required Requirements heading.

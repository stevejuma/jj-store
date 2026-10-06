---
schema: 1
id: E-UI-076
---

# E-UI-076

<!-- evidence-field:id markdown -->
## Id

E-UI-076
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M9.5
- M11.4
<!-- /evidence-field -->

<!-- evidence-field:date markdown -->
## Date

2026-10-02
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

Community structural Focus sheets for Thread replies and tool discussions
<!-- /evidence-field -->

<!-- evidence-field:revision markdown -->
## Revision

Enclosing Jujutsu change sykxvszs
<!-- /evidence-field -->

<!-- evidence-field:requirements list -->
## Requirements

- JST-UI-002
- JST-UI-005
- JST-UI-018
<!-- /evidence-field -->

<!-- evidence-field:result markdown -->
## Result

pending
<!-- /evidence-field -->

<!-- evidence-field:remaining list -->
## Remaining

- Visual baseline approval remains separate. Root aggregate lanes, Rust
  builds/tests and production built-CSP qualification are out of scope.
<!-- /evidence-field -->

<!-- evidence-field:scope markdown -->
## Scope

Owned UI-only Focus/Split presentation. Inspected Community structural surface-layer composition and released Design Core AppShell.SurfaceLayer. Focus covers the main body, standard right sidebar and independent outer details rail, leaving 7rem of desktop main content visible; mobile uses the root surface with 0.5rem insets. Uses the public controlled open API so mobile reparented content closes cleanly. Escape, scrim and Close use shell dismissal/focus restoration. One application-owned focused tool discussion; multiple Split drafts retain values across layout changes and latest-open ownership. Closing it restores the previous open draft. No backend restart/build, demo reset, fixture mutation, main landing, source dependency or release.
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- catalog/components/store-application/DiscussionSheets.stories.svelte
- catalog/components/store-application/appearance-tool-discussion-play.ts
- catalog/components/store-threads/StoreThreads.stories.svelte
- catalog/fixtures/DiscussionSheetFixture.svelte
- spec/src/api.md
- spec/src/ui.md
- spec/src/milestones.md
- web/src/lib/ui/store-application/StoreApplication.svelte
- web/src/lib/ui/store-collaboration/StoreAgentToolReview.svelte
- web/src/lib/ui/store-discussion/StoreDiscussionSheet.svelte
- web/src/lib/ui/store-discussion/StoreDiscussionWorkspace.svelte
- web/src/lib/ui/store-discussion/StoreFocusedDiscussion.svelte
- web/src/lib/ui/store-discussion/discussion-focus.svelte.ts
- web/src/lib/ui/store-threads/StoreThreadFocus.svelte
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

[]
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

Svelte5.57.1;Vitest4.1.11;Storybook10.5.7;existing released Design Core AppShell.SurfaceLayer.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

Existing native Thread reply and tool-call review target identities, memory-only draft ownership, permissions and Focus/Split preferences retained. No server/API/schema changes.
<!-- /evidence-field -->

<!-- evidence-field:next markdown -->
## Next

Complete adjacent tool-call grouping and remove redundant resting failure label.
<!-- /evidence-field -->

<!-- evidence-field:artifacts markdown -->
## Artifacts

/tmp/jj-focus-sheet-*.log
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages

- [E-UI-076 details 2](details-02.md)

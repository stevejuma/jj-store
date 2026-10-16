# Design references

The latest user correction supersedes the earlier stack-body summary placement.
The maintained Op Log component at parent change mtwuvpky/4decee3e,
web/src/lib/ui/community-projects/CommunityProjectsRepositoryLogDiff.svelte and
repository-log-diff.css, supplies the two-card composition reference: details
first, selected file second, in one column. New application composition preserves
the existing per-origin file and hunk owners.

Community was inspected before foundation selection: the maintained PR creation
dialog and chat composer's additionalActions seam. No Community checkout
dependency or new source copy is introduced.

jjbutler at 66597dfb058ce156ec104e979ff77c802bea6a0a was inspected read-only:
apps/desktop/src/components/CreateReviewBox.svelte, FloatingCommitBox.svelte and
apps/desktop/src/lib/floating/FloatingModal.svelte. Its FSL-1.1-MIT sources were
design references only; no code copied. The reusable editor and return-inline
behavior informed the application adapter.

Changeyard at b0caf0741f17454f2bc48ef3448d741d231f9299 was inspected read-only:
packages/vcs/src/components/vcs-pull-request-actions.tsx,
packages/vcs/src/views/jj-board-view.tsx and packages/web-ui/src/pull-request.tsx.
Contextual PR summaries and viewport-bounded window geometry informed the design;
no code copied. Source revisions and ownership are in docs/ui-provenance.json.

The implementation uses released Design Core WorkspaceShellController, its
imperative view registry, WorkspaceFloatingLayer and Dialog.Portal; existing
shadcn Collapsible, Button and DropdownMenu controls; the maintained application
chat composer and safe Markdown renderer. Drag, resize, minimize, maximize and
dock reuse foundation controls. The application adapter owns reparenting and
session focus/draft placement. No foundation source, package aliases or
node_modules patches were changed.

Visual approval remains pending. No screenshot comparison or baseline update was
performed. This evidence proves scoped interaction, accessibility, types and
lifecycle behavior, not live-forge or packaged qualification.

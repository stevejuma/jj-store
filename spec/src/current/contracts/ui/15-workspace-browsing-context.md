# Workspace browsing context

The body section header follows Community Files: the same FolderRoot icon as the
application's Projects navigation and `Workspace` title at the left and a
compact outlined searchable workspace Command View at the right, after the tabs.
Repository overview clears workspace context; selecting a workspace resolves its
recorded working-copy commit at the displayed operation. Files and default JJ
Log use that commit while explicit bookmarks/revisions remain independently
inspectable. Operation Log, AgentStore and ReviewStore remain repository-wide.
Workspace selection is read-only and never changes physical checkout files.

Repository Agents places New agent session and Manage agents in this Workspace
title row as icon-only buttons with accessible names and shadcn hover/focus
tooltips. The session list does not duplicate these actions. Launch retains the
current repository context; Manage agents uses the existing directory route.

The exact workspace name is encoded as `workspace` in every page hash, with an
explicit `at_op`. Shared working-copy commits do not merge workspace identities.
Missing workspace routes show an unavailable state rather than selecting another
workspace. Explicit Refresh retains workspace context, following its new working
copy when browsing that working copy. The workspace picker discovers names
through authorized pinned reference pages. Section headers, collapsed-sidebar
and mobile layouts exercise production components; changed stories remain
visual-pending.

The picker is disabled while a text-file draft is being edited or a mutation is
previewing, prepared, applying or awaiting an unknown outcome, preserving the
draft and stable request identity until that flow is finished.

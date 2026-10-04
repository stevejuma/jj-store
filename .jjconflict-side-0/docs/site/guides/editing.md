# Editing and conflicts

Editing depends on both the repository profile and the artifact you selected.
Read-only access, a historical selection or an unsupported local capability can
remove an action. The browser's capability checks are more specific than the
existence of a CLI command in the reference.

## Native commands

Native repository actions use typed preparation, preview, apply and status
contracts. Inspect the proposed change before applying it. The prepared intent
binds exact repository state and preconditions; a stale selection fails rather
than silently targeting newer code. A successful committed result refreshes the
active view.

If a response is unknown, retain the pending command and use its status action.
A loading indicator or retry attempt is not evidence that publication failed. Do
not create a second request merely to clear the UI. Field validation remains
beside the editor, while transient operational feedback uses session notices.

## File editing

Use the file's Edit control when available. Keep Source/Preview/Edit selections
within the file body; navigation and code context remain separate. Save submits
the applicable profile's action, while Cancel discards the unsaved editor draft.
Follow dirty-edit prompts when changing context.

In the local Git-backed browser, physical working-file Save is filesystem-only.
It neither snapshots those files into a JJ operation nor publishes companion
metadata. Check the foreground JJ working copy before using its normal snapshot
workflow. This distinction also applies when a saved file looks identical to an
older committed preview.

## Resolve native conflicts

Open the conflict editor for an admitted conflict and inspect all relevant
terms. Base, left/right or multiple terms are identities, not a rule that the
last displayed side wins. Some conflicts are not ordinary textual conflicts;
follow the available typed resolution controls.

A resolution command can include multiple paths atomically. Its preconditions
must still match at apply time. If another publication changes those paths,
reload the conflict and prepare a new intent rather than replaying a stale
replacement under a different request ID. Keep unresolved terms visible until an
accepted resolution actually selects the new state.

Review comments about a conflict are collaboration metadata. Resolving or
editing a comment does not resolve the code conflict, and resolving code does
not erase the review history.

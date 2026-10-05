# Reviews and reactions

Reviews attach discussion to exact retained artifacts. Begin from the artifact
or repository you intend to discuss, and confirm its selected operation,
revision and profile. A current path or abbreviated label is not a portable
review anchor.

## Create and read a review

Use the repository review list/detail in the browser, or `jj review list` and
`show` from a bound workspace. Native access uses the authorized native service;
local Git-backed access uses the companion provider without a server. Reads do
not initialize missing companion identity.

Local repository discussions can initialize metadata on their first write.
Portable local code/agent anchors require separate qualification and are not
silently replaced by repository-level anchors. A disabled control therefore
means a different capability is required, not that you should manufacture an
anchor from display text.

## Edit with exact versions

Replies bind the selected workflow version. Comment edits bind the selected
comment revision. Workflow changes bind the thread snapshot. Read the current
state before submitting a CLI command and keep its original request ID and
complete body through any unknown outcome.

Concurrent replicated comment revisions require explicit resolution. Supply all
observed alternatives through the command's repeated expected-revision
arguments. Missing or stale alternatives fail rather than selecting a hidden
winner. A successful replacement records the versions it supersedes.

`jj review status REQUEST_ID` looks for the durable result after an interrupted
command. `not_known` is not proof of rejection. Native opaque cursors and local
`--after` with `--at-metadata` describe different continuation contracts.

## Threads and reactions

Repository Threads provide human conversations and replies. Their stored
messages remain independent of captured agent activity, although exact supported
associations can connect them. Changing a discussion does not execute an agent
command.

A reaction targets an exact immutable entity/version. Concurrent state and
observed values matter for set, clear and resolution. Custom emoji definitions
and retained raster bytes use versioned metadata; a preview URL is not the
emoji's authoritative identity. Current permission is rechecked for mutations,
and read-only views do not expose writable pickers.

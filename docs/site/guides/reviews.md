# Reviews and reactions

Reviews attach discussion to exact retained artifacts. Begin from the artifact
or repository you intend to discuss, and confirm its selected operation,
revision and profile. A current path or abbreviated label is not a portable
review anchor.

## Create and read a review

Start from repository reviews or exact retained files, including supported file
selections in an agent chat.

### Open repository reviews

Use the repository review list/detail in the browser, or `jj review list` and
`show` from a bound workspace. Native access uses the authorized native service;
local Git-backed access uses the companion provider without a server. Reads do
not initialize missing companion identity.

### Review retained files

Local repository discussions can initialize metadata on their first write. Local
file reviews support exact retained code evidence through the qualified
companion provider. Comments on unchanged workspace files are supported when the
provider can retain their source. Portable agent anchors and the native portable
carrier remain separately gated. Unsupported anchors are not silently replaced
by repository-level anchors or manufactured from display text.

### Send file comments from an agent chat

In an agent chat, file comments join the pending group and are published on
Send. ReviewStore owns the comments; ThreadStore holds repository-qualified
references to their exact revisions. Original-context reads use retained source
evidence rather than the current file. Changed or unavailable selections keep
their drafts; an answer link does not resolve a review automatically.

## Edit with exact versions

Replies bind the selected workflow version. Comment edits bind the selected
comment revision. Workflow changes bind the thread snapshot. Read the current
state before submitting a CLI command and keep its original request ID and
complete body through any unknown outcome.

### Resolve concurrent comment edits

Concurrent replicated comment revisions require explicit resolution. Supply all
observed alternatives through the command's repeated expected-revision
arguments. Missing or stale alternatives fail rather than selecting a hidden
winner. A successful replacement records the versions it supersedes.

### Recover an interrupted review command

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

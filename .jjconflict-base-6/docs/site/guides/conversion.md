# Repository conversion

Conversion creates an independent repository in the other storage profile. Use
it to copy an eligible registered Git-backed JJ repository into server storage,
or to create a new local Git-backed JJ folder from a native repository. The
source remains available. This is separate from folder registration, workspace
attachment and ongoing synchronization.

## Check the source and destination

Open the repository's Settings and find **Repository storage**. The action is
**Use server storage** for a local repository or **Use local Git-backed JJ** for
a native repository. These controls require the installation's conversion
capability and local administrative access. A filesystem-only installation
cannot supply native destination storage.

1. Enter a destination name. For a local destination, enter an absolute path to
   a folder that does not exist.
2. Select a committed operation head. Conversion does not snapshot uncommitted
   working-file edits or reconcile multiple heads for you.
3. Choose **Check conversion**. Preflight pins the source, checks code and
   collaboration history, and reports unsupported representations.
4. Once preflight passes, choose **Convert and open destination**.

The destination receives a new repository identity and a fresh operation log.
Supported committed code ancestry and complete selected collaboration history
are retained through explicit identity mappings and an exact evidence archive.
Existing project links keep their original repository IDs; inspect them before
using the copy as a project's code repository. Device credentials, grants,
settings, hook installation and active execution state are not transferred.

## Interrupted conversion

Closing the dialog, losing a response or restarting does not cancel the original
request. **Continue conversion** reopens its retained intent. **Check status**
inspects that request; **Retry conversion** retains the same request and
destination identity. Do not clear browser or installation recovery state to
work around an unknown result.

After success, **Download mapping report** records source and destination IDs,
translated code references and collaboration selections. **Last conversion
report** retrieves the completed report again. Native publication happens only
after its dependency closure is verified; a local destination never replaces an
existing folder.

## Current limits

The source archive is bounded to 100,000 objects, 16 MiB and dependency
depth 512. Signing, code conflicts, non-placeholder copy IDs and unsupported
portable code/agent anchors fail preflight. Unsupported historical evidence can
block a conversion even when the current page is readable. No source records are
silently omitted to make the copy succeed.

The bounded conversion profile has focused provider, cold browser and real
storage evidence. Broader local full-flow and portable anchored profiles remain
separate qualification gates. See [capabilities](../reference/capabilities.md)
and [local metadata architecture](../technical/local.md).

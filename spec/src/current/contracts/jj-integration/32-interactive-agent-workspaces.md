# Interactive agent execution

Session workspace creation and snapshotting use the native-aware owned JJ
provider with a fixed executable and exact revision arguments. A source folder
is never snapshotted by preparation or a read. Workspace metadata binds its
repository, profile, physical path and original launch identity. Recovering an
existing destination verifies those identities instead of repeating creation.

M24 adds
[reusable agent conversations](../collaboration/37-reusable-agent-conversations.md)
under JST-COLLAB-034 through JST-COLLAB-036. Existing M23 qualification remains
unchanged.

Contextual participation also owns a separate coordination checkout for its
metadata home. Native preparation uses existing LocalTrusted materialization;
local preparation uses an isolated workspace sharing the companion metadata.
Reusing native materializations verifies repository ID and exact socket in the
bounded connection record before accepting the checkout. This tightens physical
binding verification without changing native identities or publication formats.

JST-JJ-003/004 workspace inspection reads committed paths, bytes, parents and
references through the existing typed native/local readers at exact operation
pins. Physical enumeration and ignore-policy reads are bounded and read-only; no
CLI display output defines a source identity. The configured workspace target is
read without creating configuration. Missing default evidence requires an
explicit comparison selection; common ancestors must be unique.

Explicit prompt dispatch records a preparing boundary before snapshotting the
owned agent workspace and records the exact result before the prompt is written
to the harness. Queued prompts remain controller-owned until the prior turn's
completion/snapshot handling finishes. An uncertain snapshot prevents dispatch
and is never silently repeated on restart. Source checkouts stay untouched.

Original local file-review reads walk the exact retained Git commit/tree/blob
through the owned adapter. The capsule must belong to the selected review
inventory; the physical commit tree must match its recorded tree. Every object
hash and kind is verified, metadata paths and symlink traversal are refused, and
regular-file content is capped at 256 KiB. These reads require no working copy,
current operation, derived index, CLI display parsing or mutable Git reference.

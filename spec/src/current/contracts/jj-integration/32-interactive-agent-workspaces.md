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

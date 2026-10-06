# Interactive agent execution

Session workspace creation and snapshotting use the native-aware owned JJ
provider with a fixed executable and exact revision arguments. A source folder
is never snapshotted by preparation or a read. Workspace metadata binds its
repository, profile, physical path and original launch identity. Recovering an
existing destination verifies those identities instead of repeating creation.

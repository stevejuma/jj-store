# Choose a repository context

The first decision is which authority should own the repository. Opening a
folder, registering discovery metadata, attaching a physical workspace and
creating a repository are different operations.

| Context                  | Code authority                          | Collaboration authority                      | Service requirement                                                     |
| ------------------------ | --------------------------------------- | -------------------------------------------- | ----------------------------------------------------------------------- |
| Native repository        | Manifest-selected native JJ objects     | Independent roots in the native manifest     | Local or authenticated native service                                   |
| Git-backed JJ folder     | Existing JJ/Git repository              | Owned `refs/lapis/state/v1` metadata history | CLI can operate without a server; browser uses the local folder service |
| Mixed local installation | Each repository retains its own profile | Each repository retains its own profile      | Local service composes native and registered folders                    |
| Project metadata home    | Home's selected profile                 | Home owns project/task/comment records       | Follows the home, independently of linked code                          |

## Native repositories

Choose native storage when you want the service to own repository objects,
publication, receipts and local indexes. Managed RustFS is part of the local
installation. A workspace is a client working directory attached to that
repository; its files are not a server-side working copy. Several workspaces can
refer to the same native authority.

`jj store init` creates new repository authority. `jj store attach` creates a
workspace for authority that already exists. `jj-store repository register`
verifies existing native authority and makes it discoverable; registration does
not import a filesystem repository.

## Existing folders

Choose the local Git-backed profile when code already lives in a Git-backed JJ
repository. Explicit folder registration gives the local browser permission to
open that folder. It neither uploads the folder into native storage nor creates
a continuously mirrored native repository.

Collaboration metadata uses a separate owned Git ref. Reading an uninitialized
companion does not initialize it; a supported first metadata write does. Normal
Git fetch/push does not transport that ref as collaboration synchronization.

## Access and location are separate

LocalTrusted is an installation access mode using literal loopback HTTP and an
owner-only Unix socket. It is not a claim that every folder on the machine is
registered or readable. Authenticated discovery uses a manual token and checks
permission independently for each repository. A project link is a relationship,
not an access grant.

When following a guide, match all three contexts: installation access,
repository profile, and selected project metadata home. This avoids applying
native sync instructions to a filesystem-only installation or expecting a task
link to grant code access.

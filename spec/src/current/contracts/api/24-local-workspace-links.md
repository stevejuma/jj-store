# Local workspace links

The joined Folder path/Browse focus group and divided bottom action row change
only Settings presentation. JST-UI-016 and existing link authorization, observed
revision, request identity, validation and native picker behavior remain
unchanged.

The optional `local_workspace_links` capability is advertised only by a
LocalTrusted admin deployment with a composed workspace manager. Repository
`GET /local/workspaces` returns a versioned revision and bounded list of
canonical device folders with explicit profile and ready/unavailable state.
`POST /local/workspaces/link` accepts only version, expected revision and an
absolute folder path; it returns the updated list. Both routes reauthorize the
repository and require local administration before consulting device state.
Errors distinguish invalid folder, unsupported profile, stale revision, limits
and unavailable state. The transport delegates to owned service interfaces; CLI
composition owns JJ discovery and the server owns registry persistence. No
request may supply a principal, socket, executable, state directory or shell
command.

The Settings client uses this advertised capability independently of Search. It
binds link requests to the displayed device revision and preserves the path
draft after failures, including stale or unknown outcomes. An explicit folder
refresh rereads saved device state before retry without moving the repository
snapshot. Repository/source changes invalidate pending responses and clear
private folder drafts; a late request cannot populate another repository.

The optional `local_folder_picker` capability additionally requires a composed
OS picker. `POST /local/workspaces/pick` accepts an empty JSON object and
returns `{version: 1, path: string | null}`; null denotes cancellation. It
requires the same LocalTrusted admin admission and repository authorization as
linking. It neither links nor validates a workspace. Failures distinguish busy,
timeout, invalid helper output and unavailable picker. No executable, prompt or
initial path is accepted from a caller. Existing HTTP request deadlines still
apply.

Settings grouping and joined ID Copy controls preserve full wire identities and
existing explicit actions. Tabs and clipboard feedback are local presentation
state and never publish or change the selected repository snapshot.

LocalTrusted admin Sync settings use optional `local_sync_management`.
Repository GET `/local/sync` returns a V1 local-only or configured-worker
report; POST `/local/sync/control` accepts only V1 pause, resume or retry
intent. Both reauthorize before accessing the worker. No upstream URL, token or
environment variable is exposed or editable.

LocalTrusted Admin `local_search_management` adds GET `/local/search`, POST
`/local/search/save` and POST `/local/search/control`. V1 reports contain a
configuration-byte revision, desired/effective enablement, explicit source
scopes/prefixes/index budget and operation state. Save and Apply carry the
observed revision. Control intents are apply, retry and rebuild. Browser
requests cannot select configuration paths or credentials. Accepted lifecycle
operations continue after HTTP disconnect and remain queryable through status.

Sidebar disclosure persistence is presentation state only: versioned boolean
maps for Projects/Repositories sections and individual projects. It cannot
restore authorization, credentials, repository snapshots or capability claims.

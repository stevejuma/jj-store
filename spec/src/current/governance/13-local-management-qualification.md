# Local-management qualification

Storage's positive integrity and restart acceptance uses
`scripts/storage-browser-acceptance.mjs` with a fresh bounded native repository
and an explicitly selected real RustFS executable. It qualifies exact prepared
targets, publication, complete integrity reports and report persistence through
two service/storage restarts. The runner owns its temporary state and processes,
registers repositories while the service is stopped, and cleans up afterward.
The test requires this fixture; a larger demo may legitimately exhaust the
unchanged audit budget and MUST report incomplete rather than healthy. Record
that result separately from successful full verification of the bounded fixture.

The registered local workspace capture fixture is
`content_acceptance::http_acceptance::local_management::local_workspace_management_requires_local_admin_and_captures_contracts`.
It exercises the actual list/link handlers, current repository authorization,
local-only admission, strict request fields, revision conflicts and unchanged
native authority. Generated-contract checks validate its responses alongside the
existing captures. M14 records device-registry and JJ-discovery tests separately
from cold real-RustFS lifecycle and browser acceptance. CLI composition may
inject the owned workspace-verifier interface into the server; this does not
permit a server dependency on CLI or upstream JJ.

The owned development restart helper honors explicitly disabled Search settings:
it checks the repository capability before waiting for an index and reports
disabled rather than timing out. It preserves the saved Search configuration.

The development profile seed uses production JJ publication, local companion
commands and native WorkStore commands. It preserves the original fixture and
sample identities, resumes missing samples and migrates only untouched seeded
links/targets. Native server-only fixture preparation leaves no physical folder
or workspace after publication. Git-backed fixture serving remains deferred; its
real local identity is never advertised as native authority. Installer
acceptance uses an isolated prefix and preserves the installed `jj`.

Agent seed setup, hooks and removal MUST run with the isolated physical fixture
folder as their working directory, as well as an explicit repository argument.
The harness must not temporarily install hook configuration in the source
checkout. Cold acceptance verifies teardown in both locations.

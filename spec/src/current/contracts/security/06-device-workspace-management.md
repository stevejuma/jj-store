# Device workspace management

Local workspace links require LocalTrusted administration of the selected
repository and the existing browser admission checks. Bearer sessions cannot
obtain this capability, even when they have repository admin permission. The
installation owns a bounded, private registry outside checkout content. Reads
only inspect existing files; they do not initialize state, probe executables,
load a working copy or publish an operation.

Linking requires an absolute existing physical workspace root, bounded JJ
metadata, the native backend/profile and repository binding, and the exact
configured installation Unix socket. Git-backed folders remain explicitly
unsupported until M13.4. Resolve linked-workspace repository pointers through JJ
discovery without loading/reconciling the repository. Revalidate the canonical
workspace and metadata locations before each action. A moved, missing or rebound
folder is reported unavailable and cannot authorize setup.

The registry is revision checked, locked and atomically replaced with parent
directory durability. Identical link retries recover the existing link; a stale
revision cannot add another link. Limits apply to paths, bytes and link counts.
Linking never copies checkout files or agent captures. Unlinking and agent
setup/recovery are not exposed by this foundation slice.

An explicit LocalTrusted admin folder-pick request may invoke the composition's
fixed OS folder-dialog helper, independently of the read-only verifier. No
client or repository data selects the executable or script. One dialog per
service, a 25-second deadline and bounded output limit resource use;
cancellation, timeout and request teardown terminate and reap the child before
releasing ownership. The selected absolute path is only a draft; linking still
performs full physical workspace, profile, repository and installation
validation. GETs never open a dialog, and bearer sessions never obtain picker
capability.

Sync browser management requires exact LocalTrusted admin admission and fresh
repository authorization, including for status. Typed commands delegate to the
existing owned worker; no remote endpoint, credential, configuration path or
shell execution is accepted.

Search settings reauthorize LocalTrusted Admin access before reading or changing
configuration. Writes are bounded, revision checked, locked and atomically
replaced outside checkout content. No browser-supplied filesystem path, token or
command is accepted. An unacknowledged save requires status refresh; lifecycle
operations have owned completion independent of the HTTP request lifetime.

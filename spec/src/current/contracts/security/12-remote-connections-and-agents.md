# Remote connections and agent execution (M27)

## Requirements

| ID          | Requirement                                                                                                                                                                                                                                                                                                                                          |
| ----------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| JST-SEC-009 | Saved remote connections MUST remain device-owned hints outside checkouts. Only LocalTrusted Admin may manage the bounded gateway. Each remote request MUST bind a registered HTTPS origin, verified installation identity and memory-only caller credential; redirects, arbitrary proxy targets and forwarded LocalTrusted authority are forbidden. |

Local remains the first-use default. Saved selection and destinations are public
device preferences, never credentials. Explicit links win over root restoration.
Unknown connection IDs and installation identity changes require visible
recovery; they never silently target Local. Browser sign-out destroys protected
connection state. Switching pauses its readers without stopping a remote job.

The gateway creates ordinary authenticated server-to-server requests to a fixed
registered origin. It does not relax browser CSP, CORS, Host/Origin admission or
remote authorization. It accepts only supported public API methods and paths;
remote local-management routes, cookies, redirects and arbitrary headers are
refused. Source and response bodies, connections and stream lifetimes are
bounded. Tokens and ephemeral connection handles never enter persisted metadata
or URLs.

Gateway freshness uses the remote's registered origin for its WebSocket Origin
header, its retained installation fence and the existing first-frame bearer
handshake. The local browser's Origin, cookies and LocalTrusted authority are
never forwarded. Production two-instance acceptance must exercise this admission
path; a permissive transport fixture cannot qualify it.

Remote agents are disabled by default and configured by operators. Execution is
for trusted collaborators, not hostile-user OS isolation. Model/tool
definitions, secrets, installs and machine-wide lifecycle remain
administrator-controlled. Native repositories already present on the remote
server and existing WorkStore homes are the initial profile. Personal
provisioning and Git transfer are deferred.

The separate operator editor requires fresh `manage-agents` authorization for
reads and saves, including before file replacement and response delivery. It
exposes no local-management API. `manage-identities`, repository Admin and
execution permission cannot authorize agent configuration. Sharing is a saved
principal allowlist, with the existing restart activation boundary made visible.
The fixed-origin gateway admits these bounded operator endpoints and existing
identity administration; the remote checks their independent capabilities.

Installation token bindings select the caller. Repository token grants must name
the same principal. Profiles and avatars are presentation, not authorization.
Sessions, permission decisions, requests, receipts and streams belong to that
caller; published conversation history follows the home's repository readers.
Native workspace access is delegated to the caller at current permission,
limited to admitted repositories. A revoked/expired grant cannot dispatch or
publish. Restart and transport failure preserve unknown outcomes and never
replay prompts.

Qualification covers Linux and macOS independently. Source, fixture and browser
passes do not replace cold installed-runtime and real-storage acceptance.

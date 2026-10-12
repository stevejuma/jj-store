# Authenticated agent admission

JST-API-011 extends interactive execution with a separately enabled
authenticated profile. Operator-defined agents expose presentation summaries to
allowlisted principals; private instructions, runtime options and machine paths
stay private. Installation profile and avatar identity is read through the
selected server's authorized directory, independently of repository and
execution permission.

Each remote conversation has an existing native WorkStore home. Native code
attachments require separate current grants, and its home/origin cannot change
under a retained conversation identity. Caller-partitioned journals retain
stable request identities, receipts and recovery; restart does not replay
prompts. Publication uses the existing AgentStore/ReviewStore authority and
current delegated repository Write. The shared immutable object, WAL and
manifest formats remain unchanged. LocalTrusted runtime management and
personal-home behavior retain their existing contracts.

This profile supports trusted collaborators on the configured server. Client
agent advertising, relay routing, WebRTC, personal provisioning and remote
machine administration remain outside M27.

The operator configuration/sharing editor is a separate administration boundary,
not an execution operation. It requires independent `manage-agents` permission
and reports restart-bound activation. Identity enrollment and repository Write
onboarding retain their separate authorities and existing durable receipts.
Native conversation homes and code repositories remain explicit prerequisites;
the editor does not provision homes, upload local dirty files or mirror a
working copy.

E-REMOTE-AGENTS-002 verifies these unchanged publication and recovery contracts
on macOS with real RustFS and a deterministic ACP process. Native code-grant
revocation and installation identity revocation are tested independently.
Published AgentStore history survives restart with the original completed
request and one started event. This evidence does not qualify a real provider.

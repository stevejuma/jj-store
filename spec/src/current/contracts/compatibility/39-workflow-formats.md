# Workflow compatibility

JST-WF-001 adds an optional WorkflowStore root and state version in manifest
format 6. It requires the existing WorkStore home root, retains frozen origin
metadata, and participates in native copy, selected history and recovery.
Manifests without workflows retain their existing serialization. Older binaries
cannot serve a format-6 home; upgrade the paired server and custom CLI together.
There is no automatic format downgrade or WorkflowStore origin reducer.

Workflow HTTP/native RPC documents and the operator journal use version 1.
Execution-host principal admission defaults empty and requires a server restart.
Host journal generation participates in stable request identity. Unknown
attempts retain their source, owner and report identity and cannot be replayed
or taken over automatically. This does not guarantee exactly-once external
effects.

The initial executor profile is named Unix commands plus existing admitted
AgentManager APIs. Current execution qualification is macOS ARM64 and the pinned
embedded RustFS runtime. Local `forgejo-runner exec` is an operator wrapper;
unchanged remote Forgejo Runner registration/polling, full Actions expression
semantics and ConnectRPC are not implemented by this profile. See JST-WF-004,
JST-WF-005 and the M28 ledger for live provider/runner and visual approval gaps.

Version-1 definitions add optional conditions. Empty conditions and absent
conversation fields are omitted so existing canonical records and source event
identities re-encode identically. Conversation sources have optional code pins
and exact typed ThreadStore/ReactionStore references; no code anchor is
invented. Condition results are trusted observations from explicitly admitted
hosts. Source message bodies and reaction content remain in their owning
repositories.

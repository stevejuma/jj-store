# Reusable agent workflows

The staged M33 extraction builds on M32's independently selected collaboration
stores. Lapis AI owns execution and source acquisition, Agent Redaction owns
native privacy, and JJ libraries own publication and conversation coordination.
Design Core owns generic Chat and Diff presentation. Hosts own credentials,
admission, context, application tools and resource lifetime.

The conversation library accepts store, runtime, context, workspace and private
journal capabilities. It has no server, transport, Tauri, search or native
object store dependency. A shared typed client adapts receipts and immutable
records to presentation without becoming publication authority. Message
publication, per-recipient prompt delivery and captured answers remain separate
outcomes. Unknown prompts never replay. Persisted agent attribution and mention
tokens do not authorize a device-local executable or restore a provider session.

Native redaction moves mechanically to `lapis-agent-redaction`. Its hook and
transcript policy IDs, scanner/rule versions, canonical output and omission
rules remain unchanged. The JJ adapter constructs existing bounded domain
payloads; the redaction library imports no JJ models. Device roots, credential
values and pending replacement prefixes remain private. Hosts admit cache
locations before construction. The TypeScript complete-block policy remains
distinct.

Source readers move separately from JJ normalization and durable publication.
Discovery is read-only and grants no repository, activity or process authority.
Source leases and acknowledged progress retain their existing identities.
Automatic discovery for ordinary Git-backed folders requires new qualification;
native discovery qualification cannot substitute for it.

The first Notes milestone supports existing Git-backed JJ vaults,
operation-pinned read-only history/diffs and a recoverable one-agent
conversation. Persistence is explicitly selected per vault. New JJ conversations
use AgentStore and ThreadStore; legacy file conversations retain an explicit
backend. No silent fallback or dual writer is permitted. Companion transport,
repository initialization/conversion and code/history mutation remain separate
work.

The Tauri host reuses physical-vault/window generation admission. Native managed
sessions have one raw event owner; sanitized durable capture precedes display
delivery and acknowledgement. Renderer subscriptions cannot take over raw
session acknowledgement. Disable and closure drain persistence before releasing
native authority. Shared presentation uses public Design Core exports with
host-owned navigation, tools and notifications.

M33's eight stages cover contracts; privacy/acquisition; conversation
extraction; local history/Tauri; Notes integration; addressed agents; explicit
import and opt-in capture; and standalone qualification. Every crate has a short
README; full documentation remains in the manual. Public package release is a
separate operator action.

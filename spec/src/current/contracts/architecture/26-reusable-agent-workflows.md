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

M33 acquisition adapters consume `lapis-ai-capture` for approved roots, bounded
metadata walks and exact Codex header inspection. The JJ Cursor scanner retains
its required `jj-store.json` hint; the source library knows no JJ sidecar.
Existing device/inode and raw-header witness hashing remains in the JJ adapter.
Source profile inventory maps explicitly without changing serialized JJ
profiles. Availability fences cannot prove retirement after an unknown IO
outcome.

The optional `application` capture feature retains the original durable spool,
WAL request and RPC/service adapters, enabled by default for compatibility.
Without defaults, hosts receive read-only sources, sanitizers, source leases,
reader progress and exact acknowledgement types with no WAL, object store, RPC,
service or upstream JJ dependency. Hosts must durably retain sanitized records
and matching progress before acknowledging them. The feature boundary neither
supplies publication authority nor changes existing spool encodings.

The first `jj-store-conversations` slice extracts shared Control V3 stream
framing and bounded per-participant assistant/tool accumulation. A host-injected
sanitizer performs prefix replacement and final frame scanning; the library
holds undecidable prefixes only in memory. JJ's adapter preserves its admitted
native privacy policy and domain payload limits. Hosts retain one raw event
owner and journal sanitized output before publication, display or ACK.

Durable prompt coordination injects admission, context preparation, private
journal and runtime delivery ports. Journal admission either claims a new
explicit turn or returns retained outcomes. Every recipient is prepared before
any dispatch; preparation failure releases exact prior reservations. A durable
prepared fence precedes dispatch. Proven not-sent delivery releases its own
reservation; an uncertain delivery remains unknown and cannot replay. Runtime
enqueue acceptance is distinct from provider delivery and captured answers.
Existing JJ home/workspace/store preparation remains host-owned and retains its
native publication and exact request identities.

The headless TypeScript client lives in `@lapismd/jj-conversations` with public
built ESM/type exports and no UI/runtime dependency. The private application
consumes those exports through its source workspace; publishable manifests keep
registry dependency metadata. Consumer commands build the package before use;
there is no source alias. Mention tokens retain exact installation setup IDs,
not member keys or execution grants. Delivery observations preserve full-width
host sequences; existing UI adapters own any display-only numeric conversion.

Typed client transports receive immutable exact encoded commands under a fixed
host scope. A lost response retains an unknown operation; retries only read
status and never silently submit it again. Restored unknown intents cannot start
execution. Message publication, recipient delivery and captured answers remain
distinct; session-only client state never substitutes for native durable
receipts or host authorization.

The local-history extraction first separates owned query errors, limits,
cancellation, accounting and non-queuing worker admission from native store
execution. Existing native and local browser public paths re-export the same
types and preserve default budgets, diagnostic schemas and worker sharing. This
internal refactor alone does not expose an independent local-history feature or
qualify a Tauri reader; those retain separate delivery gates.

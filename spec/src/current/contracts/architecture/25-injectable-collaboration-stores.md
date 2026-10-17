# Injectable collaboration stores

E-CRATE-READMES-001 explains the current core and focused JJ entry points and
links detailed documentation. JST-ARCH-006 store construction and publication
contracts are unchanged; provider qualification is recorded separately by M32.

JST-ARCH-006 introduces library-owned AgentStore, WorkStore, ReviewStore,
ThreadStore and ReactionStore interfaces beside stock jj-lib. Each store has its
own named factory registration and may be disabled independently. The host
supplies implementation names; existing repository profiles supply defaults.
Unknown names, duplicate registrations and incompatible repository/provider
bindings fail explicitly. Loading never initializes repository identity, actor
provenance or a domain. First explicit writes retain existing initialization
rules.

The core owns typed requests, pinned reads, original-request status lookup,
shared immutable projection and canonical record codecs. It has no upstream JJ,
native object/WAL, application service, transport, search, UI or runner
dependency. Existing domain models retain their ownership and formats. Git
companion persistence consumes this core; native adapters retain manifest/WAL
authority. The JJ integration owns qualified source evidence and may be selected
without native or application features.

Factories receive host-owned repository capabilities, identity admission,
authorization, clock, limits and evidence readers. Backend implementation
selection does not authorize a caller or replace source-closure validation.
Snapshots bind repository and publication-provider identity and exact immutable
selections. Cross-domain commands capture consistent source selections and
publish atomic changes through one provider. Separately pinned external sources
retain their existing qualification. No distributed transaction is inferred.

Git publication keeps refs/lapis/state/v1 and native publication keeps the
manifest CAS, WAL, receipts and recovery. Staging is invisible. Unknown outcomes
retain original identities; replay checks original receipts before mutable
preconditions. Refactoring preserves canonical bytes, native identities and
qualified profile limits. Host implementation configuration adds no repository
marker or portable metadata format.

M32 owns incremental extraction and external-consumer qualification. Each slice
records its actual acceptance and remaining work; partial interface or package
checks cannot complete provider migration. WorkflowStore retains its current
native provider. Dynamic-library loading and publication are separate work.

The standalone Git carrier owns generic ref publication, bounded object access
and readonly selected-root capabilities. JJ evidence and historical stack
retention stay in the JJ integration; its focused companion feature omits native
storage and application modules. Snapshot roots and constructors remain private
across this boundary. The qualified JJ conflict header remains `jj:trees`.

Each factory receives the same admitted host context but retains its independent
registration. Compound Agent/Thread calls explicitly carry the selected Thread
participant. Matching repository/provider strings do not establish transactional
participation; implementations must share a provider-owned runtime capability.
Provider-neutral requests may retain an existing native publication identity and
outcomes retain exact provider receipts for adaptation without identity
synthesis. Git factories reject native publication identities rather than
silently dropping them.

The loader seals a private registry of the actual selected participants after
all factories are bound. Store handles are exposed through immutable accessors;
post-load field replacement cannot retain a stale built-in participant. A
cross-domain command requires the selected source's compatible runtime provider
capability and fresh source admission before staging. Disabled, unbound or
incompatible sources fail explicitly. Ordinary independent commands load no
unrelated current source domain. Retained immutable dependency inventories still
receive their full historical closure audit.

Sharing a publication capability is an implementation contract: participants
must use the same domain representations, captured source selections and atomic
publication semantics. Wrappers may delegate that capability; an unrelated store
cannot claim compatibility from matching names or scope strings. Separately
authorized repository-review references retain their explicit reader capability.

The native library adapter lives in jj-store-native and depends on owned core,
model and WAL interfaces only. Its five named factories use a host-supplied
publication namespace and the coordinator's existing repository binding and
limits. Native Agent and Review enablement remains an explicit host operation;
loading and empty Work, Thread or Reaction reads stage no roots. Unsupported
portable native evidence remains unsupported.

Native capture preparation produces a durable typed delta carrying its admitted
actor, exact request/publication IDs, result bytes and optional accepted Thread
binding. Publishing requires this preparation; retries never silently prepare a
new capture. Adapting existing requests preserves every field. Receipt
adaptation uses the selected original transaction's domain root and version,
even after later writes or compaction. A receipt for another domain is an intent
conflict. Transient receipt-read failures after a committed publication remain
unknown under the original request identity. Host clocks are explicit library
capabilities; existing coordinator entry points retain their current
system-clock defaults.

Native and Git adapters preserve neutral stale, invalid, missing-target and
review-state errors; application consumers retain their existing typed failures.

Application command entry points compose their existing repository profile's
factories. Native service hosts retain current principal/epoch admission and
exact native receipts; LocalTrusted hosts retain folder admission and filesystem
provenance. Typed invalid, target-missing and review-state errors pass through
without becoming generic corruption or transport failures. Existing historical
read and synchronous compatibility APIs remain explicit provider capabilities.

A missing selected native receipt/control record during adaptation is a receipt
availability failure, never a missing command target or proven nonpublication.
After known commit it retains an unknown outcome; status retains the original
request and reports unavailable authority until the selected receipt is
readable.

Independent embedding qualification uses a private Cargo workspace owned by the
JJ integration's test subtree. Its plain Git, focused stock-JJ companion and
optional native profiles exercise the public interfaces without application
services, transports, search or runners. A one-store external implementation
requires no sibling implementation. Complete feature-specific dependency audits
and runtime receipt/source-preservation proofs accompany the SDK guide under
docs; crate-root READMEs remain concise introductions.

E-INJECTABLE-STORES-006 completes M32's source-library embedding qualification
across plain Git, stock JJ, native without JJ and combined profiles. The
application and real cold RustFS evidence remain in E005. This completes no
WorkflowStore, new-platform, topology or package-publication gate.

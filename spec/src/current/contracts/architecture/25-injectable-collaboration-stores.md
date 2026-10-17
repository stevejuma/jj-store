# Injectable collaboration stores

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

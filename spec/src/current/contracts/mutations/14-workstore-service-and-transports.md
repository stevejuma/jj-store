# WorkStore service and transports

WorkStore typed commands derive authenticated actors and repository-bound
publication IDs, validate entity/project versions and retain durable status
semantics. Home writers control task/comment metadata. Shared service requests
accept only version, request ID and the implemented command union.

Git bridge acceptance is a typed application mutation over the same native
session permission and semantic guard as CLI head publication. It binds exact
immutable facts and optional code heads, derives the actor from admitted
transport identity, verifies representation bytes/mappings before acceptance,
and preserves stable request identity through unknown outcomes. Freshness hints
fire only for committed acceptance. Reads and receipt lookup reauthorize current
repository access and never perform Git transport.

Git state reports whether explicit bridge activation is currently qualified for
the selected domains. The CLI can check this immutable snapshot before normal
working-copy snapshotting or endpoint execution; it grants no additional access.

Durable Git attempts bind the admitted principal as executor. Typed acceptance
rejects a new intent or event for another executor before publication. Retained
intent dependencies, destination-bound observation bases and per-ref outcomes
are verified independently of transport. Started, unknown, result-recorded and
applied facts remain distinct. An applied event and its observation are selected
with native bookkeeping; a remote acknowledgement never substitutes for that
local receipt. Recovery reads or repairs data and cannot dispatch external
writes.

New Git push intents prove accepted source-operation ancestry and immutable
bookmark selection through the owned JJ validator before publication. An
otherwise well-formed staged source operation cannot authorize external Git
publication. Receipt replay retains the already accepted source proof.

Code-repository Git acceptance selects its immutable Git root, native
transition, complete inventory and semantic-format-2 origin in the same CAS.
Receiver reduction uses the owned JJ conversion and frozen-view verifier. New
normal origin intents prove source admission from accepted code publications; a
first seed preserves receipt-selected baseline facts without reviving old code
heads. WorkStore publication still freezes its origin ledger.

WorkStore browse reads are an owned read-only service surface. They derive
summary counts and labels from selected native state, share permission and
resource admission with existing WorkStore reads and never publish empty roots.

Local Git-backed repository review commands use the owned review projector and
companion receipt protocol. Local actor identity is device provenance, and
filesystem admission supplies the access boundary. Exact immutable thread or
comment selections and workflow counters retain native stale semantics. Unknown
ref publication returns the original request identity; status/retry checks the
selected receipt before reapplying any entity precondition.

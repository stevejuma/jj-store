# Durable push attempts

Push attempts are immutable native Git records with an authenticated executor,
frozen source operation, explicit bookmark refs, destination fingerprint and
causal observation bases. Prepared representations and the intent MUST be
manifest-selected before the transport-started event is committed. Events retain
causal predecessors and per-ref acknowledged, observed-satisfied, rejected or
unknown outcomes. An applied event names the accepted observation and is
selected with the native ref transaction. All stages use stable request
identities.

`jj store git status` reads durable attempts without reconciling code. Explicit
`jj store git recover <attempt>` checks the original device binding, inspects
refs and repairs satisfied native bookkeeping. It never sends another push. An
interrupted started attempt remains externally unknown until evidence is
recorded; cache deletion and native undo cannot remove its intent or results.
Unsupported selectors, signing/conflict profiles and push options reject before
transport. Dry-run performs bounded planning/conversion with no intent, ref
publication or external write.

The initial existing-bookmark push profile additionally proves that a new
intent's source operation belongs to accepted code-operation ancestry and its
frozen targets equal that operation's native bookmark fields. A staged operation
or arbitrary caller-selected target is insufficient. Retained intent replay and
cold verification use immutable view evidence; later native undo does not erase
external evidence. Generated selectors need their own qualified source proof.

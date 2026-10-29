# jj-collab

Shared models and pure transitions for Threads, human messages, conversation
metadata, reactions and custom emoji. Exact source references connect these
records to agent activity, reviews and WorkStore comments.

Apply a `ThreadCommandV1` with `ThreadStateV1`, or a `ReactionCommandV1` with
`ReactionStateV1`, using a `CollabGraphV1` for immutable objects. The embedding
provider supplies source admission, actor, time and atomic publication.

See [collaboration concepts](../../docs/site/concepts/collaboration.md) and
[architecture](../../docs/site/technical/architecture.md).

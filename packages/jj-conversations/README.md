# @lapismd/jj-conversations

Headless conversation clients, exact agent mention tokens and delivery
projections for JJ applications. This package has no UI, runtime, network or
storage dependency.

Construct `ConversationClient` with a fixed app/vault scope, a typed transport
and the host's lossless command encoder. `send` retains the exact encoded
command and submits it once. After response loss, use `refresh` to read status;
it never resubmits. Restore interrupted work with `restoreUnknown(requestId)`
without persisting private command bodies in the client. Native durable receipts
remain authoritative across restart.

Publication, recipient delivery and captured answers are separate outcomes.
Retiring a receipt requires an explicit host finality decision. Mention IDs are
installation-local setup identities, not member keys or execution grants.
Delivery projections preserve full-width host sequences, including `bigint`.

Use the public ESM/type exports. Full embedding and ownership documentation is
in the
[conversation guide](https://github.com/stevejuma/jj-store/blob/main/docs/site/technical/conversations.md).

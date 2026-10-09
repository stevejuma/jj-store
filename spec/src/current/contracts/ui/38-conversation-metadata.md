# Conversation details and discovery

Successful document-composer submissions and receipt recovery retain opted-in
agent recipient pins through the owned composer's draft-reset hook, matching
Classic. Clearing submitted prose is not a user action to unpin recipients.

JST-UI-012 presents one repository-backed metadata view for live and recorded
agent conversations in Document and Classic layouts. The full-height details
sidebar keeps captured harness identity separate from the conversation title. It
shows exact IDs, captured activity times/counts, coverage, generated summary,
keywords, goals, model/protocol provenance and source freshness. Missing times
remain unavailable; counts describe captured activity, not fabricated totals.

Rename and Use suggested title are conditional publication actions. Refresh
metadata preserves a custom title. Regenerate title explicitly adopts the new
generated title at its original metadata precondition. A changed revision keeps
the result for review; Publish retained result is explicit and preserves later
custom titles. Unknown requests expose their original identity and recovery
controls. Cancel is available before publication, with its own receipt.

Rename/configuration drafts and credential inputs are session-owned, survive
remounts/navigation and never enter browser persistence. Later typing is not
cleared by an earlier successful request. Duplicate submissions are blocked.
Unavailable publication capabilities remain beside disabled controls;
operational errors use the existing notification authority.

Settings → Agents → Conversation metadata has a dedicated provider/model and
credential configuration. It defaults to unconfigured, has no coding harness,
execution-mode or tool controls, and leaves normal agent defaults independent.
Changing it performs no inference and starts no compute. Disabling it does not
remove committed metadata. Defaults saves retain the independent metadata
choice.

Loaded managed/recorded directories filter title, summary, keywords and goals.
Recorded pagination remains explicit while filtering. Native content search adds
metadata to the existing owner Thread hit without duplicating it; normal owner
navigation and repository authorization remain authoritative. Local indexed
content search retains its existing capability refusal.

M24 remains In progress; interaction/Axe results do not approve visual
baselines.

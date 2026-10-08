# Document conversations

JST-UI-001, JST-UI-015, JST-UI-018 and JST-COLLAB-034 through JST-COLLAB-036
extend the catalog prototype into agent and ThreadStore conversations. M24
remains In progress until recorded live acceptance closes these gates. This
contract states required behavior, not proof of completion.

## Delta comparison and acceptance

| Behavior                                   | Delta evidence                                                                                        | Existing prototype                                                | Live acceptance                                                              |
| ------------------------------------------ | ----------------------------------------------------------------------------------------------------- | ----------------------------------------------------------------- | ---------------------------------------------------------------------------- |
| Exact selections and collapsed discussions | Captured thread, screenshots and user observations                                                    | UTF-16 block/revision/range/quote; inline and collapsed cards     | Resolve immutable native/local source bytes and preserve unavailable anchors |
| Pending comments above input               | Supplied pending-queue screenshot                                                                     | Shared inline/sidebar/queue drafts; comments-only simulated sends | Receipt-backed batch publication followed by explicit dispatch               |
| Several comments at one anchor             | Supplied screenshot and retained discussion data                                                      | Grouped cards with explicit additional comments                   | Stable discussion and comment identities, exact edit revisions               |
| Several answers in one assistant turn      | Sanitized Hello World export has one turn with tools and two distinct answers                         | Catalog-owned answer sections                                     | Ordered live segments, exact persistent answer links                         |
| Reply tags                                 | Previously inspected Delta instruction text requires comment-reply blocks with copied quote selectors | Explicit fixture mappings; no live protocol                       | Versioned ID-based protocol in retained sent context                         |
| Double-check navigation and reply arrow    | Supplied references; hover behavior specified by user                                                 | Linked icon button, tooltip and answer indicator                  | Only validated committed destinations become navigable                       |

Delta's quote/context matching is reference behavior; this application uses
exact discussion/comment revisions. Exported CRDT offsets are not native
identities. Native hover/IME findings remain separate from fixture assertions.

## Renderer and draft ownership

One device-local appearance preference, conversationView = document or classic,
selects all live/recorded agent chats, ordinary threads and thread replies.
Missing or invalid values default to document. Existing Focus/Split placement is
independent. Standalone review and task panels keep their specialized renderers.
Both conversation renderers offer range comments, pending batches and reply
links.

A shared controller owns drafts, audience, exact pending submissions and source
mappings across view switches, navigation, sidebar changes and remounts. Drafts
remain session-only; no draft text is stored in browser persistence. Typing
never publishes or dispatches. Enter adds a newline; Cmd/Ctrl+Enter or Send
submits the whole prompt. Escape retains a draft; Remove explicitly discards it.
Same-author editing resumes a pending exact-anchor comment; Add another creates
a distinct comment. Successful acknowledgements clear only matching submitted
snapshots.

Readers preserve the prototype's immutable content, CodeMirror widget lifecycle,
8px tool-flow/16px message spacing, transparent disclosures, code backgrounds,
folds, comments below collapsed headers, navigation previews and fixed footer.
Source/reply navigation opens the necessary disclosures, history pages, folded
lines and read sidebar before focusing the exact destination. Stale sources
retain their quote and explanation rather than relocating by text matching.

Ordered presentation segments retain native message/tool IDs and source offsets.
Streaming updates do not remount editors or overwrite drafts. Historical data
without ordering evidence remains honest; no inferred tool/answer chronology.
Exact range publication requires a committed source mapping. Nontext sources
support whole-section comments; unavailable/unpublished sources explain why a
pending comment cannot yet be submitted.

Existing permissions, cancellation, queued prompts, multiple recipients,
reactions, mentions, Sent-context inspectors, provider-aware runtime choices and
saved-change panels remain available in both renderers. Captured agent identity
is separate from current runtime presence. Switching a provider never silently
changes an existing process's launch configuration.

## Linked answers

Submitted comments remain grouped in the user turn; agent answers appear in the
transcript with the relevant source quote and user comments, followed by the
inline reply arrow and answer. Several answer sections may belong to one turn. A
sent comment without an answer has a noninteractive delivery icon. A validated
answer enables the green double-check button; hover/focus changes its icon and
shows Go to reply. Multiple destinations expose an agent/turn chooser. Links do
not resolve reviews or mutate comment bodies. Reply state belongs to the exact
comment revision, not whatever text is currently selected after an edit.

## Implementation review

E-CHAT-LIVE-005 implements the shared conversation projection for live sessions,
managed participations, recorded captures, ordinary threads and reply bodies.
The Document default and Classic setting use the same session-owned drafts and
receipt recovery. Linked participations keep their own repository feedback
reader, cancellation, Sent inspector, working state and saved-change controls.
Recorded views observe new addressed participations within the same body.

Exact range comments require a selected committed text part. Streaming and
unmapped raw JSON remain readable without invented selectable sources. Original
capture strings remain unchanged when numbered gutters, Markdown decorations or
middle folds hide presentation characters. Navigation retains a pending exact
destination until the mounted reader or reply node is revealed.

The affected Storybook family and focused controllers verify presentation and
state; a controlled ACP fixture separately qualifies cold publication and
restart. Neither proves model compliance or native IME behavior. The execution
and collaboration chapters own delivery, permission and authorization gates; M24
remains In progress and visual baselines remain unapproved.

# Commentable chat body prototype

JST-UI-001 and JST-UI-015 include a standalone Storybook prototype of the agent
chat body. This catalog-only integration does not replace live chat, publish
comments, send agent prompts or expand M24 qualification.

The body presents messages, compact tool groups, captured code/patches and
custom context in one scrollport, using repository-owned presentation and
released foundational APIs. Text surfaces expose a caret and block-local
selection without editing captured content. Typing or pasting starts an anchored
comment and retains the first input exactly once. A visible Comment action and
keyboard shortcut provide equivalent access. With no selection, comments target
a prose paragraph, code/patch line or entire custom section. Selections cannot
cross content blocks in this prototype.

Discussions appear below the anchor and remain highlighted. Typing immediately
queues a nonempty draft, synchronized between its inline editor and a
collapsible avatar/count group above the next-message input. That group shows
shaded source quotes containing compact avatar/comment cards. Enter inserts a
newline; Cmd/Ctrl+Enter from any composer sends the message and all queued
comments as one host-owned batch. Escape closes editing and retains the draft;
Remove explicitly discards it. Comments-only sends are valid. Failed sends
preserve text and comments; successful sends retain immutable historical batch
summaries. Same-author input on the same exact anchor resumes its pending entry,
or appends a new entry to the existing discussion after delivery. An explicit
Add another comment action adds a separate pending entry at the same anchor;
ordinary editing resumes the latest pending entry. All entries at one anchor
share a single bordered container. Reply and add-another actions use named icons
inside the comment row, revealed on hover/focus and available on touch.
Delivered comments show a green Lucide double-check icon with an accessible Sent
label; linked responses use the Replied label and expose View reply. Inline
entries also link to their exact pending editor or delivered message, opening
its batch and focusing it. Sent and replied states remain distinct. Collapsed
files, tools and sections retain their discussions below the nearest visible
collapsed header. Source/reply navigation opens necessary disclosures, read
panels and middle folds before focusing the exact target. Source links are
available on hover, keyboard focus and touch. Unavailable anchors retain their
quote and explanation without retargeting. Returning focus, tool
collapse/reopen, resize and appended content preserve exact anchors, drafts and
submitted local threads. Comment targets retain stable block identity, content
revision and selected context. Changed content must not silently retarget a
comment. Comment state belongs to the host, independently of mounted editors or
disclosure panels.

The main Delta thread “Add configurable screen lock overlay” supplies the
comparison fixture. A sanitized JSON projection retains source ordering, visible
text, tool inputs/results and captured patch hunks. The adapter uses explicit
UTF-8 byte to UTF-16 offset conversion and separate assistant display segments
within a turn. Missing preimages, line numbers or status remain unknown; opening
details never reads files or executes tools. The untouched export remains
ignored. Synthetic comments/failures/streaming are labelled.

Pointer, keyboard, input composition, immutable content, collapse/reopen,
Unicode mapping, focus, teardown, themes and narrow presentation require focused
interaction/accessibility evidence and a cold Storybook run. Human comparison
stays visual-pending; no visual baselines are approved.

The catalog composition uses the owned CommunityAppShell wrapper and public
AppShell regions, with the same joined edges as Threads. Captured reads open in
its full-height right sidebar beside the conversation toolbar and body, using
the same owned chrome and replace-main behavior as thread replies. The shell
owns resizing: pointer drag, Left/Right in 16px steps (32px with Shift), and
Home/End for 280px/720px bounds. Width is retained across captured-file changes,
close/reopen and mobile transitions within the fixture lifetime. The shell
protects 360px for the conversation; constrained desktop layouts replace the
main region, and mobile uses the shell's right panel without a resize handle. A
fixed sidebar heading shows the tool title and actions above its own ScrollArea.
Closing restores the opener. Resizing keeps raw/rich state, comments and active
editors. The fixture title has one full-width bottom divider, aligned with the
read sidebar's header; both reference headers use the same shell height without
additional vertical sidebar padding. Patches and command output remain in
document flow. Multi-file captures give every file a separate disclosure with
retained comments/drafts; the first file starts open. Tool rows align to the
left with consistent spacing. Focused or selected text readers have no
surrounding section outline; the caret and exact-range highlights indicate
selection. Keyboard focus indicators remain on buttons and links. Raw JSON uses
a named icon button at the right of each tool header, revealed on hover or
keyboard focus and kept visible on touch devices. Read panels repeat the icon in
their header for mobile access. The avatar is 24px with an 8px text gap and a
shared first-line alignment. A compact right-edge rail represents conversation
turns, grouping a user prompt with its assistant messages and tool flow.
Hover/focus shows one turn card with the prompt preview, Turn Summary,
author/available agent-model metadata, captured patch-line counts and recorded
time. Summary text is the captured final response's first paragraph; it is not a
newly generated agent summary. The current reference has one exchange, so it has
one marker. Comment-only turns use Comment/Comments previews with the actual
comment text, author, available agent/model/time metadata and comment count.
Mixed turns include their comments alongside the message summary. Pending
comments have a separate marker before Jump to End, with the same preview and
navigation into the shared queue. These markers participate in session-only
bookmarks, Cmd/Ctrl-click and Alt-wheel navigation, which preserve discussion
state. Dense markers have an equivalent large navigation target in the
keyboard-accessible card. Rail markers have no hover outline; keyboard focus
remains visible. A final distinct marker jumps to the document end and shows a
compact Jump to End hint. Cmd/Ctrl+Down works while focus is in the navigation.
Tool headers remain transparent during hover, with no internal padding. All
section and file disclosure chevrons sit immediately after the label with an 8px
gap; controls fit their text rather than stretching the icon to the row edge.
Their small rounded icon region alone gains a muted background on pointer hover
or keyboard focus; the surrounding header stays transparent. The icon region
remains part of the same named disclosure button, with no extra tab stop. Tool
and file titles use the proportional UI font at 0.875rem, distinct from captured
code typography. Expanded tool/file content has no surrounding frame. Code and
patch readers have a muted background inside their own rounded content region.
File heading corners inherit that content radius without clipping hover controls
or comments. Known numbered captures retain their number/tab prefixes in the
immutable document. Prefixes are hidden from the text display and rendered in a
separate CodeMirror gutter using their recorded labels, including gaps. Wrapped
content keeps its own left edge; middle folds preserve the original trailing
line labels, and comment widgets receive no line labels. Atomic display ranges
keep caret navigation outside the hidden prefixes. Exact UTF-16 comment anchors
remain unchanged. Syntax tokens are parsed from file contents without those
prefixes, then mapped back to exact captured UTF-16 spans, split at each line so
prefixes stay neutral. The display model marks this captured format explicitly;
ordinary code and raw JSON do not infer numbering. Added and removed patch lines
have full-line diff backgrounds and retain their prefix/gutter distinction. File
headers stay neutral; triple-prefix content inside a hunk remains a changed
line. Turn-card totals use the same classification as the reader and count
captured changes, without inventing preimages or net totals. Successful tool
titles start at the same text edge as group/file titles, with no leading success
icon or horizontal inset; completion remains described to assistive technology.
Errors and running tools retain visible status icons. Vertical spacing and
offset focus rings keep controls distinguishable. Captures over 32 lines
initially retain 12 leading and 8 trailing lines, with a Show N more lines
control replacing the middle. Revealing is local and retained across
disclosure/body remounts. The complete immutable document and its UTF-16 anchor
offsets remain unchanged. Hidden lines containing an active or submitted
discussion are revealed so comments stay reachable.

Returned XML-like agent context uses the existing prompt-element renderer and
whole-section comment anchors, rather than displaying its tags as prose.
Adjacent rendered blocks retain their existing spacing, including prose followed
by a shaded code block. This spacing remains stable during focus changes and
preserves the first block's avatar alignment. Thought, handoff and file titles
share compact tool typography and transparent hover surfaces. The reference
subtitle is omitted. Hover/focus actions have a subtle background, border and
distinct Comment hover state; message actions reuse the shared reaction picker
and bar. The caller owns reaction models and their toggle callback. Catalog
reactions are memory-only and survive body remounts; this introduces no reaction
publication or backend mutation.

## Implementation approach

The reference has an editable next-message row at the document end, with the
same small avatar and a generous blank draft region. Actions remain fixed at the
bottom of the body. The body uses the public Design Core shadcn ScrollArea;
navigation and append retention bind to its actual viewport. Shell and footer
remain fixed while the document scrolls. These application stories opt into the
catalog's full-viewport shell layout with no outer frame padding or page scroll.
Reading carets are thicker and slightly taller, using the theme's muted grey;
the next-message caret shares that shape and uses the primary accent. The draft
has no placeholder, and its first line centers vertically with the small avatar.
Enter inserts a newline; Send or Cmd/Ctrl+Enter submits through a caller-owned
callback. The catalog simulates submission and explicitly labelled replies
locally; it never dispatches an agent. One shared host-owned comment controller
retains stable author IDs, drafts, delivery batches and reply destinations. A
structured send callback receives message text and exact comment snapshots. The
Hello World comparison fixture adds recorded delivered/replied discussions and a
pending comment alongside the tool-rich Bedtime fixture. Explicit verified
source maps are used where the Delta export lacks resolved offsets; CRDT
fragments are not interpreted as document offsets. Quote and comment Markdown
fill the card from its start edge. Comment action icons share the avatar/text
row height so top and bottom card insets stay balanced. Tool flows use 8px
sibling/header-content gaps and messages/turns use 16px gaps, without
accumulated nested trailing padding. Jump to End focuses this draft when
present. Optional composer/footer slots keep the body independent of host draft
state and live conversation contracts.

The fixed footer provides compact agent, thinking and model menus followed by an
icon-only Send button with an accessible name and shortcut hint. Model and
thinking labels remain visible at ordinary widths; narrow footers retain their
icons, accessible names and menu access without horizontal overflow. Agent/model
menus use an 18rem width and thinking a 14rem width, bounded by the viewport
with edge clearance instead of inheriting their compact trigger widths. All
choices remain readable without horizontal scrolling. The host owns available
choices and selections; the shared footer emits callbacks. The catalog supplies
synthetic choices and retains them across body remounts without resetting
message or comment drafts. Each simulated delivery retains its selected
harness/model, so later selection changes do not rewrite reply identity.
Captured Codex messages and simulated replies use the existing bundled harness
avatars through the owned member-avatar component. These selectors introduce no
live catalog reads, settings writes or agent dispatch.

1. Project the captured Delta JSON into catalog data and convert byte anchors
   explicitly. Map it to an owned display model without changing AgentStore
   persistence or flattening separate assistant segments into one message.
2. Use public MiraCodeEditor handles for immutable selectable text. Maintain
   fixed reading decorations rather than revealing source Markdown on selection.
   Reject document edits and route first input, paste and composition into the
   comment composer. The body host owns drafts/disclosure; the caller owns
   submitted local threads and may supply that state across body remounts.
3. Mount owned Svelte comment views as block widgets, with DOM-keyed cleanup,
   height estimates and resize measurement following Mira's rendering patterns.
   Supply an optional custom-block snippet for arbitrary Svelte content, with
   whole-section anchors and interaction guards for embedded controls.
4. Keep tool inspection callback-driven. Share comment state between the body
   and captured-read panel without fetching current files, dispatching tools or
   editing agent events. Retain exact revision/range/quote anchors; display
   changed-content drafts and submitted threads with an unavailable explanation.
5. Verify this story family, Unicode/patch mapping and a fresh isolated browser
   run. Human Delta comparison and native platform IME remain separate gaps.

Future live integration requires an explicit event-model adapter, durable
comment authority and streaming reconciliation design. This slice introduces
none of those contracts and is reviewable entirely through the catalog.

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

Active threads appear below the anchor. Enter inserts a newline and
Cmd/Ctrl+Enter submits. Escape closes and retains the draft; explicit Cancel
discards it. Returning focus, tool collapse/reopen, resize and appended content
preserve exact anchors, drafts and submitted local threads. Comment targets
retain stable block identity, content revision and selected context. Changed
content must not silently retarget a comment. Comment state belongs to the host,
independently of mounted editors or disclosure panels.

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

The catalog composition uses the public AppShell body regions. Captured read
results open in its right body sidebar, keeping their raw/rich state and inline
comments independently of tool-group disclosure. Closing the pane restores the
opener; narrow layouts use the shell's mobile body panel. Patches and command
output remain in document flow. Multi-file captures give every file a separate
disclosure with retained comments/drafts; the first file starts open. Tool rows
align to the left and reserve clearance for keyboard/selected focus borders. Raw
JSON uses a named icon button at the right of each tool header, revealed on
hover or keyboard focus and kept visible on touch devices. Read panels repeat
the icon in their header for mobile access. The avatar is 24px with an 8px text
gap and a shared first-line alignment. A compact right-edge rail represents
conversation turns, grouping a user prompt with its assistant messages and tool
flow. Hover/focus shows one turn card with the prompt preview, Turn Summary,
author/available agent-model metadata, captured patch-line counts and recorded
time. Summary text is the captured final response's first paragraph; it is not a
newly generated agent summary. The current reference has one exchange, so it has
one marker. Session-only bookmarks, Cmd/Ctrl-click and Alt-wheel navigation
preserve discussion state. Dense markers have an equivalent large navigation
target in the keyboard-accessible card. Rail markers have no hover outline;
keyboard focus remains visible. A final distinct marker jumps to the document
end and shows a compact Jump to End hint. Cmd/Ctrl+Down works while focus is in
the navigation. Tool headers remain transparent during hover, with no internal
padding. All section and file disclosure chevrons sit immediately after the
label with an 8px gap; controls fit their text rather than stretching the icon
to the row edge. Their small rounded icon region alone gains a muted background
on pointer hover or keyboard focus; the surrounding header stays transparent.
The icon region remains part of the same named disclosure button, with no extra
tab stop. Tool and file titles use the proportional UI font at 0.875rem,
distinct from captured code typography. Expanded tool/file content has no
surrounding frame. Code and patch readers have a muted background inside their
own rounded content region. File heading corners inherit that content radius
without clipping hover controls or comments. Known numbered captures retain
their number/tab prefixes in the immutable document. Syntax tokens are parsed
from file contents without those prefixes, then mapped back to exact captured
UTF-16 spans, split at each line so prefixes stay neutral. The display model
marks this captured format explicitly; ordinary code and raw JSON do not infer
numbering. Added and removed patch lines have full-line diff backgrounds and
retain their prefix/gutter distinction. File headers stay neutral; triple-prefix
content inside a hunk remains a changed line. Turn-card totals use the same
classification as the reader and count captured changes, without inventing
preimages or net totals. Successful tool titles start at the same text edge as
group/file titles, with no leading success icon or horizontal inset; completion
remains described to assistive technology. Errors and running tools retain
visible status icons. Vertical spacing and offset focus rings keep controls
distinguishable. Captures over 32 lines initially retain 12 leading and 8
trailing lines, with a Show N more lines control replacing the middle. Revealing
is local and retained across disclosure/body remounts. The complete immutable
document and its UTF-16 anchor offsets remain unchanged. Hidden lines containing
an active or submitted discussion are revealed so comments stay reachable.

Returned XML-like agent context uses the existing prompt-element renderer and
whole-section comment anchors, rather than displaying its tags as prose.
Adjacent rendered blocks reserve clearance for the reader's offset focus
outline, including prose followed by a shaded code block. This spacing remains
stable during focus changes and preserves the first block's avatar alignment.
Thought, handoff and file titles share compact tool typography and transparent
hover surfaces. The reference subtitle is omitted. Hover/focus actions have a
subtle background, border and distinct Comment hover state; message actions
reuse the shared reaction picker and bar. The caller owns reaction models and
their toggle callback. Catalog reactions are memory-only and survive body
remounts; this introduces no reaction publication or backend mutation.

## Implementation approach

The reference has an editable next-message row at the document end, with the
same small avatar and a generous blank draft region. Actions remain fixed at the
bottom of the body. Enter inserts a newline; Send or Cmd/Ctrl+Enter submits
through a caller-owned callback. The catalog only appends a local user message
and clears its draft; it never dispatches an agent. Jump to End focuses this
draft when present. Optional composer/footer slots keep the body independent of
host draft state and live conversation contracts.

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

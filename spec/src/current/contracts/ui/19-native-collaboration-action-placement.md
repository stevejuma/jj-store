# Native collaboration action placement

The mobile main toolbar keeps navigation at the start, project breadcrumbs in
its center and the details-sidebar toggle at its far end. Projects landing tabs
start with Activity, followed by Projects, Tasks, Reviews, Agents and
Repositories. Only recorded Git/JJ code identities remain inline on agent
session rows and headers. Session, root, snapshot, author and comment identities
belong in a keyboard-accessible shadcn kebab menu that copies complete native
values without changing selection. Agent session transcript/tool counts and
duration align at the right of the bottom row, with duration last. Tool hover
actions stay fully visible and clickable without a per-call scroll viewport,
including where the toolbar is taller than a compact row. They include the same
menu for call, snapshot, turn, sanitized input/output and recorded paths. Open
portaled menus keep their hover toolbar visible; touch exposes the actions
without hover.

Native review and task bubbles use Community hover/focus Reply and message
menus. Native record menus group readable copy values in one keyboard-accessible
Copy submenu, with decorative icons matching message, comment, revision, author,
thread, agent, snapshot, tool and task values. Items copy the complete native
value and keep clipboard feedback and selection unchanged. Edit is offered for
visible comments created by the authenticated repository principal when write
access exists. The composer shows the original text above an edit and the parent
author/content above a reply, supports cancellation and retains draft/context
after a failed or unknown publication. Captured agent messages are immutable
evidence and offer copy actions rather than comment-edit callbacks. A composer
that closes after a confirmed send stops its asynchronous UI work before reading
disposed bindings. Tool comments, nested replies and session comments retain
their exact pending request and draft until receipt status confirms publication;
each inline composer offers that status check.

Task detail uses a fixed title/header row and a fixed bottom comment composer.
Only the intervening metadata, description, subtasks and Activity/comments
scroll within the available page height. The same layout works on narrow and
read-only pages; read-only detail has no composer. Collapsing Activity,
scrolling and background refresh retain the draft and reply/edit context. The
composer remains outside the content scroller and uses the current theme surface
tokens. Returning to a task restores its content scroll position through the
existing bounded in-session navigation cache.

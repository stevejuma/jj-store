# Rich history query delivery

JJ Log and Op Log start with search hidden. A circular Search button in each
column header toggles its search area, exposes pressed/expanded state and uses
an accent icon over a darker shade of the header grey while pressed. Hiding
preserves the draft and accepted query; switching pages starts with search
hidden. The full-width oval input uses the default Design Core shape with no
leading icon, and separate circular icon-only Search and query-context actions
inline at the end. Query context is initially collapsed. Its workspace/email
controls use the shared rounded Settings Field group; workspace selection uses
the searchable Command View with workspace icons. Syntax and execution guidance
use informational alerts with icons vertically centered beside their text.
Search inputs use the standard Design Core input border and focus-ring palette,
with a grey theme border at rest and a transparent border while the focus ring
is visible. Invalid-query feedback retains its destructive theme treatment.

JJ Log and Op Log MUST use Design Core `SearchFilterInput` in custom-language
mode, with jj-store-owned revset and operation grammars, static-class
highlighting and catalog-driven keyboard completion. Query execution is
debounced and uses the displayed operation, explicit workspace, session-only
author email and frozen time context. Only successful text/time is stored in the
URL; page switches clear incompatible syntax. Invalid drafts retain the last
successful graph and its cursor, superseded requests are canceled, and
navigation, disconnection and disposal suppress waiting submissions and late
results. Clearing restores the normal graph scope; queries and pagination must
never fabricate edges across filtered-out rows. Cold browser acceptance MUST
retain strict CSP. Visual baselines remain pending.

Automatic current-head refresh MUST retain the unapplied history draft and its
diagnostic while reloading the accepted query with its frozen time. Advancing
the displayed operation cancels superseded query requests; a changed accepted
query or repository, page or workspace selection resets the editor context. The
route type and shared reader/query construction remain owned controller modules
behind the existing public contracts.

The macOS desktop host can explicitly request an integrated native header. This
presentation reserves fixed pixel clearance for the native traffic lights and
marks header background as a drag region, excluding interactive controls. It
retains a header in narrow layouts. Ordinary browser and catalog presentation
keep their existing spacing unless the desktop profile is explicitly selected.
The presentation marker grants no repository access; the host separately limits
native window permissions to its managed main window and exact origin.

The shared application keeps its typed composition props in an owned leaf
module. Desktop presentation is an optional prop; both native marker defaults
and explicit catalog selections use the same production shell.

Catalog fixtures for unavailable reviews explicitly omit the optional review
reader. Repository profile alone does not imply unsupported reads; qualified
local repository readers retain their existing review capability.

History query editors MUST preserve multiline whitespace and use visible
semantic syntax colors. Enter inserts a line or accepts an active completion;
Ctrl/Command+Enter and Search execute. Default search focus retains the caret
and visible oval outline; invalid drafts tint the input with spaced diagnostics
and keep prior results. The session-only author email initially uses query
metadata's optional default; manual edits, including empty values, MUST win
across metadata/head refreshes. Email MUST remain absent from URLs and persisted
UI state.

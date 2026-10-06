# Repository terminal

Desktop terminal sessions are governed by JST-DESK-006. Desktop MUST own
bounded, repository-bound interactive terminal sessions independently of the
managed repository service. Only the exact managed main window and origin may
use the typed terminal bridge. Targets come from revalidated ready local
registrations or native workspace links, never arbitrary renderer paths,
commands or environment. Missing links require explicit linking; opening a
terminal must not clone, create a workspace, or publish metadata.

Sessions use a filtered login-shell environment, a PTY and a native Alacritty
grid. Input, dimensions, session count, scrollback, parser work and frame
delivery are bounded. A subscription allows one in-flight frame and one
replacement snapshot; stale acknowledgements cannot release a newer
subscription. Terminal output cannot access the clipboard. Native sessions and
output remain in memory; repeated open request identities resolve the same
session. Hide/navigation and renderer reload preserve sessions, tab close ends
its shell, and Quit cleans up sessions before the managed service. Restart does
not recreate shells. Late callbacks from closing or removed sessions MUST NOT
publish obsolete error notices; failures for active sessions remain visible.

The desktop header MUST place a terminal toggle beside the right-sidebar toggle,
using a bottom-panel icon whose filled region and accessible expanded state
track the dock. No terminal footer bar is shown. The terminal toolbar uses a
muted surface, rounded tabs and add control, with maximize and hide controls at
the right. Each tab has an inline leading close button that remains separately
operable from selection. Tab selection retains focus for arrow-key navigation.
Tabs and toolbar buttons MUST have visible hover and keyboard focus states in
light and dark themes. The resize separator MUST be a single visible pixel while
retaining pointer and keyboard resizing. The folder path is available through
the tab tooltip, without a duplicate row above shell output. Maximizing MUST
close the far-right details sidebar; restoring the dock does not reopen that
sidebar. The Svelte dock provides repository/folder-bound tabs, keyboard and
pointer resizing, maximize/restore, Cmd+J, IME, selection/copy, bracketed paste
and readable terminal text. Multiple eligible folders require a choice; one is
selected automatically. Only versioned dock geometry is persisted. Existing
refresh retains historical selections, editor drafts and pending mutations. Cold
native WKWebView and production-CSP evidence remains distinct from fixture
stories.

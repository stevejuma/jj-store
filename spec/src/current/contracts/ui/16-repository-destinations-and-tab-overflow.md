# Repository destinations and tab overflow

The left sidebar retains repository discovery and search. Branches, JJ Log, Op
Log, Agents, Reviews and Projects move into a Details menu below the right-rail
page heading and create action, using Community repository-rail icon rows and
current-page semantics. Settings replaces that rail as before. Selecting a view
on mobile returns to the main content stage.

The Workspace section header has no top or bottom margin and retains its
horizontal inset.

Body tabs use the labels Overview, Branches, Commits, Op Log, Agents, Reviews
and Projects in that order. Overview is the existing Files destination; its
native file browser and legacy `files` hash remain intact. Width measurement
follows the repository-document tabs: keep a visible leading prefix, move
trailing tabs to More, preserve the selected hidden destination, and restore
tabs when width returns. Pill colors, shape and typography stay unchanged;
keyboard selection and dropdown focus remain available.

# Repository destinations and tab overflow

The left sidebar retains repository discovery and search. Branches, JJ Log, Op
Log, Agents, Reviews and Projects move into a Details menu below the right-rail
page heading and create action, using Community repository-rail icon rows and
current-page semantics. Global Settings hides repository controls. Repository
Settings opens from a trailing icon-only body-header gear and hides the rail
while retaining its open/collapsed state for return. Category and actual subtab
rows share accessible overflow; repeated headings and description-only rows are
omitted. Selecting a view on mobile returns to the main content stage.

The Workspace section header has no top or bottom margin and retains its
horizontal inset.

Body tabs use the labels Overview, Branches, Commits, Op Log, Agents, Reviews
and Projects in that order. Overview is the existing Files destination; its
native file browser and legacy `files` hash remain intact. Width measurement
follows the repository-document tabs: keep a visible leading prefix, move
trailing tabs to More, preserve the selected hidden destination, and restore
tabs when width returns. When no tab fits fully, More includes every section,
including the first; no clipped first tab is stranded outside its menu. Pill
colors, shape and typography stay unchanged; keyboard selection and dropdown
focus remain available.

Application navigation reuses the same owned responsive tab strip in Projects,
selected-project views, the legacy work surface, Settings and conversation
Files/Changes. Each preserves its existing enclosing tab panel context, disabled
sections and selection callback. Fitting measures the space remaining after
leading/trailing controls, refits after search closes and restores the leading
prefix when width returns. The selected overflow entry remains marked in More.
Natural widths are measured in an inert hidden surface without replacing active
controls. Fitting does not subscribe to its own output state, repeatedly reset
visible tabs or remount an open overflow menu while the sidebar is resized.
Repository document previews retain their existing responsive line tabs and
icons; fixed avatar/profile mode switches are distinct from navigation pills.

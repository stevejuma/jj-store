# macOS integrated header

The main window uses `TitleBarStyle::Overlay` and a hidden native title while
retaining native close, minimize and fullscreen controls. Traffic lights use the
Buzz reference position of 16,25 logical pixels. The application header reserves
80 CSS pixels at its leading edge and a fixed 40 CSS pixel height. Its 28px
controls and centered header content use Buzz's 3px macOS offset, aligning their
centers with the native controls. The inset MUST configure both the native
window and webview so geometry restoration and resizing retain it. Native
controls and header geometry do not scale with application text. Narrow layouts
retain this header.

The host supplies a presentation-only macOS marker before application startup.
Ordinary browser clients do not reserve native-control space or enable window
drag regions. The managed main window's exact loopback origin receives only
window dragging, Tauri's internal double-click zoom and the single boolean
`set_glass_background` presentation command. No wildcard origin, generic
filesystem, shell, dialog or core permission is granted. The repository terminal
extension adds only its named desktop commands. Window controls and links inside
the header retain their ordinary click behavior. This slice uses Tauri's
standard double-click zoom; matching the macOS custom double-click preference
remains a separate refinement.

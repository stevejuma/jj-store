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
drag regions. Under JST-DESK-004, `pnpm dev:desktop:ui` MAY select macOS header
geometry in the existing fixture-backed Vite application for browser debugging.
The development-only selection MUST NOT grant native capabilities or change
production browser defaults. An owned host interface separates presentation from
optional terminal, glass, window-dragging and sleep-prevention capabilities.
Host resolution precedes appearance and mounting; components and service owners
receive the same host explicitly. Native adapter loading requires supported
versioned metadata and available Tauri IPC. Metadata remains a hint, never
authorization. Browser desktop mode keeps native controls unavailable, uses
opaque glass fallback and introduces no HTML traffic lights. Backend-only
restarts preserve presentation; ordinary `pnpm dev:ui` restores browser layout.

The managed main window's exact loopback origin receives only window dragging,
Tauri's internal double-click zoom and the single boolean `set_glass_background`
presentation command. No wildcard origin, generic filesystem, shell, dialog or
core permission is granted. The repository terminal extension adds only its
named desktop commands. Window controls and links inside the header retain their
ordinary click behavior. This slice uses Tauri's standard double-click zoom;
matching the macOS custom double-click preference remains a separate refinement.

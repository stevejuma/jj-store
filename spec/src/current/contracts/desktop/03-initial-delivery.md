# Initial delivery

The first slice establishes supervised launch and the native window without
adding repository-privileged JavaScript APIs or relaxing the ordinary browser
CSP. Native folder-dialog integration, first-render handshake, zoom and
dirty-edit Quit protection remain subsequent gates. The existing server-owned
folder picker remains available through its authorized HTTP route.

`jj-store run --desktop-supervised` requires LocalTrusted literal-loopback mode
and a pipe on standard input. Optional desktop bind, web-root and RustFS-runtime
overrides apply only to that supervised launch and never rewrite installation
configuration. EOF requests the ordinary graceful shutdown; other launch modes
retain their existing signal behavior. Overrides cannot enable development
browser annotations, authenticated serving or external storage.

The host uses an empty private installation and the existing Projects hub. Fresh
initialization selects and records a private loopback port once. Future launches
retain it; a port collision is an error. Resource relocation overrides only the
executable and web bundle paths. Installation identity, credentials, catalog and
volume remain selected by the existing format-2 configuration.

Local development and qualification use explicit separate state paths. Public
signing, notarization, updates, login startup and additional platforms remain
deferred. Optional agent execution is governed by JST-COLLAB-031 through
JST-COLLAB-033. Screenshots are visual-pending and cannot qualify process
lifetime, publication, storage or accessibility.

## Native application branding

JST-DESK-001 packages repository-owned blue-jay and orange-gear artwork from
`desktop/icons`. The default PNG and multi-resolution macOS ICNS use the
transparent variant with a pale outer edge that retains contrast on light and
dark surfaces. Both light and dark SVG sources and exports remain owned assets.
Tauri uses the default PNG for its window/tray icon and the explicit ICNS for
the macOS bundle. Icon selection is static and does not change appearance
preferences. Branding preserves the installation, managed service and Dock/tray
lifecycle contracts in JST-DESK-001 and JST-DESK-004; complete app/DMG
qualification under JST-DESK-005 remains a separate gate.

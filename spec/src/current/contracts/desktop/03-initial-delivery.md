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
signing, notarization, updates, login startup, additional platforms and agent
execution remain deferred. Screenshots are visual-pending and cannot qualify
process lifetime, publication, storage or accessibility.

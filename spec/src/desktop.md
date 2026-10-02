# Desktop application

The desktop application uses Tauri 2 on macOS ARM64 and the existing production
Svelte application. Its bundled foreground service owns persistent embedded
RustFS; native presentation never becomes repository authority. The Buzz host at
ef2aa1ae38fadcc0bc22b8bf6ed96b35933146be is an architectural reference.

## Requirements

| ID           | Requirement                                                                                                                                                                                                                                                                    |
| ------------ | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| JST-DESK-001 | Desktop MUST use an independently locked host with explicit bundled executable and browser paths, a private installation outside checkouts, and no installed-tool replacement or implicit attachment to another service.                                                       |
| JST-DESK-002 | Desktop MUST validate its child's listening event and protected LocalTrusted bootstrap before loading the same-origin production UI. Its installation origin MUST survive restart; occupied ports and incompatible state MUST fail explicitly without reset.                   |
| JST-DESK-003 | Desktop MUST own a bounded parent-lifetime channel, startup and shutdown deadlines and captured output. Closing that channel MUST gracefully stop the server and its RustFS helper. Interrupted mutations retain their existing unknown-outcome semantics.                     |
| JST-DESK-004 | The macOS host MUST support single-instance reopening, retained window geometry, Dock/tray reopening and explicit Quit. Closing the main window hides it while its managed service remains alive. Native permissions MUST be restricted to the managed main window and origin. |
| JST-DESK-005 | Packaged ARM64 app and DMG qualification MUST use cold real-RustFS and WKWebView flows, including restart, relocation, failure and persistence. Native controls, dirty-edit protection and public distribution require their own evidence.                                     |

## Initial delivery

The first slice establishes supervised launch and the native window without
adding privileged JavaScript APIs or relaxing the ordinary browser CSP. Native
folder-dialog integration, first-render handshake, custom titlebar, zoom and
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
signing, notarization, updates, login startup, additional platforms, terminals
and agent execution remain deferred. Screenshots are visual-pending and cannot
qualify process lifetime, publication, storage or accessibility.

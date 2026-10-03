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
signing, notarization, updates, login startup, additional platforms, terminals
and agent execution remain deferred. Screenshots are visual-pending and cannot
qualify process lifetime, publication, storage or accessibility.

## macOS integrated header

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
`set_glass_background` presentation command. No wildcard origin, filesystem,
shell, dialog or generic core permission is granted. Window controls and links
inside the header retain their ordinary click behavior. This slice uses Tauri's
standard double-click zoom; matching the macOS custom double-click preference
remains a separate refinement.

## macOS Glass background

JST-DESK-004 and JST-UI-018 govern the Buzz glass adaptation. The NSWindow MUST
remain opaque. Enabling glass MUST clear any previous native material, install
Sidebar vibrancy and clear only the WKWebView canvas before the renderer exposes
transparency. The renderer MUST serialize native toggles and expose only the
current successfully installed effect; failure leaves opaque theme surfaces.
Disabling paints opaque CSS before clearing vibrancy. Changing only the theme or
tint opacity MUST NOT reinstall the native effect. Disposal clears the effect
without allowing late acknowledgements to change a destroyed renderer.

The exact managed origin and main window receive only the boolean appearance
command; no material selector, arbitrary window target or generic effects API is
exposed. Native glass uses one translucent theme wash over native blur with
30-90 percent tint opacity, the Buzz light/dark gradient and neutral 2.4 percent
wash. Nested chrome passes through that wash; main content and portalled
controls remain opaque. Ordinary browser glass retains its application-only
backdrop. The macOS private API feature is confined to this development host;
distribution qualification remains a separate gate.

AppKit focus, movement, resize, late window updates and glass relayout MUST
retain the same native control centers and native hover tracking. The host
restores geometry and tracking on the main UI thread using the original NSWindow
buttons; no HTML traffic-light replacements or custom hover glyphs are
introduced. One window-filtered AppKit update observer corrects late layout only
when geometry differs, with a reentrancy guard and no polling. Its retained
token is removed on host exit. Platform interop is confined to documented
NSWindow and retained view hierarchy borrows within Tauri's main-thread callback
or a live notification's retained window object; no platform pointer escapes or
becomes authority.

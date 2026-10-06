# macOS Glass background

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

# Desktop host boundary

The independently locked desktop host invokes explicit paired binaries and
serves the existing production web bundle through its managed LocalTrusted
listener. It has no dependencies on upstream JJ, model, WAL, storage or service
crates. The root Cargo workspace excludes the host. See
[desktop](../desktop/index.md) for lifetime and qualification contracts.

The desktop workspace owns `jj-store-terminal-engine`, adapted from the pinned
Buzz PTY/Alacritty implementation. Its bounded queues, parser, process cleanup
and in-memory grid depend only on terminal/platform libraries. The host resolves
eligible folders through existing protected loopback reads using a bounded HTTP
client, then exposes named exact-origin Tauri commands. Neither engine nor
renderer can select repository roots or publish metadata. The web bundle uses
the released Tauri JavaScript API, aligned to the host's minor version, while
the Svelte terminal controller owns tab selection and disposable subscriptions.
No shell HTTP route or core storage dependency is introduced.

Native glass and traffic-light layout depend only on the host's independently
locked Tauri, window-vibrancy and minimal AppKit bindings. The renderer receives
one boolean appearance command restricted to the managed main window and exact
origin. Native vibrancy preserves an opaque NSWindow while clearing only the
WKWebView canvas. Main-thread native geometry and tracking corrections borrow
the retained window hierarchy only within Tauri's callback or the window's
AppKit update notification. A single owned observer token is removed on host
exit; corrections are idempotent and do not poll. They store no platform pointer
and add no repository or filesystem authority.

Local identity hints remain inside the JJ adapter's read-only query metadata
boundary. Host configuration discovery and registered-folder JJ/Git fallback
MUST expose only bounded email; evaluator settings, aliases, storage, model and
WAL remain independent. HTTP composition admits the hint only for LocalTrusted
serving. Configuration reading MUST NOT invoke migration or publication.

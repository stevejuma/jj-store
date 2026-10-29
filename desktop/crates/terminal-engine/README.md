# jj-store-terminal-engine

Terminal emulation and supporting lifecycle helpers for the desktop repository
terminal. It provides grid state, parser fences, shared snapshots and damage
information for the renderer, using Alacritty and the maintained Buzz
adaptation.

Use `Terminal` and `SharedTerminal` in a host that owns the PTY session,
transport and window lifetime. The `listener`, `fences`, `reader` and
`lifecycle` modules provide the supporting integration points. This crate is
part of the desktop workspace rather than repository storage.

See [desktop development](../../../docs/desktop-development.md) and
[process lifecycle](../../../docs/site/technical/lifecycle.md).

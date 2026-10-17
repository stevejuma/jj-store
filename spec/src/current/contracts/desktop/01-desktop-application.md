# Desktop application

E-CRATE-READMES-001 moves the existing longer desktop development and terminal
notes into docs/desktop-development.md and introduces the terminal-engine crate.
JST-DESK-001/002/003 process ownership, private installation and qualification
remain unchanged; this documentation move adds no bundled Help or host behavior.

The desktop application uses Tauri 2 on macOS ARM64 and the existing production
Svelte application. Its bundled foreground service owns persistent embedded
RustFS; native presentation never becomes repository authority. The Buzz host at
ef2aa1ae38fadcc0bc22b8bf6ed96b35933146be is an architectural reference.

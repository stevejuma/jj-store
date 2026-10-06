# Desktop terminal isolation

JST-SEC-005 continues to prohibit arbitrary execution in server operations. The
separate native desktop terminal is an explicit local-user shell, not a server
command or repository sandbox. Its exact-window/origin command ACL and
server-revalidated folder targets are enforced in the host. Its child
environment excludes service credentials; terminal output and escape sequences
confer no clipboard, network API or filesystem bridge permissions. No HTTP
terminal or arbitrary shell Tauri plugin is exposed.

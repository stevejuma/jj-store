# Desktop terminal consumer

The desktop host consumes existing protected local repository registrations and
workspace-link reads to resolve terminal targets. These routes remain read-only
and do not execute shells. Terminal frames and controls use the desktop-only
Tauri interface, not an HTTP or repository publication API. Maximizing the dock
closes the application-owned details sidebar through local layout state; this
presentation action adds no terminal IPC command or server mutation.

# Desktop repository terminal

JST-UI-002 and JST-UI-006 include the native-only repository bottom dock.
Sessions belong to their original repository and folder across navigation. Hide
preserves the shell; Close terminates a tab. The shell retains repository and
editor state while rendering tabs, folder choice, accessible output, a resizable
dock and maximize/restore. Tab close is visually inline and independently
accessible; tabs and header controls have clear hover/focus states. A one-pixel
separator retains its resize hit area. The duplicate directory row is omitted,
and maximize closes the far-right details sidebar without reopening it on
restore. Cmd+J toggles only with a repository selected and returns focus on
hide. Unsupported browser clients do not advertise the terminal. Missing folders
explain how to link an existing folder; operational failures use the shared
notification service. Stories exercise the production dock with a fake bridge
and remain visual-pending.

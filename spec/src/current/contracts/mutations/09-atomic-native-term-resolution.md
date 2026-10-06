# Atomic native term resolution

ResolveConflicts names one visible commit and 1–256 distinct conflict paths.
Every selected path includes its complete read-side fingerprint and an explicit
SelectTerm (zero-based ordered native term) or Delete action. All paths and
fingerprints are checked against the original tree before writing any
replacement tree or commit. Root paths, malformed or overlapping paths,
non-conflicts, stale fingerprints and missing term positions fail atomically.
Selecting an absent term explicitly deletes the path; selecting a native file,
symlink, submodule or directory retains its exact identity, executable bit and
copy data. Remove terms may be selected explicitly, including the native base
version.

One native rewrite keeps the target change identity, rebases descendants and
updates bookmarks and logical workspaces. Unselected conflicts retain their
native values and labels, subject to native merge simplification. Preview does
not publish; apply uses the existing strict guarded receipt flow. The command
has no editor or shell execution. Authorized uploaded replacements use the
separate checked context described below; caller-selected file IDs are never
accepted as upload authority.

The service exports the owned ConflictResolution and Resolution types alongside
Command, so API consumers can construct typed intent without importing native JJ
integration internals.

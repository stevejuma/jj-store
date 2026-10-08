# Native folder selection for local settings

Local server composition supplies the separate owned folder-picker interface on
macOS using `/usr/bin/osascript` with a fixed folder-dialog script. Execution is
asynchronous, single flight, limited to 4096 path bytes and 25 seconds, and
cleans up children on timeout or caller cancellation. Other platforms advertise
no picker; missing helpers or headless sessions return unavailable and leave
manual path entry usable. No installed binary or service is changed by picking.
The same host adapter offers a fixed single-file GGUF dialog for Compute when
composed. It shares the folder dialog’s single-flight permit, bounds, deadline
and cancellation/reaping owner. Cancel returns no path; other formats are
refused. Selecting model weights fills a browser draft and never imports them.

The LocalTrusted admin server composes a browser Sync manager over the same
replication controls used by the CLI/RPC. It starts no additional worker and
modifies no upstream configuration. Paused state remains process-local.

Local Search management is composed only when the foreground CLI supplies its
startup-selected configuration path and LocalTrusted Admin access. Revision
checks cover the bounded file bytes; atomic replacement preserves unrelated TOML
values, credential references, source extraction limits and worker timing.
Disabled entries retain their desired configuration. Save alone leaves the
running worker unchanged; Apply changes only its repository projection.

The development demo retains its native local checkout and additionally seeds a
native server-only repository with no linked physical folder and an ordinary
colocated Git-backed JJ repository. A temporary server-only seed checkout is
forgotten and removed after acknowledged native publication. A private atomic
profile marker retains distinct identities across restart. The original three
projects remain in the native home and link one profile each. On upgrade,
unchanged sample project links and task targets may migrate using exact current
versions; edited records and retired links remain untouched. Local Git metadata
is initialized through the owned CLI, not written by the harness. Its project
link remains separately inaccessible in the browser until M13.4 serving passes.

`pnpm cli:install` builds locked release binaries and installs `jj-store` plus
`jj-store-jj` into `~/.local/bin` or an explicit `--bin-dir`. The custom JJ tool
uses a distinct name to preserve the existing `jj`. `--debug --no-build` selects
already-built development artifacts. Missing artifacts fail preflight; file
replacement uses a temporary executable and rename, including when the old
destination is a symlink. Installation does not start services or alter PATH.

The representative agent seed binds every setup, capture and remove subprocess
to its fixture working directory. The explicit `-R` argument alone cannot select
the installation target. Source checkout configuration remains untouched.

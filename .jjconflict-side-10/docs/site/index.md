# jj-store

jj-store brings native JJ repository storage, a browser and desktop interface,
and repository-owned collaboration into one local-first application. You can
browse immutable history, manage projects and tasks, inspect captured agent
activity, and discuss exact repository artifacts without treating a search index
or a browser cache as the source of truth.

## Start with your context

- **New native repository:** follow the
  [native tutorial](getting-started/native.md). A local service owns immutable
  objects and publication in managed RustFS.
- **Existing Git-backed JJ folder:** follow the
  [local tutorial](getting-started/local.md). Code stays in its existing
  repository; collaboration uses an independently selected Git metadata ref.
- **Daily browser or desktop use:** start with [browsing](guides/browsing.md),
  [projects and tasks](guides/work.md), or [capture](guides/capture.md).
- **Operating an installation:** read
  [configuration](reference/configuration.md), [access](reference/security.md),
  and [maintenance](guides/settings.md).
- **Integrating or developing:** use the [CLI reference](cli/index.md),
  [transport semantics](reference/transport.md), and
  [architecture](technical/architecture.md).

## What this manual describes

This manual travels with the application build. The standalone site and sidebar
Help contain the same pages. Help is read-only and works without an internet
connection; following an external reference still requires a connection. Closing
Help returns to the application view you were using.

Features differ by repository profile. A visible parser option or transport
route does not make that feature available in every context. The
[capability reference](reference/capabilities.md) identifies important
boundaries, including local filesystem limitations, capture versions and
deployment support.

The application uses upstream JJ semantics where appropriate, but this manual
covers jj-store behavior. For ordinary JJ commands and its full revset language,
use the [JJ 0.45.1 manual](https://docs.jj-vcs.dev/v0.45.1/).

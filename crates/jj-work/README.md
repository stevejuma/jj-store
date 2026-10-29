# jj-work

Storage-independent projects, repository links, task hierarchy, task state and
threaded comments. Both native repositories and Git companion metadata use the
same WorkStore commands and domain rules.

Apply a `WorkCommandV1` to `WorkStateV1` with an explicit actor and timestamp.
Persist the returned state and result through a WorkStore provider, retaining
the command's expected versions for concurrency control.

See [projects and tasks](../../docs/site/guides/work.md) and
[project concepts](../../docs/site/concepts/projects.md).

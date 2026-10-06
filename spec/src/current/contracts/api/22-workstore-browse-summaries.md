# WorkStore browse summaries

`GET /v1/repos/{repo}/work/browse` exposes typed project/task summary pages.
Optional project, repository, parent, state and title/body substring filters are
applied over the complete selected root before paging. Query text is bounded to
512 UTF-8 bytes; limits remain 1-256 records and four MiB per response. Exact
query, home, epoch and root binding, reauthorization, five-minute expiry and
resource admission match existing WorkStore reads. Existing detail and mutation
contracts retain their meaning.

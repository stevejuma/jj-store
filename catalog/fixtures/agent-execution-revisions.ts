export function revisionResponse(repository: string, operation: string) {
  return {
    snapshot: { repository, operation, profile: "git_sha1_jj045" },
    data: {
      kind: "references",
      data: {
        operation,
        references: [
          {
            kind: "bookmark",
            name: "main",
            targets: [{ commit: "77".repeat(20), description: "Main branch" }],
          },
        ],
      },
    },
    next_cursor: null,
  };
}

export function revisionFixture(
  repository: string,
  operation: string,
  required: boolean,
) {
  let ready = !required;
  const json = (body: unknown, status = 200) =>
    new Response(JSON.stringify(body), {
      status,
      headers: { "content-type": "application/json" },
    });
  const snapshot = {
    version: 1,
    repository,
    operation,
    generation: 1,
    profile: "git_sha1_jj045",
  };
  return (path: string) => {
    if (path.endsWith("/snapshot"))
      return json({
        version: 1,
        repository,
        generation: 1,
        profile: "git_sha1_jj045",
        heads: [operation],
        metadata_commit: null,
        actor: null,
      });
    if (path.includes("/index/tasks")) {
      ready = true;
      return json({
        version: 1,
        id: "88".repeat(16),
        snapshot,
        status: "ready",
      });
    }
    if (path.includes("/query"))
      return ready
        ? json(revisionResponse(repository, operation))
        : json(
            {
              version: 1,
              code: "index_rebuild_required",
              local_snapshot: snapshot,
            },
            409,
          );
  };
}

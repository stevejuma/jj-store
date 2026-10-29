export const stackRepository = "b".repeat(32),
  stackOperation = "a".repeat(128);
const base = "1".repeat(40),
  topic = "2".repeat(40),
  parent = "3".repeat(40);
const signature = {
  name: "Alex Morgan",
  email: "alex@example.test",
  timestamp: { millis: 123000, offset_minutes: 0 },
};
const pin = {
  version: 1,
  profile: "git_sha1_jj045",
  repository: stackRepository,
  generation: 2,
  operation: stackOperation,
};
const commit = (id: string, description: string, parents = [base]) => ({
  id,
  change_id: "c".repeat(32),
  parents,
  description,
  author: signature,
  committer: signature,
  tree_terms: [],
  conflicted: false,
  divergent: false,
  visible: true,
  bookmarks: [] as {
    name: string;
    remote: string | null;
    added: boolean;
    removed: boolean;
    conflicted: boolean;
  }[],
  workspaces: [],
});
const nodes = [
  commit(topic, "Topic tip", [parent]),
  commit(parent, "Topic ancestor"),
];
const reference = (name: string, node: ReturnType<typeof commit>) => ({
  kind: "bookmark",
  name,
  terms: [node.id],
  conflicted: false,
  workspaces: [],
  targets: [
    {
      commit: node.id,
      change: node.change_id,
      parents: node.parents,
      description: node.description,
      author: signature,
      committer: signature,
      conflicted: false,
      workspaces: [],
    },
  ],
});

export function fixtureNativeHistory(dependent: boolean) {
  const history = structuredClone(nodes);
  if (dependent)
    history[1]!.bookmarks = [
      {
        name: "foundation",
        remote: null,
        added: true,
        removed: false,
        conflicted: false,
      },
    ];
  return history;
}

export { base, topic, signature, pin, commit, nodes, reference };

export function fixtureStackDiff(
  query: any,
  editing: boolean,
  dragRead: string,
) {
  const q = query.query;
  return {
    version: 1,
    from: q.from,
    to: q.to,
    ...(query.kind === "diff" ? { path: q.path } : {}),
    changes:
      editing && dragRead !== "ambiguous" && q.to === topic && q.from !== base
        ? []
        : [
            {
              path: "src/topic.ts",
              before: [{ type: "file", id: base, executable: false }],
              after: [{ type: "file", id: topic, executable: false }],
              sources: [],
              content:
                query.kind === "diff"
                  ? {
                      status: "text",
                      hunks: [
                        {
                          kind: "different",
                          before_start: 0,
                          after_start: 0,
                          before: "old\n",
                          after: "new\n",
                        },
                      ],
                    }
                  : { status: "unavailable", reason: "not_requested" },
            },
          ],
    next: null,
    previous: null,
  };
}

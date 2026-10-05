import type { StoreRouteBrowser } from "../../../web/src/lib/controller/store-route-history.svelte.js";
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
  bookmarks: [],
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
const json = (value: unknown, status = 200) =>
  new Response(JSON.stringify(value), {
    status,
    headers: { "content-type": "application/json" },
  });

/** Isolated HTTP fixtures exercise the production bootstrap, lifecycle and adapter. */
export function productionStacksHttp(cold = false) {
  const requests: { path: string; body: any }[] = [];
  let prepared = !cold,
    hash = `#/repos/${stackRepository}/stacks?at_op=${stackOperation}`;
  const browser: StoreRouteBrowser = {
    hash: () => hash,
    push: (next) => (hash = next),
    replace: (next) => (hash = next),
    go: () => {},
    listen: () => () => {},
  };
  const fetcher: typeof fetch = async (input, init) => {
    const path = String(input),
      body = init?.body ? JSON.parse(String(init.body)) : null;
    requests.push({ path, body });
    if (path === "/v1/bootstrap")
      return json({
        version: 1,
        access_mode: "local_trusted",
        storage: { ready: true },
        backups: [],
        capabilities: {},
      });
    if (path.startsWith("/v1/repositories?"))
      return json({
        version: 1,
        generation: 1,
        repositories: [
          {
            id: stackRepository,
            name: "Registered JJ folder",
            permission: null,
            profile: "git_sha1_jj045",
            availability: "ready",
          },
        ],
      });
    if (path === "/v1/local/repositories")
      return json({
        version: 1,
        revision: 1,
        bindings: [
          {
            repository: stackRepository,
            name: "Registered JJ folder",
            path: "/registered/topic",
            generation: 2,
            availability: "ready",
          },
        ],
      });
    if (path.endsWith("/stacks"))
      return json({
        version: 1,
        operation: stackOperation,
        revision: 0,
        active: null,
        workspaces: [],
        warning: null,
      });
    if (path.endsWith("/snapshot"))
      return json({
        ...pin,
        heads: [stackOperation],
        metadata_commit: null,
        actor: null,
      });
    if (path.endsWith("/working-tree/observe"))
      return json({
        snapshot: {
          version: 1,
          repository: stackRepository,
          binding_generation: 2,
          operation: stackOperation,
          view: "c".repeat(128),
          workspace: "default",
          working_copy: "9".repeat(40),
          trees: ["8".repeat(40)],
          tree_labels: [],
          physical_workspace: "7".repeat(64),
          metadata_fingerprint: "6".repeat(64),
          physical_fingerprint: "5".repeat(64),
        },
        changes: [],
        obstacle_count: 0,
      });
    if (path.endsWith("/capabilities"))
      return json({
        version: 1,
        profile: "git_sha1_jj045",
        code_browse: true,
        code_mutations: false,
        index_tasks: true,
        work: false,
        agents: false,
        reviews: false,
        search: false,
      });
    if (path.includes("/events"))
      return new Response("", {
        headers: { "content-type": "text/event-stream" },
      });
    if (path.includes("/work/browse"))
      return json({
        version: 1,
        snapshot: {
          profile: "local_jj_git",
          repo_id: stackRepository,
          metadata_commit: null,
          work_state_version: 0,
        },
        projects: [],
        tasks: [],
        total_matches: 0,
        next_cursor: null,
      });
    if (path.endsWith("/index/tasks")) {
      prepared = true;
      return json({
        version: 1,
        id: "d".repeat(32),
        snapshot: pin,
        status: "ready",
      });
    }
    if (path.endsWith("/query")) {
      const query = body.request;
      if (!prepared)
        return json(
          { code: "index_rebuild_required", local_snapshot: pin },
          409,
        );
      let data;
      if (query.kind === "references")
        data = {
          version: 1,
          operation: stackOperation,
          references: [
            reference("main", commit(base, "Workspace target", [])),
            reference("topic", nodes[0]!),
          ],
          next: null,
          previous: null,
        };
      else if (query.kind === "stack_history")
        data = {
          version: 1,
          operation: stackOperation,
          head: query.query.head,
          base: query.query.base,
          commits: (query.query.head === topic
            ? nodes
            : [commit(base, "Workspace target", [])]
          ).map((commit) => ({
            commit,
            edges: commit.parents.map((target) => ({ target, kind: "direct" })),
          })),
          next: null,
        };
      else if (query.kind === "diff_paths" || query.kind === "diff") {
        const q = query.query;
        data = {
          version: 1,
          from: q.from,
          to: q.to,
          ...(query.kind === "diff" ? { path: q.path } : {}),
          changes: [
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
      } else
        throw new Error(`Unexpected production Stacks query: ${query.kind}`);
      return json({
        snapshot: pin,
        data: { kind: query.kind, data },
        next_cursor: null,
        previous_cursor: null,
      });
    }
    throw new Error(`Unexpected production Stacks request: ${path}`);
  };
  return { browser, fetcher, requests };
}

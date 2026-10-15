import type {
  ForgeConnection,
  ForgePr,
  ForgePlan,
  ForgeResponse,
  ForgeTransport,
} from "../../../web/src/lib/controller/forge/types.js";

type ForgeChecks = NonNullable<ForgeResponse["checks"]>;

const connection: ForgeConnection = {
  provider: "github",
  api_url: "https://api.github.com",
  repository: "team/repo",
  source_repository: "team/repo",
  remote: "origin",
  target_remote: "origin",
  target: "main",
  native_stacks: false,
};
const navigation =
  "\n\n<!-- jj-store:stack:v1 -->\n\n### Stack\n\n1. [#41](https://github.com/team/repo/pull/41) · Open\n2. [#42](https://github.com/team/repo/pull/42) · Open\n<!-- /jj-store:stack -->";
export function workspaceForgeTransport() {
  const prs: ForgePr[] = [
    {
      number: 41,
      head: "feature/model",
      title: "Add repository model",
      base: "main",
      body: "Introduce the repository model and its validation." + navigation,
      sha: "1".repeat(40),
      draft: false,
    },
    {
      number: 42,
      head: "feature/api",
      title: "Expose repository API",
      base: "feature/model",
      body:
        "Expose query and mutation endpoints on the repository model.\n\nThe build is running; the lower PR is ready for review." +
        navigation,
      sha: "2".repeat(40),
      draft: true,
    },
    {
      number: 43,
      head: "feature/workspace",
      title: "Add workspace toolbar",
      base: "main",
      body: "Add the workspace picker and keyboard shortcuts.",
      sha: "3".repeat(40),
      draft: false,
    },
  ].map((p): ForgePr => ({
    ...p,
    url: `https://github.com/team/repo/pull/${p.number}`,
    state: "open",
    mergeable: true,
    native_stack: null,
    updated_at: "2026-10-15T10:00:00Z",
  }));
  const states = new Map<string, "running" | "passed" | "failed" | "cancelled">(
    [
      ["feature/model", "passed"],
      ["feature/api", "running"],
      ["feature/workspace", "failed"],
    ],
  );
  function checks(pr: ForgePr, jobs = false): ForgeChecks {
    const run = String(pr.number);
    return {
      sha: pr.sha,
      complete: true,
      checks: [
        {
          id: jobs ? `${run}1` : run,
          name: jobs ? "Unit tests" : "Build and test",
          state: states.get(pr.head) ?? "unknown",
          sha: pr.sha,
          url: `https://github.com/team/repo/actions/runs/${run}`,
          run_id: run,
          log_id: jobs ? `${run}1` : null,
        },
      ],
    };
  }
  const response = (): ForgeResponse => ({
    notice: null,
    connection,
    capabilities: {
      edit: true,
      draft: true,
      merge: true,
      auto_merge: false,
      checks: true,
      logs: true,
      rerun: true,
      rerun_all: true,
      cancel: true,
      merge_methods: ["merge", "squash", "rebase"],
      native_stacks: false,
    },
    pull_requests: prs.map((p) => ({ ...p })),
    plan: null,
    receipt: null,
    restack: null,
    checks: null,
    log: null,
    merge: null,
    merges: {},
    operation: null,
  });
  const transport: ForgeTransport = async ({
    action,
  }): Promise<ForgeResponse> => {
    if (action.kind === "preview") {
      const members =
        action.bookmark === "feature/api"
          ? prs.slice(0, 2)
          : prs.filter((p) => p.head === action.bookmark);
      return {
        ...response(),
        plan: {
          request_id: `preview-${action.bookmark.replaceAll("/", "-")}`,
          operation: "a".repeat(128),
          connection,
          draft: action.draft,
          cumulative: false,
          items: members.map((p) => ({
            segment: {
              branch: p.head,
              head: p.sha,
              base: p.base,
              title: p.title,
              body: p.body,
              remote_head: "0".repeat(40),
              shared: false,
            },
            pr: { ...p },
            steps: ["push", "description"],
          })),
        },
      };
    }
    if (action.kind === "submit") {
      const plan: ForgePlan = {
        ...action.plan,
        connection,
        items: action.plan.items.map((item) => ({
          ...item,
          segment: {
            ...item.segment,
            remote_head: item.segment.remote_head ?? null,
          },
          pr: prs.find((pr) => pr.head === item.segment.branch) ?? null,
        })),
      };
      return {
        ...response(),
        receipt: {
          plan,
          outcome: "complete",
          message: null,
          pull_requests: prs.map((p) => ({ ...p })),
          steps: action.plan.items.flatMap((i) =>
            i.steps.map((kind) => ({
              branch: i.segment.branch,
              kind,
              outcome: "complete" as const,
              fingerprint: null,
            })),
          ),
        },
      };
    }
    if (action.kind === "pr" || action.kind === "pull_request") {
      const pr = prs.find((p) => p.number === action.number)!;
      const result = response();
      if (action.kind === "pr") {
        const command = action.action;
        if (command.kind === "edit") {
          pr.title = command.title;
          pr.body = command.body;
        }
        if (command.kind === "draft") pr.draft = command.draft;
        if (command.kind === "checks" || command.kind === "jobs")
          result.checks = checks(pr, command.kind === "jobs");
        if (command.kind === "logs")
          result.log =
            "Run unit tests\n42 passed · repository model and API contracts\nWaiting for browser checks…";
        if (command.kind === "rerun") states.set(pr.head, "running");
        if (command.kind === "cancel") states.set(pr.head, "cancelled");
        if (command.kind === "merge") {
          pr.state = "merged";
          result.merge = { state: "merged", receipt: null, sha: pr.sha };
        }
      }
      result.pull_requests = [{ ...pr }];
      return result;
    }
    return response();
  };
  return {
    transport,
    initial: response(),
    ci: Object.fromEntries(prs.map((p) => [p.head, checks(p)])),
  };
}

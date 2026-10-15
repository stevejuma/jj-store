import type {
  ForgeConnection,
  ForgePr,
  ForgePlan,
  ForgeResponse,
  ForgeTransport,
  ForgeCheck,
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
export function workspaceForgeTransport(
  workspacePullRequest = false,
  apiPullRequests = true,
  failPreview = false,
  unknownSubmit = false,
  options: {
    apiCheckState?: ForgeCheck["state"];
    apiChecksComplete?: boolean;
    apiLocalHead?: string;
  } = {},
) {
  const examples: ForgePr[] = [
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
  const prs = examples.filter((pr) =>
    pr.head === "feature/workspace" ? workspacePullRequest : apiPullRequests,
  );
  const states = new Map<string, ForgeCheck["state"]>([
    ["feature/model", "passed"],
    ["feature/api", options.apiCheckState ?? "running"],
    ["feature/workspace", "failed"],
  ]);
  function checks(pr: ForgePr, jobs = false): ForgeChecks {
    const run = String(pr.number);
    return {
      sha: pr.sha,
      complete:
        pr.head === "feature/api" ? (options.apiChecksComplete ?? true) : true,
      checks: [
        ...(!jobs
          ? ["Typecheck", "Unit tests"].map((name, index) => ({
              id: `${run}-passed-${index}`,
              name,
              state: "passed" as const,
              sha: pr.sha,
              url: null,
              run_id: null,
              log_id: null,
              started_at: "2026-10-15T10:00:00Z",
              completed_at: "2026-10-15T10:00:04Z",
            }))
          : []),
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
  let recovered: ForgeResponse | undefined;
  const transport: ForgeTransport = async ({
    action,
  }): Promise<ForgeResponse> => {
    if (action.kind === "recover" && recovered) return recovered;
    if (action.kind === "preview") {
      if (failPreview)
        throw new Error("The forge could not prepare this pull request");
      const members =
        action.bookmark === "feature/api"
          ? examples.slice(0, 2)
          : examples.filter((p) => p.head === action.bookmark);
      return {
        ...response(),
        plan: {
          request_id: `preview-${action.bookmark.replaceAll("/", "-")}`,
          operation: "a".repeat(128),
          connection,
          draft: action.draft,
          cumulative: false,
          items: members.map((p) => {
            const pr = prs.find((item) => item.head === p.head);
            return {
              segment: {
                branch: p.head,
                head:
                  p.head === "feature/api"
                    ? (options.apiLocalHead ?? p.sha)
                    : p.sha,
                base: p.base,
                title: p.title,
                body: pr
                  ? pr.body
                  : p.body.split("\n\n<!-- jj-store:stack:v1 -->")[0]!,
                remote_head: pr ? "0".repeat(40) : null,
                shared: false,
              },
              pr: pr ? { ...pr } : null,
              steps: pr
                ? ["push", "description"]
                : ["push", "create", "description"],
            };
          }),
        },
      };
    }
    if (action.kind === "submit") {
      for (const item of action.plan.items) {
        const existing = prs.find((pr) => pr.head === item.segment.branch);
        if (existing) {
          existing.sha = item.segment.head;
          continue;
        }
        const example = examples.find((pr) => pr.head === item.segment.branch);
        if (example)
          prs.push({
            ...example,
            sha: item.segment.head,
            title: item.segment.title,
            body: item.segment.body,
            draft: action.plan.draft,
          });
      }
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
      recovered = {
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
      if (unknownSubmit) throw new Error("Network submission outcome unknown");
      return recovered;
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

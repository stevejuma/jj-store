import type {
  WorkflowPage,
  WorkflowVersion,
  WorkflowTransport,
  WorkflowRequest,
  WorkflowRun,
  WorkflowEvidence,
} from "../../web/src/lib/controller/workflow-types.js";
import { initialDefinition } from "../../web/src/lib/ui/store-workflows/workflow-presentation.js";
export const home = "41".repeat(16),
  code = "42".repeat(16),
  actor = "01".repeat(16);
export const repositories = [
  { id: home, name: "Automation home", permission: "admin" as const },
  { id: code, name: "Parser", permission: "admin" as const },
];
export function workflowFixture(
  unknown = false,
  repository = home,
): WorkflowTransport {
  let position = 1;
  let versions: WorkflowVersion[] = [
    "Build parser",
    "Agent review",
    "Publish preview",
  ].map((name, index) => ({
    id: (11 + index).toString().repeat(16),
    version: (21 + index).toString().repeat(16),
    owner: actor,
    created_at: 1791880000000,
    definition: {
      ...initialDefinition(code),
      name,
      enabled: index !== 1,
      description: index === 0 ? "Build and test the recorded revision." : "",
      trigger: { on: "bookmark_changed", bookmark: "main" },
      actions:
        index === 1
          ? [
              {
                id: "review",
                executor: {
                  type: "agent",
                  agent: "reviewer",
                  prompt: "Review the source change",
                },
                inputs: {},
                timeout_seconds: 300,
              },
            ]
          : initialDefinition(code).actions,
    },
  }));
  const receipts = new Map<string, WorkflowRequest>();
  if (repository !== home) versions = [];
  const run: WorkflowRun | null = versions[0]
    ? {
        id: "51".repeat(16),
        workflow: versions[0].id,
        version: versions[0].version,
        event: null,
        requester: actor,
        created_at: 1791880000000,
        source: {
          repository: code,
          operation: "61".repeat(64),
          commit: "62".repeat(64),
          bookmark: "main",
        },
        attempts: [
          {
            id: "52".repeat(16),
            owner: actor,
            host: "linux-builder",
            status: "completed",
            sequence: 3,
            started_at: 1791880000000,
            updated_at: 1791880000100,
            completed_at: 1791880000100,
            result: "Tests passed",
          },
        ],
      }
    : null;
  const evidence: WorkflowEvidence[] = run
    ? [
        { type: "step", step: "build", status: "completed", output: {} },
        { type: "log", stream: "stdout", text: "12 tests passed\n" },
        {
          type: "artifact",
          name: "result.txt",
          media_type: "text/plain",
          bytes: [111, 107],
        },
      ].map(
        (data, sequence) =>
          ({
            id: `${71 + sequence}`.repeat(16),
            run: run.id,
            attempt: run.attempts[0]!.id,
            sequence,
            at: 1791880000100,
            data,
          }) as WorkflowEvidence,
      )
    : [];
  const page = (): WorkflowPage => ({
    version: 1,
    repository,
    epoch: "31".repeat(16),
    position,
    root: "32".repeat(32),
    workflows: versions,
    runs: [],
    evidence: [],
    next: null,
  });
  const result = (request: WorkflowRequest) => ({
    status: "committed" as const,
    receipt: {
      version: 1,
      request_id: request.request_id,
      publication_id: request.request_id,
      request_digest: "34".repeat(32),
      accepted: { epoch: "31".repeat(16), position, heads: [] },
      result: {
        workflow: null,
        definition: null,
        evaluation: null,
        runs: [],
        evidence: null,
        removed: false,
      },
    },
  });
  return {
    async page(query) {
      const value = page();
      if (query.workflow)
        value.workflows = versions.filter((v) => v.id === query.workflow);
      if ((query.workflow || query.run) && run) value.runs = [run];
      if (query.run) value.evidence = evidence;
      return JSON.parse(JSON.stringify(value)) as WorkflowPage;
    },
    async apply(request) {
      receipts.set(request.request_id, request);
      if (unknown)
        return { status: "outcome_unknown", request_id: request.request_id };
      if (request.command.type === "put") {
        const command = request.command;
        versions = versions.filter((v) => v.id !== command.workflow);
        versions.push({
          id: command.workflow,
          version: "35".repeat(16),
          owner: actor,
          created_at: 1791880000000,
          definition: {
            ...initialDefinition(code),
            ...command.definition,
            actions: command.definition.actions.map((a) => ({
              ...a,
              inputs: a.inputs ?? {},
              timeout_seconds: a.timeout_seconds ?? 300,
            })),
          },
        });
      } else if (request.command.type === "remove") {
        const workflow = request.command.workflow;
        versions = versions.filter((v) => v.id !== workflow);
      }
      position++;
      return result(request);
    },
    async status(id) {
      const request = receipts.get(id);
      return request && !unknown
        ? result(request)
        : { status: "not_known", request_id: id };
    },
  };
}

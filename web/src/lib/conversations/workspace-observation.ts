import { registerWorkspaceSource } from "./workspace-source.js";
import { savedTurnInspection } from "../controller/agent-turn-changes.js";
import type {
  AgentTurnSelection,
  AgentChangeSummary,
} from "../controller/agent-workspace-changes.js";
import type {
  WorkspaceBrowser,
  WorkspaceRead,
} from "./workspace-browser.svelte.js";
import type { operations } from "../api/generated.js";
export type WorkspaceRequest = NonNullable<
  operations["local_agents_workspace_inspect"]["requestBody"]
>["content"]["application/json"];
export async function selectWorkspaceTab(
  browser: WorkspaceBrowser,
  tab: "files" | "changes",
) {
  if (
    tab === "changes" &&
    browser.binding?.source === "repository" &&
    browser.range === "branch" &&
    !browser.base
  )
    browser.range = "since_last_commit";
  browser.tab = tab;
  await browser.load();
}
export function workspaceQuery(
  browser: WorkspaceBrowser,
  path: string | null = null,
  cursor: string | null = null,
  files = false,
): WorkspaceRequest {
  const b = browser.binding;
  if (!b) throw new Error("This agent has no available code workspace.");
  return {
    version: 1,
    session: b.session,
    repository: b.repository,
    path,
    cursor,
    query:
      files || browser.tab === "files"
        ? { view: "files" }
        : { view: "changes", range: browser.range, base: browser.base },
  };
}
export async function readWorkspace(
  browser: WorkspaceBrowser,
  request: WorkspaceRequest,
  signal?: AbortSignal,
) {
  if (
    browser.savedTurn &&
    request.query.view === "changes" &&
    request.query.range === "last_turn" &&
    request.session === browser.savedTurn.selection.session &&
    request.repository === browser.savedTurn.selection.repository
  ) {
    return savedTurnInspection(
      browser.savedTurn.selection,
      browser.savedTurn.summary,
      browser.binding?.workspace ?? "Saved turn",
      request.path ?? null,
    );
  }
  if (request.session.startsWith("repository:")) {
    if (!browser.repositoryReads)
      throw new Error("Repository browsing is unavailable.");
    return browser.repositoryReads.read(
      request.repository,
      request.path ?? null,
      request.cursor ?? null,
      signal,
      request.query.view === "changes"
        ? { range: request.query.range, base: request.query.base ?? null }
        : undefined,
    );
  }
  const value = await browser.client.request<WorkspaceRead>(
    "agents/workspace/inspect",
    request,
    signal,
  );
  if (
    value.session !== request.session ||
    value.repository !== request.repository ||
    (request.path && value.file?.path !== request.path)
  )
    throw new Error("The workspace selection changed. Please select it again.");
  return value;
}
export function showSavedTurn(
  browser: WorkspaceBrowser,
  selection: AgentTurnSelection,
  summary: AgentChangeSummary,
) {
  const binding = browser.bindings.find(
    (b) =>
      b.session === selection.session && b.repository === selection.repository,
  );
  if (!binding)
    browser.bindings = [
      ...browser.bindings,
      {
        session: selection.session,
        repository: selection.repository,
        repositoryName: selection.repository.slice(0, 8),
        workspace: "Saved turn",
        agent: "",
      },
    ];
  browser.bindingKey = `${selection.session}:${selection.repository}`;
  browser.tab = "changes";
  browser.range = "last_turn";
  browser.base = null;
  browser.savedTurn = { selection, summary };
  browser.listing.value = savedTurnInspection(
    selection,
    summary,
    browser.binding!.workspace,
  );
  browser.listing.dirty = false;
  browser.listing.error = "";
  browser.details = browser.listing.value;
}

export function registerWorkspaceRead(
  browser: WorkspaceBrowser,
  value: WorkspaceRead,
  reveal: () => void,
) {
  if (value.session.startsWith("repository:")) {
    browser.blocks =
      value.file?.contents != null
        ? {
            after: {
              id: value.selection + ":" + value.file.path,
              revision: value.commit,
              text: value.file.contents,
              kind: "code",
              path: value.file.path,
              label: value.file.path,
              commentable: false,
            },
          }
        : {};
    return;
  }
  registerWorkspaceSource(browser, value, reveal);
}

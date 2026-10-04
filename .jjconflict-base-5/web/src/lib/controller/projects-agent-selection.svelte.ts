import { ApiError } from "../api/client.js";
import type { StoreRoute } from "./contracts.js";
import type { ProjectAgentResponse } from "./projects-hub-types.js";
import type { ProjectAgentHome } from "./projects-agents.svelte.js";

export function createProjectsAgentSelection(context: {
  route(): StoreRoute;
  push(route: StoreRoute): void;
  homes(): readonly ProjectAgentHome[];
  response?(home: string): ProjectAgentResponse;
}) {
  const responses = new Map<string, ProjectAgentResponse>();
  let selected = $state<ProjectAgentResponse>(),
    error = $state(""),
    key = "",
    generation = 0;
  async function restore(refresh = false) {
    const route = context.route();
    const home = route.workAgentRepository,
      id = route.agentSessionId,
      next = home && id ? `${home}:${id}` : "";
    const allowed =
      !home || context.homes().some((entry) => entry.repository.id === home);
    if (!refresh && key === next && selected && allowed) return;
    const ticket = ++generation;
    selected?.reset();
    selected = undefined;
    error = "";
    key = next;
    if (!home || !id) return;
    try {
      if (!allowed) throw new ApiError("access_denied");
      let response = responses.get(home);
      if (!response) {
        if (!context.response) throw new Error("Agent response unavailable");
        response = context.response(home);
        responses.set(home, response);
      }
      selected = response;
      await response.load(id);
    } catch (cause) {
      if (ticket === generation)
        error =
          cause instanceof Error ? cause.message : "Agent response unavailable";
    }
  }
  return {
    get response() {
      return selected?.controller;
    },
    get selectionError() {
      return error;
    },
    restoreSelection: restore,
    async select(home: string | null, id: string | null) {
      context.push({
        ...context.route(),
        workAgentRepository: home,
        agentSessionId: id,
      });
      await restore();
    },
    clearSelection() {
      generation++;
      selected?.reset();
      selected = undefined;
      key = "";
      error = "";
    },
    resetSelections() {
      generation++;
      for (const response of responses.values()) response.reset();
      responses.clear();
      selected = undefined;
      key = "";
      error = "";
    },
  };
}

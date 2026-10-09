import { AgentSettingsFixture } from "./agent-settings-fixture.svelte.js";
import type { AgentStatus } from "../../web/src/lib/controller/agent-execution-types.js";
import { fixtureJsonResponse as json } from "./agent-execution-session.js";
/** Setup CAS and warm-only control receipts shared by the execution stories. */
export class AgentExecutionConfigurationFixture {
  defaults = new AgentSettingsFixture();
  controls = $state<Record<string, unknown>[]>([]);
  constructor(
    readonly get: () => AgentStatus,
    readonly set: (value: AgentStatus) => void,
    private staleSave: boolean,
  ) {}
  respond(path: string, init?: RequestInit) {
    const defaults = this.defaults.respond(path, init);
    if (defaults) return defaults;
    if (path.includes("/metadata?"))
      return json({
        version: 1,
        repository: this.get().sessions[0]?.repository ?? "11".repeat(16),
        target: { type: "conversation", id: "44".repeat(16) },
        thread: null,
        revision: null,
        value: { custom_title: null, generated: null, agent_titles: {} },
        sources: [],
        title: "Fixture conversation",
        fresh: false,
        writable: false,
        unavailable:
          "Catalog metadata writes are simulated in the metadata family.",
        details: [],
        job: null,
      });
    if (!path.endsWith("/control") && !path.endsWith("/setups")) return;
    const body = JSON.parse(String(init?.body)),
      status = this.get();
    if (path.endsWith("/control")) {
      this.controls = [...this.controls, body];
      status.live_agents = [
        {
          agent_id: body.agent_id,
          state: body.start ? "idle" : "offline",
          enabled: body.start,
          generation: 1,
          error: null,
          restart_required: false,
          inference_host: null,
        },
      ];
      return json({
        version: 1,
        request_id: body.request_id,
        session_id: null,
        state: body.start ? "starting" : "stopping",
        error: null,
      });
    }
    if (this.staleSave) {
      this.staleSave = false;
      status.revision = Number(status.revision) + 1;
    }
    if (Number(body.expected_revision) !== Number(status.revision))
      return json({ code: "agent_selection_changed" }, 409);
    const value = {
      ...status,
      revision: Number(status.revision) + 1,
      setups: [
        ...status.setups.filter(
          (s) => s.id !== (body.setup?.id ?? body.remove),
        ),
        ...(body.setup ? [body.setup] : []),
      ],
    };
    this.set(value);
    return json(value);
  }
}

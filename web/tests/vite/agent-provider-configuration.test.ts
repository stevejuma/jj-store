import { expect, test, vi } from "vitest";
import {
  changeProvider,
  changeHarness,
  changeEndpoint,
  sameLaunchOptions,
  type AgentConfiguration,
} from "../../src/lib/controller/agent-configuration.js";
import { AgentModelCatalog } from "../../src/lib/controller/agent-model-catalog.svelte.js";
import { LocalClient } from "../../src/lib/api/local-client.js";
function draft(): AgentConfiguration {
  return {
    runner: "local",
    preferences: { model: "old", effort: "high", mode: "ask" },
    options: {
      inherit_defaults: true,
      provider: "openai",
      endpoint: "https://old.example/v1",
      environment: {},
    },
  };
}
test("explicit provider, endpoint and harness changes clear dependent selections", () => {
  const d = draft();
  changeProvider(d, "anthropic");
  expect(d.preferences).toEqual({ model: null, effort: null, mode: "ask" });
  expect(d.options.endpoint).toBeNull();
  d.preferences.model = "new";
  changeEndpoint(d, "https://another.example");
  expect(d.preferences.model).toBeNull();
  changeHarness(d, "codex");
  expect(d.options.provider).toBeNull();
  expect(d.preferences.mode).toBeNull();
});
test("launch changes cannot configure a session using the previous provider", () => {
  const old = draft(),
    next = draft();
  expect(sameLaunchOptions(old, next)).toBe(true);
  next.options.provider = "anthropic";
  expect(sameLaunchOptions(old, next)).toBe(false);
});
test("preview uses a read-only POST with unsaved references and ignores obsolete responses", async () => {
  let finish: ((r: Response) => void) | undefined;
  const fetcher = vi
    .fn()
    .mockImplementationOnce(() => new Promise<Response>((r) => (finish = r)))
    .mockResolvedValue(
      Response.json({
        runner: "local",
        status: "ready",
        facts: { provider: "anthropic" },
      }),
    );
  const client = new LocalClient("", fetcher),
    catalog = new AgentModelCatalog(client),
    d = draft();
  d.options.environment = {
    ANTHROPIC_API_KEY: { source: "keychain", value: "draft-ref" },
  };
  const first = catalog.preview(
    "editor",
    d,
    { "draft-ref": "transient-secret", unrelated: "not-forwarded" },
    "1",
  );
  changeProvider(d, "anthropic");
  await catalog.preview("editor", d, { "draft-ref": "transient-secret" }, "1");
  finish!(Response.json({ runner: "local", facts: { provider: "old" } }));
  await first;
  expect(catalog.values.editor.facts).toEqual({ provider: "anthropic" });
  expect(
    fetcher.mock.calls.every(([url]) => url === "/v1/local/agents/catalog"),
  ).toBe(true);
  const request = JSON.parse(fetcher.mock.calls[1][1].body);
  expect(request.options.provider).toBe("anthropic");
  expect(request.secrets).toEqual({ "draft-ref": "transient-secret" });
  expect(fetcher.mock.calls[0][1].signal.aborted).toBe(true);
  await catalog.preview("editor", d, { "draft-ref": "transient-secret" }, "1");
  expect(fetcher).toHaveBeenCalledTimes(2);
  catalog.cancelPreview("editor");
  expect(catalog.values.editor).toBeUndefined();
  catalog.destroy();
  client.close();
});
test("catalog refresh failure retains the draft and distinguishes empty responses", async () => {
  const fetcher = vi
    .fn()
    .mockResolvedValueOnce(
      Response.json({
        runner: "local",
        status: "empty",
        facts: { models: { availableModels: [] } },
      }),
    )
    .mockRejectedValueOnce(new Error("offline"));
  const client = new LocalClient("", fetcher),
    catalog = new AgentModelCatalog(client),
    d = draft();
  await catalog.preview("editor", d, {}, "1");
  expect(catalog.values.editor.status).toBe("empty");
  expect(d.preferences.model).toBe("old");
  await catalog.preview("editor", d, {}, "1", true);
  expect(catalog.errors.editor).toContain("could not be loaded");
  expect(d.preferences.model).toBe("old");
  catalog.destroy();
  client.close();
});

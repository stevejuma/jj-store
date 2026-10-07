import { expect, test, vi } from "vitest";
import { AgentModelCatalog } from "../../src/lib/controller/agent-model-catalog.svelte.js";
import { LocalClient } from "../../src/lib/api/local-client.js";

test("editor discovery is a cached GET and never sends work", async () => {
  const fetcher = vi.fn(async () =>
    Response.json({
      runner: "codex",
      facts: { models: { availableModels: [{ modelId: "advertised" }] } },
    }),
  );
  const client = new LocalClient("", fetcher),
    catalog = new AgentModelCatalog(client);
  try {
    await catalog.load("codex");
    await catalog.load("codex");
    expect(fetcher).toHaveBeenCalledTimes(1);
    expect(fetcher.mock.calls[0][0]).toBe(
      "/v1/local/agents/catalog?runner=codex",
    );
    expect(fetcher.mock.calls[0][1].body).toBeUndefined();
    expect(catalog.values.codex.facts).toMatchObject({
      models: { availableModels: [{ modelId: "advertised" }] },
    });
  } finally {
    catalog.destroy();
    client.close();
  }
});
test("failed model reads require explicit retry and then recover", async () => {
  const fetcher = vi
    .fn()
    .mockRejectedValueOnce(new TypeError("Offline"))
    .mockResolvedValue(Response.json({ runner: "codex", facts: {} }));
  const client = new LocalClient("", fetcher),
    catalog = new AgentModelCatalog(client);
  try {
    await catalog.load("codex");
    await catalog.load("codex");
    expect(fetcher).toHaveBeenCalledTimes(1);
    expect(catalog.errors.codex).toContain("Models could not be loaded");
    expect(catalog.loading.codex).toBe(false);
    await catalog.load("codex", true);
    expect(fetcher).toHaveBeenCalledTimes(2);
    expect(catalog.errors.codex).toBe("");
  } finally {
    catalog.destroy();
    client.close();
  }
});

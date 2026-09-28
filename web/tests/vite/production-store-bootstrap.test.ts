import { expect, test } from "vitest";
import { createProductionStoreController } from "../../src/lib/controller/production-store-controller.svelte.js";

const repository = "ab".repeat(16);

function response(value: unknown) {
  return new Response(JSON.stringify(value), {
    headers: { "content-type": "application/json" },
  });
}

async function ready(predicate: () => boolean) {
  for (let attempt = 0; attempt < 100; attempt += 1) {
    if (predicate()) return;
    await new Promise((resolve) => setTimeout(resolve, 0));
  }
  throw new Error("controller did not settle");
}

test("LocalTrusted bootstrap auto-opens the sole catalog repository without a token", async () => {
  const headers: Headers[] = [];
  const fetcher: typeof fetch = async (input, init) => {
    headers.push(new Headers(init?.headers));
    const path = String(input);
    if (path === "/v1/bootstrap")
      return response({
        version: 1,
        access_mode: "local_trusted",
        storage: { ready: true },
        repositories: [{ id: repository, name: "Demo repository" }],
        backups: [],
        capabilities: {},
      });
    if (path.endsWith("/capabilities"))
      return response({
        typed_mutations: [],
        conflict_resolution_actions: [],
        conflict_upload_bytes: 0,
        search: false,
      });
    if (path.endsWith("/snapshot"))
      return response({
        repository,
        epoch: "cd".repeat(16),
        position: 0,
        revision: 0,
        heads: [],
      });
    if (path.includes("/events"))
      return new Response("", {
        headers: { "content-type": "text/event-stream" },
      });
    throw new Error(`unexpected request ${path}`);
  };
  const controller = createProductionStoreController({ fetcher });
  await ready(() => controller.state.status.connection === "connected");

  expect(controller.state.accessMode).toBe("local_trusted");
  expect(controller.state.repository).toEqual({
    id: repository,
    name: "Demo repository",
  });
  expect(controller.state.member.name).toBe("Local");
  expect(controller.state.member.status).toContain("trusted local access");
  for (const requestHeaders of headers) {
    expect(requestHeaders.get("x-jj-store-protocol")).toBe("1");
    expect(requestHeaders.has("authorization")).toBe(false);
  }
  controller.actions.destroy();
});

test("LocalTrusted bootstrap exposes a chooser for several repositories", async () => {
  const second = "cd".repeat(16);
  const controller = createProductionStoreController({
    fetcher: async () =>
      response({
        version: 1,
        access_mode: "local_trusted",
        storage: { ready: true },
        repositories: [
          { id: repository, name: "One" },
          { id: second, name: "Two" },
        ],
        backups: [],
        capabilities: {},
      }),
  });
  await ready(() => controller.state.accessMode === "local_trusted");

  expect(controller.state.status.connection).toBe("disconnected");
  expect(controller.state.repositories.map(({ name }) => name)).toEqual([
    "One",
    "Two",
  ]);
  controller.actions.destroy();
});

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
        backups: [],
        capabilities: {},
      });
    if (path.startsWith("/v1/repositories?"))
      return response({
        version: 1,
        generation: 1,
        repositories: [
          { id: repository, name: "Demo repository", permission: "admin" },
        ],
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
    permission: "admin",
  });
  expect(controller.state.member.name).toBe("Local");
  expect(controller.state.member.status).toContain("trusted local access");
  for (const requestHeaders of headers) {
    expect(requestHeaders.get("x-jj-store-protocol")).toBe("1");
    expect(requestHeaders.has("authorization")).toBe(false);
  }
  controller.actions.destroy();
});

test("LocalTrusted bootstrap exposes Projects for several repositories", async () => {
  const second = "cd".repeat(16);
  const controller = createProductionStoreController({
    fetcher: async (input) => {
      const path = String(input);
      if (path === "/v1/bootstrap")
        return response({
          version: 1,
          access_mode: "local_trusted",
          storage: { ready: true },
          backups: [],
          capabilities: {},
        });
      if (path.startsWith("/v1/repositories?"))
        return response({
          version: 1,
          generation: 1,
          repositories: [
            { id: repository, name: "One", permission: "admin" },
            { id: second, name: "Two", permission: "admin" },
          ],
        });
      if (path.includes("/work/browse"))
        return response({
          version: 1,
          snapshot: {
            repo_id: path.includes(repository) ? repository : second,
            epoch: "11".repeat(16),
            manifest_revision: 0,
            published_seq: 0,
            work_root: null,
            work_state_version: 0,
          },
          projects: [],
          tasks: [],
          total_matches: 0,
          next_cursor: null,
        });
      throw new Error(`unexpected request ${path}`);
    },
  });
  await ready(
    () =>
      controller.state.accessMode === "local_trusted" &&
      !controller.projects!.loading,
  );

  expect(controller.state.status.connection).toBe("disconnected");
  expect(controller.state.installationReady).toBe(true);
  expect(controller.state.route.page).toBe("projects");
  expect(controller.projects!.homes).toHaveLength(2);
  expect(controller.projects!.homes.every((h) => h.error === "")).toBe(true);
  expect(controller.state.repositories.map(({ name }) => name)).toEqual([
    "One",
    "Two",
  ]);
  controller.actions.destroy();
});

test("authenticated bootstrap accepts one token and exposes every authorized repository", async () => {
  const second = "cd".repeat(16);
  const token = "manual-token";
  const seen: { path: string; headers: Headers }[] = [];
  const controller = createProductionStoreController({
    fetcher: async (input, init) => {
      const path = String(input);
      seen.push({ path, headers: new Headers(init?.headers) });
      if (path === "/v1/bootstrap")
        return response({
          version: 1,
          access_mode: "authenticated",
          storage: { ready: true },
          backups: [],
          capabilities: {},
        });
      if (path.startsWith("/v1/repositories?"))
        return response({
          version: 1,
          generation: 1,
          repositories: [
            { id: repository, name: "lapis/one", permission: "read" },
            { id: second, name: "lapis/two", permission: "write" },
          ],
        });
      if (path === `/v1/repos/${second}/capabilities`)
        return response({
          typed_mutations: [],
          conflict_resolution_actions: [],
          conflict_upload_bytes: 0,
          search: false,
        });
      if (path === `/v1/repos/${second}/snapshot`)
        return response({
          repository: second,
          epoch: "ef".repeat(16),
          position: 0,
          revision: 0,
          heads: [],
        });
      if (path.startsWith(`/v1/repos/${second}/events`))
        return new Response("", {
          headers: { "content-type": "text/event-stream" },
        });
      throw new Error(`unexpected request ${path}`);
    },
  });
  await ready(() => controller.state.accessMode === "authenticated");
  await controller.actions.connect({ repository: "", token });
  expect(controller.state.status.connection).toBe("disconnected");
  expect(controller.state.repositories.map(({ name }) => name)).toEqual([
    "lapis/one",
    "lapis/two",
  ]);
  await controller.actions.openRepository(second);
  await ready(() => controller.state.status.connection === "connected");
  expect(controller.state.repository).toEqual({
    id: second,
    name: "lapis/two",
    permission: "write",
  });
  for (const request of seen.filter(({ path }) => path !== "/v1/bootstrap"))
    expect(request.headers.get("authorization")).toBe(`Bearer ${token}`);
  controller.actions.destroy();
});

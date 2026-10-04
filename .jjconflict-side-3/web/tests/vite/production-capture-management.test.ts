import { expect, test } from "vitest";
import { pretty } from "../../src/lib/api/client.js";
import { createProductionStoreController } from "../../src/lib/controller/production-store-controller.svelte.js";
import {
  captureSources,
  captureStatus,
} from "../../../catalog/fixtures/capture-management.js";

async function settle(predicate: () => boolean) {
  for (let n = 0; n < 100; n++) {
    if (predicate()) return;
    await new Promise((resolve) => setTimeout(resolve, 0));
  }
  throw new Error("Bootstrap did not settle");
}
function setup(access: "local_trusted" | "authenticated", advertised: boolean) {
  const paths: string[] = [];
  let closed: AbortSignal | null = null;
  const controller = createProductionStoreController({
    fetcher: async (input, init) => {
      const path = String(input);
      paths.push(path);
      let value: unknown;
      if (path === "/v1/bootstrap")
        value = {
          version: 1,
          access_mode: access,
          storage: { ready: true },
          backups: [],
          capabilities: { local_capture_management: advertised },
        };
      else if (path.startsWith("/v1/repositories?"))
        value = {
          version: 1,
          generation: 1,
          repositories: [],
          next_cursor: null,
        };
      else if (path === "/v1/local/capture") {
        closed = init!.signal!;
        value = captureStatus();
      } else if (path.startsWith("/v1/local/capture/sources?"))
        value = captureSources();
      else if (path === "/v1/local/capture/control")
        throw new TypeError("lost response");
      else throw new Error(`Unexpected request ${path}`);
      return new Response(pretty(value), {
        headers: { "content-type": "application/json" },
      });
    },
  });
  return { controller, paths, closed: () => closed };
}
test("installation capture survives repository disconnect", async () => {
  const f = setup("local_trusted", true);
  await settle(() => f.controller.state.accessMode === "local_trusted");
  expect(f.controller.capture!.supported).toBe(true);
  await f.controller.capture!.refresh();
  expect(f.controller.capture!.state.status).toBe("ready");
  await f.controller.capture!.control("pause");
  const original = pretty(f.controller.capture!.state.pending);
  expect(f.controller.capture!.state.pending!.uncertain).toBe(true);
  f.controller.actions.disconnect();
  expect(pretty(f.controller.capture!.state.pending)).toBe(original);
  expect(f.controller.capture!.supported).toBe(true);
  f.controller.actions.destroy();
  expect(f.controller.capture!.supported).toBe(false);
  expect(f.closed()!.aborted).toBe(true);
});
test.each([
  ["local_trusted", false],
  ["authenticated", true],
] as const)(
  "capture requires local bootstrap mode and capability (%s, %s)",
  async (mode, capability) => {
    const f = setup(mode, capability);
    await settle(() => f.controller.state.accessMode === mode);
    await f.controller.capture!.refresh();
    expect(
      f.paths.filter((path) => path.startsWith("/v1/local/capture")),
    ).toEqual([]);
    expect(f.controller.capture!.supported).toBe(false);
    f.controller.actions.destroy();
  },
);

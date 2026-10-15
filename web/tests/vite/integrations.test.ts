import { expect, it, vi } from "vitest";
import {
  Integrations,
  type IntegrationInventory,
} from "../../src/lib/controller/integrations.svelte.js";
import { LocalClient } from "../../src/lib/api/local-client.js";
import { NotificationService } from "../../src/lib/notifications/notification-service.svelte.js";
import { ForgeController } from "../../src/lib/controller/forge/controller.svelte.js";
import { workspaceForgeTransport } from "../../../catalog/fixtures/forge/workspace-transport.js";
import { createStoreRouteHistory } from "../../src/lib/controller/store-route-history.svelte.js";

it("failed account setup retains its transient draft and publishes one actionable failure", async () => {
  const service = new NotificationService();
  const manager = new Integrations(
    () => true,
    new LocalClient(
      "",
      async () =>
        new Response(JSON.stringify({ code: "forge_access_denied" }), {
          status: 403,
          headers: { "content-type": "application/json" },
        }),
    ),
  );
  manager.notifications = { service, scope: "application" };
  manager.flow = "pat";
  manager.token = "do-not-persist";
  expect(
    await manager.command({
      kind: "add_token",
      api_url: "https://api.github.com",
      token: manager.token,
    }),
  ).toBe(false);
  expect(manager.token).toBe("do-not-persist");
  expect(service.records).toHaveLength(1);
  expect(service.records[0].title).toBe("Integration action failed");
  expect(JSON.stringify(service.records)).not.toContain("do-not-persist");
  manager.destroy();
  service.dispose();
});
it("account setup success clears only the secret draft and refreshes repository access", async () => {
  const inventory: IntegrationInventory = {
    accounts: [],
    authorization: null,
    oauth_available: true,
    keychain_supported: true,
  };
  const manager = new Integrations(
    () => true,
    new LocalClient(
      "",
      async () =>
        new Response(JSON.stringify(inventory), {
          headers: { "content-type": "application/json" },
        }),
    ),
  );
  manager.flow = "pat";
  manager.token = "transient-secret";
  manager.changed = vi.fn();
  expect(
    await manager.command({
      kind: "add_token",
      api_url: "https://api.github.com",
      token: manager.token,
    }),
  ).toBe(true);
  expect(manager.token).toBe("");
  expect(manager.flow).toBe("");
  expect(manager.changed).toHaveBeenCalledOnce();
  manager.destroy();
});
it("detection notices survive refresh dismissal and open Integrations with an exact workspace return", async () => {
  const fixture = workspaceForgeTransport();
  const h = createStoreRouteHistory();
  h.stacks.enter();
  const entry = { ...h.route };
  const configure = () => h.globalSettings.open("integrations");
  const service = new NotificationService();
  const forge = new ForgeController(
    async (request) =>
      request.action.kind === "detect"
        ? {
            ...fixture.initial,
            detection: {
              provider: "github",
              remotes: [],
              accounts: [],
              account_id: null,
              readiness: "unconfigured",
            },
          }
        : fixture.transport(request),
    undefined,
    undefined,
    { managed: true, scope: "repo", openSettings: configure },
  );
  forge.notifications = { service, scope: "application" };
  expect(await forge.connection.discover()).toBe(false);
  expect(forge.ready).toBe(false);
  expect(forge.pr("feature/api")).toBeUndefined();
  const notice = service.records[0]!;
  service.invoke(notice.id, notice.action!.id);
  expect(h.route.globalSettings).toEqual({
    section: "integrations",
    tab: "accounts",
  });
  h.globalSettings.leave();
  expect(h.route).toEqual(entry);
  service.dismiss(notice.id);
  await forge.connection.discover();
  expect(service.records).toHaveLength(0);
  forge.close();
  h.destroy();
  service.dispose();
});

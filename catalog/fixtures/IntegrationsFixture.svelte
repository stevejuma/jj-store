<script lang="ts">
  import { onDestroy, untrack } from "svelte";
  import StoreRepositoryApplication from "../../web/src/lib/ui/store-application/StoreRepositoryApplication.svelte";
  import { createInMemoryStoreController } from "../controller/in-memory-store-controller.svelte.js";
  import {
    Integrations,
    type IntegrationInventory,
  } from "../../web/src/lib/controller/integrations.svelte.js";
  import { LocalClient } from "../../web/src/lib/api/local-client.js";
  import { createMemoryStoryStorage } from "./memory-storage.js";
  import { ownAppearance } from "../../web/src/lib/appearance/runtime.svelte.js";
  import { createAppearanceStore } from "../../web/src/lib/appearance/store.svelte.js";
  let {
    connected = false,
    oauth = true,
    colorMode,
  }: {
    connected?: boolean;
    oauth?: boolean;
    colorMode?: "light" | "dark";
  } = $props();
  const setup = untrack(() => {
    const base = createInMemoryStoreController("stacks");
    base.actions.openGlobalSettings("integrations");
    let inventory: IntegrationInventory = {
      accounts: connected
        ? [
            {
              id: "account-1",
              api_url: "https://api.github.com",
              user_id: "42",
              login: "stevejuma",
              name: "Steve",
              email: "steve@example.test",
              avatar_url: null,
              method: "pat",
              revision: 1,
              available: true,
            },
          ]
        : [],
      oauth_available: oauth,
      keychain_supported: true,
      authorization: null,
    };
    const fetcher: typeof fetch = async (_url, init) => {
      const command = init?.body ? JSON.parse(String(init.body)) : undefined;
      if (command?.kind === "add_token") {
        if (command.token === "bad-token")
          return new Response(JSON.stringify({ code: "forge_access_denied" }), {
            status: 403,
            headers: { "content-type": "application/json" },
          });
        inventory = {
          ...inventory,
          accounts: [
            ...inventory.accounts,
            {
              id: `account-${inventory.accounts.length + 1}`,
              api_url: command.api_url,
              user_id: "43",
              login: "octocat",
              name: "Octocat",
              email: null,
              avatar_url: null,
              method:
                command.api_url === "https://api.github.com"
                  ? "pat"
                  : "enterprise",
              revision: 1,
              available: true,
            },
          ],
        };
      }
      if (command?.kind === "forget")
        inventory = {
          ...inventory,
          accounts: inventory.accounts.filter((a) => a.id !== command.account),
        };
      if (command?.kind === "begin_oauth")
        inventory = {
          ...inventory,
          authorization: {
            session: "public-session-handle",
            user_code: "ABCD-1234",
            verification_uri: "https://github.com/login/device",
            expires_at: 9999999999,
            interval: 30,
            state: "pending",
          },
        };
      if (command?.kind === "cancel_oauth")
        inventory = { ...inventory, authorization: null };
      return new Response(JSON.stringify(inventory), {
        headers: { "content-type": "application/json" },
      });
    };
    const manager = new Integrations(() => true, new LocalClient("", fetcher));
    return {
      controller: Object.assign(base, { integrations: manager }),
      manager,
    };
  });
  const storage = untrack(createMemoryStoryStorage);
  const requestedColorMode = untrack(() => colorMode);
  if (requestedColorMode) {
    const appearance = createAppearanceStore(storage);
    appearance.mode(requestedColorMode);
    ownAppearance(appearance, storage);
  }
  onDestroy(() => {
    setup.manager.destroy();
    setup.controller.actions.destroy();
  });
</script>

<div style="height:100vh;min-height:32rem">
  <StoreRepositoryApplication
    controller={setup.controller}
    layoutStorage={storage}
    displayMode="desktop"
  />
</div>

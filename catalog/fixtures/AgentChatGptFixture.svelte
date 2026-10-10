<script lang="ts">
  import { onDestroy, untrack } from "svelte";
  import { Button } from "@lapismd/design-core/shadcn/button";
  import StoreAgentChatGptAccounts from "../../web/src/lib/ui/store-settings/StoreAgentChatGptAccounts.svelte";
  import {
    AgentChatGpt,
    type ChatGptStatus,
  } from "../../web/src/lib/controller/agent-chatgpt.svelte.js";
  import { LocalClient } from "../../web/src/lib/api/local-client.js";
  import "../../web/src/lib/ui/store-settings/StoreSettings.css";
  let { unsupported = false }: { unsupported?: boolean } = $props();
  let status: ChatGptStatus = untrack(() => ({
    supported: !unsupported,
    accounts: [
      {
        id: "account-one",
        label: "Personal workspace",
        email: "fixture@example.invalid",
        connected: true,
        plan_enabled: true,
      },
      {
        id: "account-two",
        label: "Team workspace",
        email: "fixture@example.invalid",
        connected: false,
        plan_enabled: false,
      },
    ],
    attempts: [],
  }));
  let commands = $state<string[]>([]);
  let next = 0;
  const controller = new AgentChatGpt(
    new LocalClient("", async (_, init) => {
      if (init?.body) {
        const command = JSON.parse(String(init.body));
        commands = [...commands, command.action];
        if (command.action === "connect")
          status.attempts = [
            {
              id: command.request_id,
              account: command.account ?? `new-${next++}`,
              state: "authorizing",
              error: null,
            },
          ];
        if (command.action === "cancel")
          status.attempts = status.attempts.map((a) =>
            a.id === command.attempt ? { ...a, state: "cancelled" } : a,
          );
        if (command.action === "sign_out")
          status.accounts = status.accounts.map((a) =>
            a.id === command.account ? { ...a, connected: false } : a,
          );
      }
      return new Response(JSON.stringify(status), {
        headers: { "content-type": "application/json" },
      });
    }),
    () => {},
  );
  onDestroy(() => controller.dispose());
  function complete() {
    for (const attempt of status.attempts)
      if (attempt.state === "authorizing") {
        attempt.state = "connected";
        const account = status.accounts.find((a) => a.id === attempt.account);
        if (account) {
          account.connected = true;
          account.plan_enabled = true;
        } else
          status.accounts.push({
            id: attempt.account,
            label: "Additional workspace",
            email: "second@example.invalid",
            connected: true,
            plan_enabled: true,
          });
      }
    void controller.refresh();
  }
</script>

<StoreAgentChatGptAccounts accounts={controller} />
<div class="fixture-controls" aria-label="Fixture controls">
  <Button variant="outline" onclick={complete}>Complete fixture sign-in</Button>
  <output aria-label="Account commands">{commands.join(", ")}</output>
</div>

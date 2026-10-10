<script lang="ts">
  import { onMount } from "svelte";
  import StoreWorkField from "../store-work/StoreWorkField.svelte";
  import StoreSettingsSelect from "../store-settings/StoreSettingsSelect.svelte";
  import type { AgentExecution } from "../../controller/agent-execution.svelte.js";
  import type { AgentConfiguration } from "../../controller/agent-configuration.js";
  import {
    changeAccount,
    canonicalProvider,
  } from "../../controller/agent-configuration.js";
  let {
    manager,
    draft = $bindable(),
    pending = false,
  }: {
    manager: AgentExecution;
    draft: AgentConfiguration;
    pending?: boolean;
  } = $props();
  const id = $props.id();
  onMount(() => {
    void manager.chatgpt.refresh();
  });
  const inherited = $derived(
    draft.options.inherit_defaults
      ? manager.settings.value?.options.account
      : null,
  );
  const selected = $derived(draft.options.account ?? inherited);
  const account = $derived(
    manager.chatgpt.value?.accounts.find((a) => a.id === selected),
  );
</script>

<StoreWorkField
  label="ChatGPT account"
  {id}
  description={account?.connected
    ? account.plan_enabled
      ? "Using your ChatGPT plan."
      : "Reconnect in Settings → Agents → Defaults to allow plan usage."
    : "Connect an account in Settings → Agents → Defaults."}
>
  <StoreSettingsSelect
    {id}
    value={draft.options.account ?? null}
    disabled={pending}
    options={[
      {
        value: null,
        label: inherited
          ? "Application default account"
          : "Choose a connected account",
      },
      ...(manager.chatgpt.value?.accounts ?? []).map((a) => ({
        value: a.id,
        label: `${a.email ?? a.label} · ${a.id.slice(0, 8)}${a.connected ? "" : " (signed out)"}`,
      })),
    ]}
    onValueChange={(value) => changeAccount(draft, value)}
  />
</StoreWorkField>

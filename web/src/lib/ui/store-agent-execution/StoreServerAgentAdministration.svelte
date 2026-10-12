<script lang="ts">
  import { DialogFrame } from "@lapismd/design-core/forms/dialog";
  import * as Dialog from "@lapismd/design-core/shadcn/dialog";
  import * as Tabs from "@lapismd/design-core/shadcn/tabs";
  import { Button } from "@lapismd/design-core/shadcn/button";
  import StoreNotice from "../store-notifications/StoreNotice.svelte";
  import { provideSettingsPresentation } from "../store-settings/settings-presentation.js";
  import StoreServerAgentConfiguration from "./StoreServerAgentConfiguration.svelte";
  import StoreServerAccessOnboarding from "./StoreServerAccessOnboarding.svelte";
  import type { ServerAgentAdministration } from "../../controller/server-agent-administration.svelte.js";
  let {
    manager,
    repositories,
  }: {
    manager: ServerAgentAdministration;
    repositories: readonly {
      id: string;
      name: string;
      permission: string | null;
      profile?: string;
    }[];
  } = $props();
  let tab = $state("configuration");
  const disabled = $derived(manager.busy || !!manager.pending);
  provideSettingsPresentation({ value: "global" });
  $effect(() => {
    if (manager.open) {
      if (manager.access && !manager.access.agents) tab = "access";
      void manager.refresh();
    }
  });
</script>

<Dialog.Root
  open={manager.open}
  onOpenChange={(open) => {
    if (!disabled) manager.open = open;
  }}
>
  <DialogFrame
    title="Server agent access"
    closeLabel="Close server agent access"
    busy={disabled}
    aria-describedby={undefined}
  >
    <div class="server-access store-settings--global">
      <StoreNotice
        noticeKey="server-agent-administration"
        title="Server access"
        message={manager.error?.replaceAll("_", " ")}
      />
      <StoreNotice
        noticeKey="server-agent-administration-success"
        title="Server access updated"
        message={manager.notice}
        severity="info"
        lifetime="timed"
      />
      {#if manager.pending}<div class="server-access-recovery">
          <p>Keep this connection until the original outcome is resolved.</p>
          <Button
            variant="outline"
            disabled={manager.busy}
            onclick={() => void manager.checkOutcome()}
            >Check original outcome</Button
          >
        </div>{/if}
      {#if manager.access?.agents}
        <Tabs.Root bind:value={tab}>
          <Tabs.List
            ><Tabs.Trigger value="configuration"
              >Configuration and sharing</Tabs.Trigger
            ><Tabs.Trigger value="access">Access onboarding</Tabs.Trigger
            ></Tabs.List
          >
          <Tabs.Content value="configuration" hidden={tab !== "configuration"}
            ><StoreServerAgentConfiguration {manager} /></Tabs.Content
          >
          <Tabs.Content value="access" hidden={tab !== "access"}
            ><StoreServerAccessOnboarding
              {manager}
              {repositories}
            /></Tabs.Content
          >
        </Tabs.Root>
      {:else}<StoreServerAccessOnboarding {manager} {repositories} />{/if}
    </div>
  </DialogFrame>
</Dialog.Root>

<style>
  .server-access {
    display: flex;
    flex-direction: column;
    gap: 1rem;
    min-width: 0;
  }
  .server-access :global(.server-access-form),
  .server-access :global([data-slot="tabs-content"]:not([hidden])) {
    display: flex;
    flex-direction: column;
    gap: 1rem;
  }
  .server-access :global(.store-settings__section) {
    margin-block-start: 0;
  }
  .server-access :global(input),
  .server-access :global(textarea) {
    width: min(20rem, 100%);
  }
  .server-access :global(.store-settings__actions) {
    display: flex;
    gap: 0.5rem;
    flex-wrap: wrap;
  }
  .server-access :global(.server-access-help) {
    padding: 1rem;
    color: var(--muted-foreground);
  }
  .server-access-recovery {
    display: flex;
    align-items: center;
    justify-content: space-between;
    flex-wrap: wrap;
    gap: 1rem;
  }
</style>

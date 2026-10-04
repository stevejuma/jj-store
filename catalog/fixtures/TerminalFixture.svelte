<script lang="ts">
  import { provideTerminalControls } from "../../web/src/lib/terminal/controls.svelte.js";
  import StoreTerminalToggle from "../../web/src/lib/ui/store-terminal/StoreTerminalToggle.svelte";
  provideTerminalControls();
  import StoreTerminalWorkspace from "../../web/src/lib/ui/store-terminal/StoreTerminalWorkspace.svelte";
  import StoreNotificationHost from "../../web/src/lib/ui/store-notifications/StoreNotificationHost.svelte";
  import { ownNotifications } from "../../web/src/lib/notifications/notification-context.svelte.js";
  import { FakeTerminalBridge } from "./terminal-bridge.js";
  import { untrack } from "svelte";
  let {
    folders = 1,
    inputProbe = false,
  }: { folders?: number; inputProbe?: boolean } = $props();
  const bridge = untrack(() => new FakeTerminalBridge(folders));
  let received = $state<string[]>([]);
  const send = bridge.input.bind(bridge);
  bridge.input = async (id, data) => {
    received = [...received, data];
    await send(id, data);
  };
  const notifications = ownNotifications();
  let repository = $state("repo-a");
  let draft = $state("");
  let linked = $state(false);
  let detailsOpen = $state(true);
  const layout = { onMaximize: () => (detailsOpen = false) };
</script>

<div
  style="height: 650px; display: flex; flex-direction: column; color: var(--foreground); background: var(--background)"
>
  <nav aria-label="Fixture repositories">
    <button onclick={() => (repository = "repo-a")}>Repository A</button><button
      onclick={() => (repository = "repo-b")}>Repository B</button
    >
    <StoreTerminalToggle />
  </nav>
  {#if detailsOpen}<aside aria-label="Details sidebar">
      Repository details
    </aside>{/if}
  <StoreTerminalWorkspace
    {repository}
    {bridge}
    {...layout}
    onLink={() => (linked = true)}
  >
    <div style="padding: 1rem">
      <h1>{repository}</h1>
      <label>Retained draft <textarea bind:value={draft}></textarea></label
      >{#if linked}<p role="status">Folder settings opened</p>{/if}
    </div>
  </StoreTerminalWorkspace>
</div>
{#if inputProbe}<output aria-label="Received terminal input"
    >{JSON.stringify(received)}</output
  >{/if}
<StoreNotificationHost owner={notifications} />

<script lang="ts">
  import { ScrollArea } from "@lapismd/design-core/shadcn/scroll-area";
  import { Button } from "@lapismd/design-core/shadcn/button";
  import PlusIcon from "@lucide/svelte/icons/plus";
  import MonitorIcon from "@lucide/svelte/icons/monitor";
  import SettingsIcon from "@lucide/svelte/icons/settings";
  import { serverConnections } from "../../connections/context.js";
  import type { ServerSwitcherOwner } from "./presentation.js";
  let {
    compact = false,
    owner = serverConnections(),
  }: { compact?: boolean; owner?: ServerSwitcherOwner } = $props();
</script>

{#if owner?.supported}
  <nav class="server-switcher" class:compact aria-label="Servers">
    <ScrollArea class="server-switcher__scroll"
      ><div class="server-switcher__list">
        {#each [{ id: "local", name: "Local" }, ...owner.records] as server (server.id)}
          <div
            class="server-switcher__row"
            data-selected={owner.selected === server.id}
          >
            <Button
              variant={owner.selected === server.id ? "secondary" : "ghost"}
              size={compact ? "icon" : "default"}
              aria-label={`Select ${server.name} server`}
              aria-pressed={owner.selected === server.id}
              title={server.name}
              onclick={() => owner.select(server.id)}
              class="server-switcher__button"
            >
              <span class="server-switcher__initial" aria-hidden="true"
                >{#if server.id === "local"}<MonitorIcon />{:else}{server.name
                    .trim()
                    .slice(0, 1)
                    .toUpperCase()}{/if}</span
              >
              {#if !compact}<span class="server-switcher__name"
                  >{server.name}</span
                >{/if}
            </Button>
            {#if !compact && server.id !== "local"}<Button
                variant="ghost"
                size="icon-sm"
                title={`Manage ${server.name}`}
                aria-label={`Manage ${server.name} connection`}
                onclick={() => owner.manage(server.id)}
                ><SettingsIcon aria-hidden="true" /></Button
              >{/if}
          </div>
        {/each}
        <Button
          variant="ghost"
          size={compact ? "icon" : "default"}
          aria-label="Add server"
          title="Add server"
          onclick={() => owner.add()}
          class="server-switcher__button"
          ><PlusIcon aria-hidden="true" />{#if !compact}Add server{/if}</Button
        >
        {#if compact && owner.selected !== "local"}<Button
            variant="ghost"
            size="icon-sm"
            aria-label="Manage selected server"
            title="Manage selected server"
            onclick={() => owner.manage(owner.selected)}
            ><SettingsIcon aria-hidden="true" /></Button
          >{/if}
      </div></ScrollArea
    >
  </nav>
{/if}

<style>
  .server-switcher {
    display: flex;
    min-width: 0;
    padding: 0.5rem;
    border-block-start: 1px solid var(--border);
  }
  :global(.server-switcher__scroll) {
    width: 100%;
    max-height: 18rem;
  }
  .server-switcher__list {
    display: flex;
    flex-direction: column;
    gap: 0.375rem;
    min-width: 0;
    width: 100%;
  }
  .compact .server-switcher__list {
    align-items: center;
  }
  .server-switcher__row {
    position: relative;
    display: flex;
    align-items: center;
    gap: 0.25rem;
    min-width: 0;
  }
  .server-switcher__row[data-selected="true"]::before {
    content: "";
    position: absolute;
    inset-inline-start: -0.5rem;
    width: 0.25rem;
    height: 1.5rem;
    border-radius: 0.25rem;
    background: var(--primary);
  }
  :global(.server-switcher__button) {
    justify-content: flex-start;
    min-width: 0;
    flex: 1;
  }
  .compact :global(.server-switcher__button) {
    flex: none;
    justify-content: center;
  }
  .server-switcher__initial {
    display: inline-flex;
    align-items: center;
    justify-content: center;
    width: 1.5rem;
    flex-shrink: 0;
    font-weight: 600;
  }
  .server-switcher__name {
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }
</style>

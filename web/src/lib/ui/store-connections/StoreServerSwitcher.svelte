<script lang="ts">
  import { Button } from "@lapismd/design-core/shadcn/button";
  import * as DropdownMenu from "@lapismd/design-core/shadcn/dropdown-menu";
  import PlusIcon from "@lucide/svelte/icons/plus";
  import MonitorIcon from "@lucide/svelte/icons/monitor";
  import SettingsIcon from "@lucide/svelte/icons/settings";
  import EllipsisIcon from "@lucide/svelte/icons/ellipsis";
  import ServerIcon from "@lucide/svelte/icons/server";
  import "../community-navigation/CommunityNavigation.css";
  import { serverConnections } from "../../connections/context.js";
  import type { ServerSwitcherOwner } from "./presentation.js";
  let {
    compact = false,
    owner = serverConnections(),
  }: { compact?: boolean; owner?: ServerSwitcherOwner } = $props();
  // Local stays available; the selected remote occupies the other visible slot.
  const remote = $derived(
    owner?.records.find((server) => server.id === owner.selected) ??
      owner?.records[0],
  );
  const visible = $derived([
    { id: "local", name: "Local" },
    ...(remote ? [remote] : []),
  ]);
  const overflow = $derived(
    owner?.records.filter((server) => server.id !== remote?.id) ?? [],
  );
  let moreButton = $state<HTMLButtonElement | null>(null);
</script>

{#if owner?.supported}
  <nav class="server-switcher" class:compact aria-label="Servers">
    <div class="server-switcher__list">
      {#each visible as server (server.id)}
        <div class="server-switcher__row">
          <Button
            variant="ghost"
            size={compact ? "icon-sm" : "default"}
            data-active={owner.selected === server.id ? "" : undefined}
            aria-label={`Select ${server.name} server`}
            aria-pressed={owner.selected === server.id}
            title={server.name}
            onclick={() => owner.select(server.id)}
            class="lc-community-navigation__item server-switcher__button"
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
      {#if overflow.length}
        <DropdownMenu.Root>
          <DropdownMenu.Trigger>
            {#snippet child({ props })}
              <Button
                {...props}
                bind:ref={moreButton}
                variant="ghost"
                size={compact ? "icon-sm" : "default"}
                class="lc-community-navigation__item server-switcher__button"
                aria-label="More servers"
                title="More servers"
                ><EllipsisIcon aria-hidden="true" />{#if !compact}<span
                    >More servers</span
                  >{/if}</Button
              >
            {/snippet}
          </DropdownMenu.Trigger>
          <DropdownMenu.Content
            side="right"
            align="end"
            preventScroll={false}
            aria-label="More servers"
            style="width: 16rem; max-width: calc(100vw - 2rem)"
            onCloseAutoFocus={(event) => {
              if (owner.dialog) event.preventDefault();
            }}
          >
            <DropdownMenu.RadioGroup
              value={owner.selected}
              onValueChange={(id) => {
                moreButton?.focus();
                owner.select(id);
              }}
            >
              {#each overflow as server (server.id)}
                <DropdownMenu.RadioItem value={server.id}>
                  <ServerIcon aria-hidden="true" />
                  <span class="server-switcher__name">{server.name}</span>
                </DropdownMenu.RadioItem>
              {/each}
            </DropdownMenu.RadioGroup>
          </DropdownMenu.Content>
        </DropdownMenu.Root>
      {/if}
      <Button
        variant="ghost"
        size={compact ? "icon-sm" : "default"}
        aria-label="Add server"
        title="Add server"
        onclick={() => owner.add()}
        class="lc-community-navigation__item server-switcher__button"
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
    </div>
  </nav>
{/if}

<style>
  .server-switcher {
    display: flex;
    min-width: 0;
    padding: 0.5rem 0.75rem;
    border-block-start: 1px solid var(--border);
  }
  .server-switcher__list {
    display: flex;
    flex-direction: column;
    gap: 0.25rem;
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
  :global(.server-switcher__button) {
    justify-content: flex-start;
    min-width: 0;
    flex: none;
  }
  .server-switcher__row :global(.server-switcher__button) {
    flex: 1;
  }
  .compact :global(.server-switcher__button) {
    flex: none;
    width: 2.25rem;
    justify-content: center;
  }
  .server-switcher__initial {
    display: inline-flex;
    align-items: center;
    justify-content: center;
    width: 1rem;
    flex-shrink: 0;
    font-weight: 600;
  }
  .server-switcher__initial :global(svg) {
    width: 1rem;
    height: 1rem;
  }
  .server-switcher__name {
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }
</style>

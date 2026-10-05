<script lang="ts">
  import "./CommunityChannelSettings.css";
  import { AppShell } from "@lapismd/design-core/shell";
  import { Button } from "@lapismd/design-core/shadcn/button";
  import Hash from "@lucide/svelte/icons/hash";
  import StoreReadOnlyRow from "../store-settings/StoreReadOnlyRow.svelte";
  import Pencil from "@lucide/svelte/icons/pencil";
  import type { Snippet } from "svelte";
  let {
    title,
    id,
    onClose,
    onRename,
    onMembers,
    children,
    actions,
    heading = "Thread settings",
    closeLabel = "Close thread settings",
    identity,
    details,
  }: {
    title: string;
    id: string;
    onClose: () => void;
    onRename?: () => void;
    onMembers?: () => void;
    heading?: string;
    closeLabel?: string;
    identity?: Snippet;
    details?: Snippet;
    children?: Snippet;
    actions?: Snippet;
  } = $props();
</script>

<AppShell.Sidebar.Header class="lc-channel-settings__header"
  ><strong>{heading}</strong><AppShell.Sidebar.Close
    label={closeLabel}
    onclick={onClose}
  /></AppShell.Sidebar.Header
>
<AppShell.Sidebar.Body
  ><div class="lc-channel-settings">
    <div class="lc-channel-settings__identity">
      <span class="lc-channel-settings__image" aria-hidden="true"
        >{#if identity}{@render identity()}{:else}<Hash />{/if}</span
      ><strong>{title}</strong>
    </div>
    <section class="lc-channel-settings__details">
      <h2>Details</h2>
      {#if details}
        {@render details()}
        {#if children}{@render children()}{/if}
      {:else}
        <div class="lc-channel-settings__details-card">
          {#if onRename}<Button
              variant="ghost"
              class="lc-channel-settings__row"
              onclick={onRename}
              ><span><strong>Name</strong><small>{title}</small></span><Pencil
                data-icon="inline-end"
                aria-hidden="true"
              /></Button
            >{:else}<div class="lc-channel-settings__row">
              <span><strong>Name</strong><small>{title}</small></span>
            </div>{/if}
          {#if onMembers}<Button
              variant="ghost"
              class="lc-channel-settings__row"
              onclick={onMembers}
              ><span
                ><strong>Contributors</strong><small
                  >People visible in the loaded conversation</small
                ></span
              ></Button
            >
          {/if}
          <StoreReadOnlyRow
            copyable
            label="Thread ID"
            value={id}
            standalone
            stacked
            onOpen={onClose}
            openLabel="Open thread conversation"
          />
          {#if children}{@render children()}{/if}
        </div>
      {/if}
    </section>
    {#if actions}{@render actions()}{/if}
  </div></AppShell.Sidebar.Body
>

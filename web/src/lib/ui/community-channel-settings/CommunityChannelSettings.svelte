<script lang="ts">
  import "./CommunityChannelSettings.css";
  import { AppShell } from "@lapismd/design-core/shell";
  import { Button } from "@lapismd/design-core/shadcn/button";
  import Hash from "@lucide/svelte/icons/hash";
  import Copy from "@lucide/svelte/icons/copy";
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
  }: {
    title: string;
    id: string;
    onClose: () => void;
    onRename?: () => void;
    onMembers: () => void;
    children?: Snippet;
    actions?: Snippet;
  } = $props();
</script>

<AppShell.Sidebar.Header class="lc-channel-settings__header"
  ><strong>Thread settings</strong><AppShell.Sidebar.Close
    label="Close thread settings"
    onclick={onClose}
  /></AppShell.Sidebar.Header
>
<AppShell.Sidebar.Body
  ><div class="lc-channel-settings">
    <div class="lc-channel-settings__identity">
      <span class="lc-channel-settings__image" aria-hidden="true"><Hash /></span
      ><strong>{title}</strong>
    </div>
    <section class="lc-channel-settings__details">
      <h2>Details</h2>
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
        <Button
          variant="ghost"
          class="lc-channel-settings__row"
          onclick={onMembers}
          ><span
            ><strong>Contributors</strong><small
              >People visible in the loaded conversation</small
            ></span
          ></Button
        >
        <Button
          variant="ghost"
          class="lc-channel-settings__row"
          onclick={() => navigator.clipboard.writeText(id)}
          ><span><strong>Thread ID</strong><small>{id}</small></span><Copy
            data-icon="inline-end"
            aria-hidden="true"
          /></Button
        >
        {#if children}{@render children()}{/if}
      </div>
    </section>
    {#if actions}<section class="lc-channel-settings__management-zone">
        {@render actions()}
      </section>{/if}
  </div></AppShell.Sidebar.Body
>

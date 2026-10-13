<script lang="ts">
  import { Button } from "@lapismd/design-core/shadcn/button";
  import * as Tooltip from "@lapismd/design-core/shadcn/tooltip";
  import ArrowLeft from "@lucide/svelte/icons/arrow-left";
  import Refresh from "@lucide/svelte/icons/refresh-cw";
  import Pin from "@lucide/svelte/icons/pin";
  import PinOff from "@lucide/svelte/icons/pin-off";
  import Ban from "@lucide/svelte/icons/ban";
  import Retry from "@lucide/svelte/icons/rotate-ccw";
  import CommunityRelativeTime from "../community-relative-time/CommunityRelativeTime.svelte";
  let {
    name,
    id,
    created,
    pinned,
    loading,
    action,
    disabled,
    onClose,
    onRefresh,
    onPin,
    onCommand,
  }: {
    name: string;
    id: string;
    created: number;
    pinned: boolean;
    loading: boolean;
    action?: "cancel" | "retry";
    disabled: boolean;
    onClose(): void;
    onRefresh(): void;
    onPin(): void;
    onCommand(): void;
  } = $props();
</script>

<header class="community-projects__task-detail-header">
  <Button
    variant="ghost"
    size="icon-sm"
    class="community-projects__detail-back"
    aria-label="Back to history"
    onclick={onClose}><ArrowLeft aria-hidden="true" /></Button
  >
  <div>
    <h2>{name} <span title={id}>#{id.slice(0, 8)}</span></h2>
    <p>Run created <CommunityRelativeTime timestamp={created / 1000} /></p>
  </div>
  <div class="workflow-run-actions" aria-label="Run actions">
    <Tooltip.Provider>
      <Tooltip.Root
        ><Tooltip.Trigger
          >{#snippet child({ props })}<Button
              {...props}
              variant="ghost"
              size="icon-sm"
              aria-label="Refresh run"
              disabled={loading}
              onclick={onRefresh}><Refresh aria-hidden="true" /></Button
            >{/snippet}</Tooltip.Trigger
        ><Tooltip.Content role="tooltip">Refresh run</Tooltip.Content
        ></Tooltip.Root
      >
      <Tooltip.Root
        ><Tooltip.Trigger
          >{#snippet child({ props })}<Button
              {...props}
              variant="ghost"
              size="icon-sm"
              aria-label={pinned ? "Follow latest" : "Pin this history"}
              aria-pressed={pinned}
              onclick={onPin}
              >{#if pinned}<PinOff aria-hidden="true" />{:else}<Pin
                  aria-hidden="true"
                />{/if}</Button
            >{/snippet}</Tooltip.Trigger
        ><Tooltip.Content role="tooltip"
          >{pinned ? "Follow latest" : "Pin this history"}</Tooltip.Content
        ></Tooltip.Root
      >
      {#if action}<Tooltip.Root
          ><Tooltip.Trigger
            >{#snippet child({ props })}<Button
                {...props}
                variant="ghost"
                size="icon-sm"
                aria-label={action === "retry" ? "Retry run" : "Cancel run"}
                {disabled}
                onclick={onCommand}
                >{#if action === "retry"}<Retry aria-hidden="true" />{:else}<Ban
                    aria-hidden="true"
                  />{/if}</Button
              >{/snippet}</Tooltip.Trigger
          ><Tooltip.Content role="tooltip"
            >{action === "retry" ? "Retry run" : "Cancel run"}</Tooltip.Content
          ></Tooltip.Root
        >{/if}
    </Tooltip.Provider>
  </div>
</header>

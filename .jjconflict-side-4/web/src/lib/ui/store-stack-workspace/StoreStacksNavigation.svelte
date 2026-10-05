<script lang="ts">
  import type { Snippet } from "svelte";
  import { Button } from "@lapismd/design-core/shadcn/button";
  import * as Tooltip from "@lapismd/design-core/shadcn/tooltip";
  import ArrowLeft from "@lucide/svelte/icons/arrow-left";
  import StoreViewTabs from "../store-application/StoreViewTabs.svelte";
  import type { StackWorkspaceController } from "../../controller/stack-workspace/controller.svelte.js";
  import type { StoreStacksView } from "../../controller/store-stacks-navigation.js";
  let {
    controller,
    view,
    onSelectView,
    onBack,
    actions,
  }: {
    controller: StackWorkspaceController;
    view?: StoreStacksView;
    onSelectView?: (view: StoreStacksView) => void;
    onBack?: () => void;
    actions: Snippet;
  } = $props();
  const sections = $derived([
    { value: "branches", label: "Branches" },
    { value: "workspace", label: "Workspace" },
    ...(view
      ? [
          { value: "log", label: "Commits" },
          { value: "oplog", label: "Op Log" },
        ]
      : []),
  ]);
</script>

<StoreViewTabs
  value={view ?? controller.page}
  {sections}
  disabled={controller.locked}
  label="Local repository views"
  navigationLabel="Stack workspace navigation"
  onSelect={(value) => {
    if (onSelectView) onSelectView(value as StoreStacksView);
    else controller.page = value as "branches" | "workspace";
  }}
  trailing={actions}
>
  {#snippet leading()}
    {#if onBack}<Tooltip.Provider
        ><Tooltip.Root
          ><Tooltip.Trigger>
            {#snippet child({ props })}<Button
                {...props}
                variant="ghost"
                size="icon"
                style="--ui-button-radius: 999px"
                aria-label="Back to repository"
                disabled={controller.locked}
                onclick={onBack}><ArrowLeft aria-hidden="true" /></Button
              >{/snippet}
          </Tooltip.Trigger><Tooltip.Content>Back to repository</Tooltip.Content
          ></Tooltip.Root
        ></Tooltip.Provider
      >{/if}
  {/snippet}
</StoreViewTabs>

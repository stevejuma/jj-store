<script lang="ts">
  import type { Snippet } from "svelte";
  import * as Menu from "@lapismd/design-core/shadcn/dropdown-menu";
  import RotateCw from "@lucide/svelte/icons/rotate-cw";
  import StoreRepositoryActions from "../store-mutations/StoreRepositoryActions.svelte";
  import StoreRuntimeReload from "../store-settings/StoreRuntimeReload.svelte";
  import { isStacksRoute } from "../../controller/store-stacks-navigation.js";
  import type { StoreController } from "../../controller/contracts.js";
  let {
    controller,
    disabled = false,
    menu = $bindable(),
  }: {
    controller: StoreController;
    disabled?: boolean;
    menu?: Snippet;
  } = $props();
  const reload = $derived(controller.reload);
  const view = $derived(controller.state);
  const changes = $derived(
    !isStacksRoute(view.route) &&
      !view.route.globalSettings &&
      !["settings", "threads", "workflows"].includes(view.route.page) &&
      !view.route.agentsHub &&
      view.authenticated &&
      view.mutation.available.length > 0,
  );
  let repositoryMenu = $state<Snippet>();
  let trigger = $state<HTMLButtonElement | null>(null);
  $effect(() => {
    menu = changes || reload?.value ? repositoryMenu : undefined;
  });
  function openReload() {
    if (!reload?.value) return;
    reload.returnFocus = () => trigger?.focus();
    reload.open = true;
    void reload.observe();
  }
</script>

<StoreRepositoryActions
  {controller}
  {disabled}
  renderMenu={false}
  bind:menu={repositoryMenu}
  bind:menuTrigger={trigger}
  menuLabel="Application menu"
  showChanges={changes}
  extraItems={reload?.value ? reloadItem : undefined}
  onCloseAutoFocus={(event) => {
    if (reload?.open) event.preventDefault();
  }}
/>
<StoreRuntimeReload manager={reload} triggerless />

{#snippet reloadItem()}<Menu.Group
    ><Menu.Item onSelect={openReload}
      ><RotateCw aria-hidden="true" />Reload saved settings</Menu.Item
    ></Menu.Group
  >{/snippet}

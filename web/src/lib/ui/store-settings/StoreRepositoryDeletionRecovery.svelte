<script lang="ts">
  import { onDestroy } from "svelte";
  import StoreNotificationHost from "../store-notifications/StoreNotificationHost.svelte";
  import { ConfirmDialog } from "@lapismd/design-core/forms/dialog";
  import { NativeRepositoryDeletion } from "../../controller/native-repository-deletion.js";
  import { ownNotifications } from "../../notifications/notification-context.svelte.js";
  import { storeErrorMessage } from "../../controller/store-error.js";
  import type { StoreController } from "../../controller/contracts.js";
  let {
    controller,
    fetcher,
  }: { controller: StoreController; fetcher?: typeof fetch } = $props();
  const { context, owned } = ownNotifications();
  let manager: NativeRepositoryDeletion | undefined;
  let checked = false;
  let open = $state(false),
    busy = $state(false),
    name = $state("");
  const scope = "repository-management",
    key = "repository-deletion-recovery";
  $effect(() => {
    if (!controller.state.nativeRepositoryDeletion || checked) return;
    checked = true;
    try {
      manager = new NativeRepositoryDeletion(localStorage, fetcher);
      if (manager.pending) {
        name = manager.pending.name;
        context.service.publish({
          scope,
          key,
          title: "Repository deletion needs attention",
          message: `Check or continue the original deletion of ${name}.`,
          lifetime: "dismiss",
          severity: "warning",
          action: {
            id: "continue",
            label: "Continue deletion",
            run: () => {
              open = true;
            },
          },
        });
      }
    } catch (cause) {
      report(cause);
    }
  });
  onDestroy(() => manager?.close());
  function report(cause: unknown) {
    context.service.publish({
      scope,
      key,
      title: "Repository deletion failed",
      message: storeErrorMessage(cause),
      lifetime: "dismiss",
      action: {
        id: "retry",
        label: "Continue deletion",
        run: () => {
          open = true;
        },
      },
    });
  }
  async function resume() {
    if (!manager?.pending || busy) return false;
    busy = true;
    try {
      await manager.remove();
      context.service.resolve(scope, key);
      open = false;
      controller.actions.disconnect();
      await controller.projects?.openHub("repositories");
      await controller.projects?.refresh();
      return true;
    } catch (cause) {
      report(cause);
      return false;
    } finally {
      busy = false;
    }
  }
</script>

<ConfirmDialog
  bind:open
  title={`Continue deleting ${name}?`}
  description="Continue the original request to permanently erase all server objects for this repository. Local folders and independent backups remain untouched."
  confirmLabel="Check and continue deletion"
  destructive
  pending={busy}
  onConfirm={resume}
/>

{#if owned}<StoreNotificationHost service={context.service} />{/if}

<script lang="ts">
  import * as Alert from "@lapismd/design-core/shadcn/alert";
  import { Button } from "@lapismd/design-core/shadcn/button";
  import * as Dialog from "@lapismd/design-core/shadcn/dialog";
  import GitMergeIcon from "@lucide/svelte/icons/git-merge";
  import type { StoreController } from "../../controller/contracts.js";
  import type { Head } from "../../api/client.js";
  import StoreMutationDialog from "../store-mutations/StoreMutationDialog.svelte";
  let { controller }: { controller: StoreController } = $props();
  let open = $state(false),
    loading = $state(false),
    error = $state("");
  let captured = $state<Head>();
  const view = $derived(controller.state);
  const unresolved = $derived(
    ["unknown", "applying"].includes(view.mutation.status),
  );
  const resume = $derived(
    unresolved && view.mutation.command?.type === "reconcile_operations",
  );
  const available = $derived(
    view.mutation.available.includes("reconcile_operations") &&
      (resume ||
        Boolean(
          view.settings.syncSettings.value?.worker?.reconciliation_required,
        )),
  );
  const pause = `sync-reconciliation-${crypto.randomUUID()}`;
  $effect(() => {
    controller.actions.setRefreshPaused?.(pause, open);
    return () => controller.actions.setRefreshPaused?.(pause, false);
  });
  async function launch() {
    if (loading) return;
    error = "";
    if (resume) {
      open = true;
      return;
    }
    loading = true;
    try {
      captured = await controller.actions.captureReconciliation();
      open = true;
    } catch (cause) {
      error =
        cause instanceof Error ? cause.message : "Reconciliation unavailable";
    } finally {
      loading = false;
    }
  }
  async function changed(next: boolean) {
    if (!next && view.mutation.status === "committed")
      await controller.actions.refreshSyncSettings();
  }
</script>

{#if available}
  <div class="store-settings__action-row">
    <div>
      <strong>Operation reconciliation</strong>
      <p>Review and merge the current operation heads.</p>
    </div>
    <Button
      variant="outline"
      disabled={loading || (unresolved && !resume)}
      onclick={() => void launch()}
      ><GitMergeIcon data-icon="inline-start" aria-hidden="true" />{resume
        ? "Recover reconciliation"
        : "Reconcile operations"}</Button
    >
  </div>
{/if}
{#if error}<Alert.Root variant="destructive"
    ><Alert.Description>{error}</Alert.Description></Alert.Root
  >{/if}
<Dialog.Root bind:open onOpenChange={changed}>
  <StoreMutationDialog
    {controller}
    action="reconcile"
    {open}
    capturedHead={captured}
  />
</Dialog.Root>

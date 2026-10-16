<script lang="ts">
  import { getContext, onDestroy } from "svelte";
  import { Button } from "@lapismd/design-core/shadcn/button";
  import { ConfirmDialog } from "@lapismd/design-core/forms/dialog";
  import TrashIcon from "@lucide/svelte/icons/trash-2";
  import StoreSettingsCard from "./StoreSettingsCard.svelte";
  import StoreSettingsFooter from "./StoreSettingsFooter.svelte";
  import StoreNotificationHost from "../store-notifications/StoreNotificationHost.svelte";
  import { ownNotifications } from "../../notifications/notification-context.svelte.js";
  import { ApiError } from "../../api/client.js";
  import { LocalRepositoryRemoval } from "../../controller/local-repository-removal.js";
  import { NativeRepositoryDeletion } from "../../controller/native-repository-deletion.js";
  import {
    repositorySettingsGuardKey,
    type RepositorySettingsDraftGuard,
    sameRepositorySettingsOwner,
  } from "./repository-settings-guards.js";
  import { settingsNavigationReason } from "./settings-navigation.js";
  import { storeErrorMessage } from "../../controller/store-error.js";
  import type { StoreController } from "../../controller/contracts.js";
  let {
    controller,
    fetcher,
  }: { controller: StoreController; fetcher?: typeof fetch } = $props();
  const id = $props.id();
  const { context, owned } = ownNotifications();
  const view = $derived(controller.state);
  const local = $derived(view.repository.profile === "git_sha1_jj045");
  const enabled = $derived(
    view.accessMode === "local_trusted" &&
      view.repository.permission === "admin" &&
      (local || view.nativeRepositoryDeletion === true),
  );
  const draftGuard = getContext<RepositorySettingsDraftGuard | undefined>(
    repositorySettingsGuardKey,
  );
  const blocked = $derived(
    settingsNavigationReason(view.settings) ||
      draftGuard?.folders ||
      (["previewing", "prepared", "applying", "unknown"].includes(
        view.mutation.status,
      ) ||
      controller.configuration?.state.drafts.sync?.dirty ||
      controller.configuration?.state.drafts.sync?.unknown ||
      controller.configuration?.state.saving
        ? "Finish the current action before removing the repository."
        : ""),
  );
  let open = $state(false),
    busy = $state(false),
    pending = $state(false);
  let manager: LocalRepositoryRemoval | NativeRepositoryDeletion | undefined;
  let destroyed = false;
  let selected = $state({ id: "", name: "" });
  function report(title: string, cause: unknown) {
    context.service.publish({
      scope: "repository-management",
      key: `repository-removal:${selected.id}`,
      title,
      message: storeErrorMessage(cause),
      lifetime: "dismiss",
      ...(manager instanceof NativeRepositoryDeletion && manager.pending
        ? {
            action: {
              id: "retry-original-deletion",
              label: "Retry original deletion",
              run: () => {
                if (busy) return;
                manager?.close();
                manager = new NativeRepositoryDeletion(localStorage, fetcher);
                void remove();
              },
            },
          }
        : {}),
    });
  }
  $effect(() => {
    const actions = controller.actions;
    actions.setNavigationGuard?.(id, (before, after) =>
      (busy || pending) && !sameRepositorySettingsOwner(before, after)
        ? "Resolve the repository removal before leaving Settings."
        : "",
    );
    return () => actions.setNavigationGuard?.(id);
  });
  onDestroy(() => {
    destroyed = true;
    // Repository authorization loss may close Settings during erasure. The
    // already confirmed installation request owns its remaining bounded calls.
    if (!(busy && manager instanceof NativeRepositoryDeletion))
      manager?.close();
  });
  function show() {
    if (!enabled || blocked || busy) return;
    if (!pending) {
      manager?.close();
      selected = { id: view.repository.id, name: view.repository.name };
      try {
        if (local) manager = new LocalRepositoryRemoval(selected.id, fetcher);
        else {
          const snapshot = view.settings.snapshot;
          if (!snapshot || !("epoch" in snapshot))
            throw new ApiError("snapshot_unavailable");
          manager = new NativeRepositoryDeletion(localStorage, fetcher, {
            repository: selected.id,
            name: selected.name,
            epoch: snapshot.epoch,
          });
          pending = Boolean(manager.pending);
        }
      } catch (cause) {
        report("Repository removal unavailable", cause);
        return;
      }
    }
    open = true;
  }
  async function remove() {
    if (!manager || busy || (!pending && (!enabled || blocked))) return false;
    busy = true;
    pending = true;
    try {
      await manager.remove();
    } catch (cause) {
      report("Repository removal failed", cause);
      if (manager instanceof NativeRepositoryDeletion && !manager.pending) {
        pending = false;
        open = false;
      }
      if (cause instanceof ApiError && cause.code === "stale_local_revision") {
        // A new confirmation is required before capturing a newer revision.
        pending = false;
        open = false;
      }
      return false;
    } finally {
      busy = false;
      if (destroyed) manager.close();
    }
    context.service.resolve(
      "repository-management",
      `repository-removal:${selected.id}`,
    );
    pending = false;
    open = false;
    // Release repository resources before opening the discovery owner.
    if (controller.state.repository.id === selected.id)
      controller.actions.disconnect();
    try {
      await controller.projects?.openHub("repositories");
      await controller.projects?.refresh();
    } catch (cause) {
      report("Repository list refresh failed", cause);
    }
    return true;
  }
</script>

{#if enabled}
  <StoreSettingsCard
    id={`${id}-removal`}
    title={local ? "Remove repository" : "Delete repository"}
    description={local
      ? "Remove this local repository from jj-store."
      : "Permanently delete this repository from server storage."}
    Icon={TrashIcon}
  >
    <StoreSettingsFooter>
      {#snippet description()}<p>
          {local
            ? "Your folder, files and repository history will remain on disk."
            : "Deletes all server objects, including code, history and collaboration data. Local folders remain untouched."}
        </p>{/snippet}
      <Button variant="destructive" disabled={busy || !!blocked} onclick={show}
        ><TrashIcon aria-hidden="true" />{pending
          ? "Continue removal"
          : local
            ? "Remove repository"
            : "Delete repository"}</Button
      >
    </StoreSettingsFooter>
  </StoreSettingsCard>
{/if}
<ConfirmDialog
  bind:open
  title={`${local ? "Remove" : "Delete"} ${selected.name}?`}
  description={local
    ? "This removes the repository from this installation. Your folder, files and repository history will remain on disk. You can add the folder again later."
    : "This permanently deletes all server objects for this repository, including code, history, projects, reviews and agent records. This cannot be undone. Local folders and independent backups remain untouched."}
  confirmLabel={pending
    ? "Check and retry removal"
    : local
      ? "Remove repository"
      : "Delete repository"}
  destructive
  pending={busy}
  confirmDisabled={!enabled || !!blocked}
  onConfirm={remove}
/>
{#if owned}<StoreNotificationHost service={context.service} />{/if}

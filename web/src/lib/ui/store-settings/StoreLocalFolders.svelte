<script lang="ts">
  import { settingsPresentation } from "./settings-presentation.js";
  import StoreSettingsSkeleton from "./StoreSettingsSkeleton.svelte";
  import StoreLocalFolderBinding from "./StoreLocalFolderBinding.svelte";
  import StoreSettingsFooter from "./StoreSettingsFooter.svelte";
  import StoreSettingsCard from "./StoreSettingsCard.svelte";
  import StoreSettingsNotice from "./StoreSettingsNotice.svelte";
  import { getContext, onDestroy } from "svelte";
  import {
    repositorySettingsGuardKey,
    type RepositorySettingsDraftGuard,
  } from "./repository-settings-guards.js";
  import { onMount, type Snippet } from "svelte";
  import { Button } from "@lapismd/design-core/shadcn/button";
  import StoreLocalFolderDialog from "./StoreLocalFolderDialog.svelte";
  import StoreSettingsEmpty from "./StoreSettingsEmpty.svelte";
  import FolderGitIcon from "@lucide/svelte/icons/folder-git-2";
  import StoreLocalFolderBindingActions from "./StoreLocalFolderBindingActions.svelte";
  import GitBranchPlusIcon from "@lucide/svelte/icons/git-branch-plus";
  import PlusIcon from "@lucide/svelte/icons/plus";
  import RefreshCwIcon from "@lucide/svelte/icons/refresh-cw";
  import type { Model } from "../../api/client.js";
  import type { StoreController } from "../../controller/contracts.js";
  import { LocalFolderManager } from "../../controller/local-folder-manager.js";
  import { storeErrorMessage } from "../../controller/store-error.js";
  let {
    controller,
    repository,
    visible = true,
    mode = "settings",
    trigger,
  }: {
    controller: StoreController;
    repository?: string;
    visible?: boolean;
    mode?: "settings" | "add";
    trigger?: Snippet<[add: () => void, disabled: boolean]>;
  } = $props();
  const draftGuard = getContext<RepositorySettingsDraftGuard | undefined>(
    repositorySettingsGuardKey,
  );
  $effect(() => {
    if (repository && draftGuard)
      draftGuard.folders =
        pending || busy || (open && (path || name))
          ? "Finish or cancel the folder form before changing repositories."
          : "";
  });
  onDestroy(() => {
    if (repository && draftGuard) draftGuard.folders = "";
  });
  let manager = $state<LocalFolderManager>();
  let bindingsLoaded = $state(false);
  let bindings = $state<Model<"LocalBinding">[]>([]);
  let open = $state(false),
    path = $state(""),
    name = $state(""),
    reconnect = $state<string | null>(null);
  let inspected = $state<Model<"LocalFolder"> | null>(null),
    pending = $state(false),
    busy = $state(false),
    message = $state("");
  const visibleBindings = $derived(
    bindings.filter(
      (binding) => !repository || binding.repository === repository,
    ),
  );
  async function run(action: () => Promise<void>) {
    if (busy) return;
    busy = true;
    message = "";
    try {
      await action();
    } catch (cause) {
      message = storeErrorMessage(cause);
    } finally {
      pending = Boolean(manager?.pending);
      busy = false;
    }
  }
  async function list() {
    bindings = (await manager!.list()).bindings;
    bindingsLoaded = true;
  }
  async function changed() {
    await list();
    await controller.projects?.refresh();
  }
  onMount(() => {
    try {
      manager = new LocalFolderManager(undefined, localStorage);
      pending = Boolean(manager.pending);
      if (manager.pending) {
        path = manager.pending.path;
        name = manager.pending.name;
      }
      void run(async () => {
        await list();
        if (manager!.pending) {
          const result = await manager!.status();
          if (result?.status === "committed") await changed();
        }
      });
    } catch (cause) {
      message = storeErrorMessage(cause);
    }
    const subscription = controller.freshness?.watch({
      initial: false,
      matches: (change) =>
        change.repository === null &&
        ["repository", "discovery", "workspaces", "conversion"].includes(
          change.domain,
        ),
      active: () =>
        Boolean(
          visible &&
          manager &&
          !busy &&
          !pending &&
          (mode === "settings" || open),
        ),
      refresh: async () => {
        await run(list);
        return !message;
      },
    });
    const timer = subscription
      ? undefined
      : setInterval(() => {
          if (
            visible &&
            !document.hidden &&
            manager &&
            !busy &&
            !pending &&
            (mode === "settings" || open)
          )
            void run(list);
        }, 5000);
    return () => {
      subscription?.stop();
      clearInterval(timer);
      manager?.close();
    };
  });
  function edit(id: string | null) {
    reconnect = id;
    inspected = null;
    if (!pending) {
      path = "";
      name = "";
    }
    open = true;
  }
  async function save() {
    if (reconnect) {
      await manager!.reconnect(reconnect, inspected!.path);
    } else {
      const outcome = await manager!.add(inspected?.path ?? path, name);
      if (outcome.status !== "committed") {
        message =
          "Registration outcome is unknown. Check status or retry the retained request.";
        return;
      }
    }
    open = false;
    await changed();
  }
  const presentation = settingsPresentation();
  const modern = $derived(presentation.value !== "default");
</script>

{#snippet bindingActions(binding: Model<"LocalBinding">)}
  <StoreLocalFolderBindingActions
    disabled={busy || pending}
    onReconnect={() => edit(binding.repository)}
    onRemove={() =>
      run(async () => {
        await manager!.remove(binding.repository);
        if (controller.state.repository.id === binding.repository)
          controller.actions.disconnect();
        await changed();
      })}
  />
{/snippet}

{#if mode === "add"}
  {#if trigger}{@render trigger(
      () => edit(null),
      busy || !manager,
    )}{:else}<Button
      variant="ghost"
      size="icon-sm"
      aria-label="Add local repository"
      title="Add local repository"
      disabled={busy || !manager}
      onclick={() => edit(null)}
      ><GitBranchPlusIcon aria-hidden="true" /></Button
    >{/if}
{:else}
  {#snippet folderManagement()}
    <StoreSettingsFooter>
      {#snippet description()}<div>
          <strong
            >{repository ? "Registered folder" : "Register a folder"}</strong
          >
          <p>
            Register an existing JJ folder for browsing and projects. Add
            initializes companion metadata when it is missing. Search, code
            edits, reviews, conversion and sync are unavailable.
          </p>
        </div>{/snippet}

      {#if !repository}<Button
          variant="outline"
          disabled={busy || !manager}
          onclick={() => edit(null)}
          ><PlusIcon aria-hidden="true" />{pending
            ? "Resume Add"
            : "Add folder"}</Button
        >{/if}
      <Button
        variant="outline"
        disabled={busy || !manager}
        onclick={() => run(list)}
        ><RefreshCwIcon aria-hidden="true" />Refresh folders</Button
      >
      {#if pending}<Button
          variant="outline"
          disabled={busy}
          onclick={() =>
            run(async () => {
              const result = await manager!.status();
              message = result?.status ?? "No pending registration";
              if (result?.status === "committed") await changed();
            })}>Check registration</Button
        >{/if}
    </StoreSettingsFooter>{/snippet}
  <StoreSettingsCard
    id="local-folders-heading"
    title="Git-backed JJ folders"
    Icon={FolderGitIcon}
  >
    {#if !repository && !modern}{@render folderManagement()}{/if}
    {#if repository && !bindingsLoaded && !message}
      <StoreSettingsSkeleton
        section="folders"
        rows={4}
        label="Loading registered folder"
      />
    {:else if visibleBindings.length}
      <svelte:element
        this={repository || modern ? "div" : "dl"}
        class={repository || modern
          ? undefined
          : "store-settings__rows store-settings__rows--controls"}
      >
        {#each visibleBindings as binding (binding.repository)}
          <StoreLocalFolderBinding {binding} repository={Boolean(repository)}
            >{@render bindingActions(binding)}</StoreLocalFolderBinding
          >
        {/each}
      </svelte:element>
    {:else if !busy && !message}<StoreSettingsEmpty
        Icon={FolderGitIcon}
        title="No Git-backed JJ folders"
        description="Add an existing JJ folder to browse its code and projects."
      />{/if}
    {#if repository || modern}{@render folderManagement()}{/if}
    <StoreSettingsNotice>
      Removing a registration preserves the folder and project links.
    </StoreSettingsNotice>
    {#if message}<StoreSettingsNotice>{message}</StoreSettingsNotice>{/if}
  </StoreSettingsCard>
{/if}
<StoreLocalFolderDialog
  bind:open
  bind:path
  bind:name
  {busy}
  {pending}
  {inspected}
  {message}
  reconnect={!!reconnect}
  onPathChange={() => (inspected = null)}
  onBrowse={() =>
    run(async () => {
      const value = await manager!.pick();
      if (value.path) {
        path = value.path;
        inspected = null;
      }
    })}
  onInspect={() =>
    run(async () => {
      inspected = await manager!.inspect(path);
    })}
  onSave={() => run(save)}
/>

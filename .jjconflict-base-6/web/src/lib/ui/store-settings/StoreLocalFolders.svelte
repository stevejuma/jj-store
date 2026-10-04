<script lang="ts">
  import StoreSettingsNotice from "./StoreSettingsNotice.svelte";
  import { onMount, type Snippet } from "svelte";
  import { Button } from "@lapismd/design-core/shadcn/button";
  import StoreLocalFolderDialog from "./StoreLocalFolderDialog.svelte";
  import StoreSettingsEmpty from "./StoreSettingsEmpty.svelte";
  import FolderGitIcon from "@lucide/svelte/icons/folder-git-2";
  import GitBranchIcon from "@lucide/svelte/icons/git-branch";
  import GitBranchPlusIcon from "@lucide/svelte/icons/git-branch-plus";
  import PlusIcon from "@lucide/svelte/icons/plus";
  import RefreshCwIcon from "@lucide/svelte/icons/refresh-cw";
  import UnlinkIcon from "@lucide/svelte/icons/unlink";
  import type { Model } from "../../api/client.js";
  import type { StoreController } from "../../controller/contracts.js";
  import { LocalFolderManager } from "../../controller/local-folder-manager.js";
  import { storeErrorMessage } from "../../controller/store-error.js";
  let {
    controller,
    repository,
    mode = "settings",
    trigger,
  }: {
    controller: StoreController;
    repository?: string;
    mode?: "settings" | "add";
    trigger?: Snippet<[add: () => void, disabled: boolean]>;
  } = $props();
  let manager = $state<LocalFolderManager>();
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
        Boolean(manager && !busy && !pending && (mode === "settings" || open)),
      refresh: async () => {
        await run(list);
        return !message;
      },
    });
    const timer = subscription
      ? undefined
      : setInterval(() => {
          if (
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
</script>

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
  <section
    class="store-settings__section"
    aria-labelledby="local-folders-heading"
  >
    <header><h2 id="local-folders-heading">Git-backed JJ folders</h2></header>
    <div class="store-settings__action-row">
      <div>
        <strong>Register a folder</strong>
        <p>
          Register an existing JJ folder for browsing and projects. Add
          initializes companion metadata when it is missing. Search, code edits,
          reviews, conversion and sync are unavailable.
        </p>
      </div>
      <div class="store-settings__actions">
        <Button
          variant="outline"
          disabled={busy || !manager}
          onclick={() => edit(null)}
          ><PlusIcon aria-hidden="true" />{pending
            ? "Resume Add"
            : "Add folder"}</Button
        >
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
      </div>
    </div>
    {#if visibleBindings.length}<dl
        class="store-settings__rows store-settings__rows--controls"
      >
        {#each visibleBindings as binding (binding.repository)}
          <div>
            <dt>
              <strong>{binding.name}</strong>
              <p class="store-settings__row-description">{binding.path}</p>
              <p class="store-settings__row-description">
                {binding.availability.replaceAll("_", " ")}
              </p>
            </dt>
            <dd class="store-settings__actions">
              <Button
                variant="outline"
                disabled={busy || pending}
                onclick={() => edit(binding.repository)}
                ><GitBranchIcon aria-hidden="true" />Reconnect folder</Button
              >
              <Button
                variant="outline"
                disabled={busy || pending}
                onclick={() =>
                  run(async () => {
                    await manager!.remove(binding.repository);
                    if (controller.state.repository.id === binding.repository)
                      controller.actions.disconnect();
                    await changed();
                  })}
                ><UnlinkIcon aria-hidden="true" />Remove registration</Button
              >
            </dd>
          </div>
        {/each}
      </dl>
    {:else if !busy && !message}<StoreSettingsEmpty
        Icon={FolderGitIcon}
        title="No Git-backed JJ folders"
        description="Add an existing JJ folder to browse its code and projects."
      />{/if}
    <StoreSettingsNotice>
      Removing a registration preserves the folder and project links.
    </StoreSettingsNotice>
    {#if message}<StoreSettingsNotice>{message}</StoreSettingsNotice>{/if}
  </section>
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

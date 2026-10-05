<script lang="ts">
  import { tick, untrack } from "svelte";
  import { MediaQuery } from "svelte/reactivity";
  import { Button } from "@lapismd/design-core/shadcn/button";
  import { Spinner } from "@lapismd/design-core/shadcn/spinner";
  import Eye from "@lucide/svelte/icons/eye";
  import FileCheck from "@lucide/svelte/icons/file-check-2";
  import StoreNotice from "../store-notifications/StoreNotice.svelte";
  import StoreConflictSurface from "../store-conflicts/StoreConflictSurface.svelte";
  import StoreStackConflictVersions from "./StoreStackConflictVersions.svelte";
  import CommunityProjectsColumnAlert from "../community-projects/CommunityProjectsColumnAlert.svelte";
  import StoreConflictText from "../store-conflicts/StoreConflictText.svelte";
  import type { WorkspaceConflict } from "../../controller/stack-workspace/types.js";
  import type { StackWorkspaceController } from "../../controller/stack-workspace/controller.svelte.js";
  let {
    controller,
    open = $bindable(false),
  }: { controller: StackWorkspaceController; open?: boolean } = $props();
  let activePath = $state(""),
    browserOpen = $state(true),
    versionPickerOpen = $state(false);
  const compact = new MediaQuery("(max-width: 700px)");
  const selected = $derived(
    controller.snapshot?.conflicts.find((item) => item.path === activePath) ??
      controller.snapshot?.conflicts[0],
  );
  let loaded = $state<Record<string, WorkspaceConflict>>({});
  let drafts = $state<Record<string, string>>({}),
    completed = $state<Record<string, boolean>>({});
  let loading = $state(false),
    failure = $state("");
  let scope = "",
    ownerScope = "";
  const locked = $derived(controller.locked);
  const conflict = $derived(
    selected
      ? (loaded[selected.path] ??
          (!controller.adapter.readConflict ? selected : null))
      : null,
  );
  $effect(() => {
    if (compact.current) browserOpen = false;
  });
  $effect(() => {
    const snapshot = controller.snapshot,
      path = selected?.path;
    const owner = JSON.stringify([snapshot?.repository, snapshot?.active]);
    if (owner !== ownerScope) {
      drafts = {};
      activePath = "";
      ownerScope = owner;
    }
    const identity = JSON.stringify([
      snapshot?.repository,
      snapshot?.readKey,
      snapshot?.revision,
    ]);
    if (identity !== scope) {
      loaded = {};
      completed = {};
      scope = identity;
    }
    failure = "";
    if (!open || !snapshot || !path || !controller.adapter.readConflict) {
      loading = false;
      return;
    }
    if (untrack(() => loaded[path])) {
      loading = false;
      return;
    }
    let cancelled = false;
    loading = true;
    void controller.adapter
      .readConflict({
        repository: snapshot.repository,
        revision: snapshot.revision,
        readKey: snapshot.readKey,
        path,
      })
      .then((value) => {
        if (!cancelled) loaded[path] = value;
      })
      .catch((error) => {
        if (!cancelled)
          failure =
            error instanceof Error ? error.message : "Conflict unavailable.";
      })
      .finally(() => {
        if (!cancelled) loading = false;
      });
    return () => {
      cancelled = true;
    };
  });
  function close() {
    if (!locked) open = false;
  }
  async function prepare() {
    if (
      !conflict ||
      !completed[conflict.path] ||
      conflict.reason ||
      loading ||
      failure ||
      !controller.can("resolve_conflict")
    )
      return;
    const command = {
      kind: "resolve_conflict" as const,
      path: conflict.path,
      content: drafts[conflict.path] ?? conflict.base,
    };
    // Release the shell surface and its focus lease before opening the preview dialog.
    open = false;
    await tick();
    await controller.prepare(command);
    if (!controller.preview && !controller.pending) open = true;
  }
</script>

<StoreConflictSurface
  {open}
  label="Resolve workspace conflicts"
  busy={locked}
  dismissOnEscape={!versionPickerOpen}
  {locked}
  bind:browserOpen
  compact={compact.current}
  files={controller.snapshot?.conflicts ?? []}
  activePath={selected?.path ?? ""}
  onSelect={(path) => {
    activePath = path;
    if (compact.current) browserOpen = false;
  }}
  onClose={close}
>
  {#snippet actions()}
    <div
      class="store-conflict-editor__actions"
      role="group"
      aria-label="Conflict actions"
    >
      <Button
        size="sm"
        variant="outline"
        disabled={!conflict ||
          Boolean(conflict.reason) ||
          loading ||
          Boolean(failure) ||
          locked ||
          !controller.can("resolve_conflict")}
        onclick={() => {
          if (conflict) completed[conflict.path] = true;
        }}
        ><FileCheck data-icon="inline-start" aria-hidden="true" />Stage merged
        file</Button
      >
      <Button
        size="sm"
        disabled={!conflict ||
          Boolean(conflict.reason) ||
          loading ||
          Boolean(failure) ||
          !completed[conflict.path] ||
          !controller.can("resolve_conflict")}
        onclick={prepare}
        ><Eye data-icon="inline-start" aria-hidden="true" />Preview resolution</Button
      >
    </div>
  {/snippet}
  <StoreNotice
    noticeKey={`stack-conflict:${selected?.path}`}
    title="Conflict unavailable"
    message={failure}
    lifetime="condition"
  />
  {#if loading}<p role="status"><Spinner />Loading conflict…</p>{/if}
  {#each controller.snapshot?.conflicts ?? [] as item (item.path)}
    {@const value =
      loaded[item.path] ?? (!controller.adapter.readConflict ? item : null)}
    {#if value}<div
        class="store-conflict-editor__path"
        hidden={loading || Boolean(failure) || selected?.path !== item.path}
      >
        {#if value.reason}<CommunityProjectsColumnAlert
            description={value.reason}
          />
        {:else if value.terms && value.terms.length !== 3}
          <StoreStackConflictVersions
            {value}
            {locked}
            onPickerOpen={(next) => {
              if (next) versionPickerOpen = true;
              else
                queueMicrotask(() => {
                  versionPickerOpen = false;
                });
            }}
            onDraft={(content, resolved) => {
              drafts[item.path] = content;
              completed[item.path] = resolved;
            }}
          />
        {:else}
          <StoreConflictText
            path={item.path}
            left={value.left}
            base={value.base}
            right={value.right}
            editable
            readOnly={locked}
            onResolvedChange={(change) => {
              drafts[item.path] = change.content;
              completed[item.path] = change.resolved;
            }}
            onBaseChange={(content) => {
              drafts[item.path] = content;
              completed[item.path] =
                content === value.left || content === value.right;
            }}
          />
        {/if}
      </div>{/if}
  {/each}
  {#if !selected}<p role="status">No unresolved conflicts.</p>{/if}
</StoreConflictSurface>

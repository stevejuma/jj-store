<script lang="ts">
  import { untrack } from "svelte";
  import type {
    RepositoryBlob,
    StoreController,
  } from "../../controller/contracts.js";
  import type { BlameViewState } from "../../controller/blame-types.js";
  import StoreNotice from "../store-notifications/StoreNotice.svelte";
  import StoreBlameView from "./StoreBlameView.svelte";
  import { sourceLines, BLAME_UNAVAILABLE } from "./blame-model.js";
  let {
    controller,
    blob,
    now,
  }: { controller: StoreController; blob: RepositoryBlob; now?: number } =
    $props();
  let state = $state<BlameViewState>({ status: "loading" });
  const route = $derived(controller.state.route);
  const available = $derived(
    Boolean(
      controller.blame &&
      route.page === "files" &&
      route.commitId &&
      route.snapshotOperationId &&
      route.path === blob.path,
    ),
  );
  const key = $derived(
    JSON.stringify([
      route.repository,
      route.snapshotOperationId,
      route.commitId,
      blob.path,
    ]),
  );
  $effect(() => {
    const provider = controller.blame;
    const { repository, snapshotOperationId, commitId } = route;
    const path = blob.path;
    const lineCount = sourceLines(blob.contents ?? "").length;
    state = { status: "loading" };
    if (!available || !provider || !snapshotOperationId || !commitId) return;
    return untrack(() =>
      provider.observe(
        {
          repository,
          operation: snapshotOperationId,
          commit: commitId,
          path,
          lineCount,
        },
        (value) => {
          state = value;
        },
      ),
    );
  });
</script>

<StoreNotice
  noticeKey={`blame:${key}`}
  title="Unable to load Blame"
  message={state.error}
/>
{#if !available}<p class="store-blame-unavailable">{BLAME_UNAVAILABLE}</p>{/if}
<StoreBlameView
  contents={blob.contents ?? ""}
  path={blob.path}
  {route}
  {now}
  view={available ? state : { status: "failed" }}
  openRoute={controller.actions.openRoute}
/>

<style>
  .store-blame-unavailable {
    padding: 0.75rem 1rem;
    color: var(--muted-foreground);
    font-size: var(--community-text-secondary);
  }
</style>

<script lang="ts">
  import { untrack } from "svelte";
  import StoreRepositoryApplication from "../../web/src/lib/ui/store-application/StoreRepositoryApplication.svelte";
  import { createThreadsController } from "../../web/src/lib/controller/production-threads.svelte.js";
  import { initialStoreRoute } from "../../web/src/lib/controller/store-route-history.svelte.js";
  import type { StoreController } from "../../web/src/lib/controller/contracts.js";
  import { createInMemoryStoreController } from "../controller/in-memory-store-controller.svelte.js";
  import { createThreadTransport, THREAD_IDS } from "./thread-transport.js";
  import { createMemoryStoryStorage } from "./memory-storage.js";
  let {
    unknownFirst = false,
    selected = false,
    readOnly = false,
  }: {
    unknownFirst?: boolean;
    selected?: boolean;
    readOnly?: boolean;
  } = $props();
  const transport = untrack(() => createThreadTransport({ unknownFirst }));
  const base = untrack(() => createInMemoryStoreController());
  const layoutStorage = untrack(createMemoryStoryStorage);
  let route = $state({
    ...initialStoreRoute(),
    repository: THREAD_IDS.repo,
    page: "threads" as const,
    threadId: untrack(() => (selected ? THREAD_IDS.thread : null)),
    replyId: null as string | null,
  });
  const threads = createThreadsController({
    source: () => transport.transport,
    repository: () => route.repository,
    principal: () => THREAD_IDS.actor,
    route: () => route,
    select: (id) => {
      route = { ...route, threadId: id, replyId: null };
    },
    selectReply: (id) => {
      route = { ...route, replyId: id };
    },
  });
  const repository = {
    id: THREAD_IDS.repo,
    name: "Collaboration home",
    permission: untrack(() =>
      readOnly ? ("read" as const) : ("admin" as const),
    ),
    principal: THREAD_IDS.actor,
  };
  const controller: StoreController = {
    threads,
    get state() {
      return { ...base.state, route, repository, repositories: [repository] };
    },
    actions: {
      ...base.actions,
      openThreads: (id = null) => threads.open(id),
      refresh: threads.refresh,
    },
  };
  $effect(() => {
    void untrack(() => threads.restore());
    return () => threads.clear();
  });
</script>

<StoreRepositoryApplication
  {controller}
  {layoutStorage}
  initiallyCollapsed={false}
/>

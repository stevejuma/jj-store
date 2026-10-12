<script lang="ts">
  import { ThreadWorkspaces } from "../../web/src/lib/controller/thread-workspaces.svelte.js";
  import { LocalClient } from "../../web/src/lib/api/local-client.js";
  import { untrack } from "svelte";
  import { ApiError } from "../../web/src/lib/api/api-error.js";
  import StoreRepositoryApplication from "../../web/src/lib/ui/store-application/StoreRepositoryApplication.svelte";
  import { createThreadsController } from "../../web/src/lib/controller/production-threads.svelte.js";
  import { initialStoreRoute } from "../../web/src/lib/controller/store-route-history.svelte.js";
  import type { StoreController } from "../../web/src/lib/controller/contracts.js";
  import { createInMemoryStoreController } from "../controller/in-memory-store-controller.svelte.js";
  import { createThreadTransport, THREAD_IDS } from "./thread-transport.js";
  import { createMemoryStoryStorage } from "./memory-storage.js";
  import { createRepositoryThreadNavigation } from "../../web/src/lib/controller/repository-thread-navigation.svelte.js";
  let {
    unknownFirst = false,
    selected = false,
    readOnly = false,
    replySelected = false,
    withReply = false,
    rejectReaction = false,
    otherRepository = false,
    multiRepository = false,
    initiallyCollapsed = false,
    displayMode = "auto",
    searchable = false,
    pagedSearch = false,
  }: {
    unknownFirst?: boolean;
    selected?: boolean;
    readOnly?: boolean;
    replySelected?: boolean;
    withReply?: boolean;
    rejectReaction?: boolean;
    otherRepository?: boolean;
    multiRepository?: boolean;
    initiallyCollapsed?: boolean;
    displayMode?: "auto" | "desktop" | "mobile";
    searchable?: boolean;
    pagedSearch?: boolean;
  } = $props();
  const transport = untrack(() => createThreadTransport({ unknownFirst }));
  const replyId = "77".repeat(16);
  untrack(() => {
    if (searchable) {
      const summary = transport.threads.get(THREAD_IDS.thread)!;
      const id = "88".repeat(16);
      transport.threads.set(id, {
        ...summary,
        value: { ...summary.value, thread_id: id, title: "planning" },
        directory: {
          message_count: 1,
          excerpt: "Release checklist for October",
          reply_count: 0,
          participants: [],
          last_activity_at_millis: 1,
        },
      });
      transport.history.set(id, []);
      transport.addMessage(id, "89".repeat(16), "Release planning discussion");
      if (pagedSearch)
        transport.transport.list = async (cursor) =>
          transport.page(
            {
              type: "threads",
              threads: [
                transport.threads.get(cursor ? id : THREAD_IDS.thread)!,
              ],
            },
            cursor ? null : "next-thread-page",
          );
    }
    if (rejectReaction) {
      const command = transport.transport.command;
      transport.transport.command = async (kind, request) => {
        if (kind === "reactions") throw new ApiError("access_denied");
        return command(kind, request);
      };
    }
    if (replySelected || withReply) {
      const root = transport.history.get(THREAD_IDS.thread)![0]!;
      transport.addMessage(
        THREAD_IDS.thread,
        replyId,
        "A retained reply",
        root.value.event.type === "message" ? root.value.event.revision : null,
      );
    }
  });
  const base = untrack(() => createInMemoryStoreController());
  const layoutStorage = untrack(createMemoryStoryStorage);
  const otherId = "99".repeat(16);
  const other = untrack(() => createThreadTransport());
  let route = $state({
    ...initialStoreRoute(),
    repository: THREAD_IDS.repo,
    page: "threads" as const,
    threadId: untrack(() => (selected ? THREAD_IDS.thread : null)),
    replyId: untrack(() => (replySelected ? replyId : null)) as string | null,
  });
  const threads = createThreadsController({
    directory: false,
    source: () =>
      route.repository === otherId ? other.transport : transport.transport,
    repository: () => route.repository,
    principal: () => THREAD_IDS.actor,
    route: () => route,
    select: (id) => {
      route = { ...route, page: "threads", threadId: id, replyId: null };
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
  const repositories = untrack(() =>
    otherRepository
      ? [repository, { ...repository, id: otherId, name: "Other repository" }]
      : [repository],
  );
  if (untrack(() => multiRepository)) {
    const row = transport.threads.get(THREAD_IDS.thread)!;
    transport.threads.set(THREAD_IDS.thread, {
      ...row,
      value: { ...row.value, codebase: [THREAD_IDS.repo, otherId] },
    });
    other.threads.clear();
  }
  const threadNavigation = createRepositoryThreadNavigation({
    repositories: () => repositories,
    credential: () => null,
    source: (id) => ({
      list: async (cursor) => {
        sidebarReads[id] = (sidebarReads[id] ?? 0) + 1;
        const page = await (id === otherId ? other : transport).transport.list(
          cursor,
        );
        return { ...page, snapshot: { ...page.snapshot, repo_id: id } };
      },
      close: () => {},
    }),
  });
  const sidebarReads = $state<Record<string, number>>({});
  const configurationReceipts = new Map<string, unknown>();
  const threadWorkspaces = new ThreadWorkspaces(
    new LocalClient("", async (_url, init) => {
      const request = JSON.parse(String(init?.body));
      if (configurationReceipts.has(request.request_id))
        return Response.json(configurationReceipts.get(request.request_id));
      const command = request.command;
      const id =
        command.type === "create" ? request.request_id : command.thread;
      if (command.type === "create")
        await transport.transport.command("threads", {
          version: 1,
          request_id: request.request_id,
          command: {
            type: "create",
            thread_id: id,
            title: command.title,
            baseline: null,
          },
        });
      const row = transport.threads.get(id)!;
      transport.threads.set(id, {
        ...row,
        value: { ...row.value, codebase: command.repositories },
      });
      const outcome = {
        request_id: request.request_id,
        home: THREAD_IDS.repo,
        thread: id,
        revision: row.reference,
      };
      configurationReceipts.set(request.request_id, outcome);
      return Response.json(outcome);
    }),
    async (outcome) => {
      threadNavigation.invalidate();
      await threadNavigation.load();
      route = { ...route, repository: outcome.home };
      await threads.open(outcome.thread);
    },
  );
  const controller: StoreController = {
    threadWorkspaces,
    threads,
    threadNavigation,
    get state() {
      return {
        ...base.state,
        route: { ...route, globalSettings: base.state.route.globalSettings },
        repository: repositories.find((repo) => repo.id === route.repository)!,
        repositories,
      };
    },
    actions: {
      ...base.actions,
      openThreads: async (id = null, owner = route.repository) => {
        if (owner !== route.repository) {
          threads.clear();
          route = { ...route, repository: owner };
        }
        await threads.open(id);
      },
      refresh: threads.refresh,
    },
  };
  $effect(() => {
    void untrack(() => threads.restore());
    return () => {
      threads.clear();
      threadNavigation.clear();
    };
  });
</script>

<output hidden data-testid="sidebar-thread-reads"
  >{JSON.stringify(sidebarReads)}</output
>

<StoreRepositoryApplication
  {controller}
  {layoutStorage}
  {initiallyCollapsed}
  {displayMode}
/>

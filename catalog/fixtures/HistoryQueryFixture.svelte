<script lang="ts">
  import { onDestroy, untrack } from "svelte";
  import StoreHistoryQuery from "../../web/src/lib/ui/store-history-query/StoreHistoryQuery.svelte";
  import { createHistoryQuery } from "../../web/src/lib/controller/history-query.svelte.js";
  import { createProductionStoreState } from "../../web/src/lib/controller/production-store-state.svelte.js";
  import { createStoreRouteHistory } from "../../web/src/lib/controller/store-route-history.svelte.js";
  import type { StoreController } from "../../web/src/lib/controller/contracts.js";
  import type { StoreDataSource } from "../../web/src/lib/controller/native-data-source.js";
  import type { HistoryQueryInfo } from "../../web/src/lib/controller/history-query-types.js";
  let { operations = false }: { operations?: boolean } = $props();
  const repository = "11".repeat(16),
    operation = "22".repeat(64);
  const data = createProductionStoreState();
  const history = createStoreRouteHistory({
    hash: () => "",
    push() {},
    replace() {},
    go() {},
    listen: () => () => {},
  });
  history.connect(repository, operation);
  history.page(untrack(() => operations) ? "oplog" : "log");
  data.status.connection = "connected";
  data.graphAtTimeMillis = 12345;
  const info: HistoryQueryInfo = {
    version: 1,
    jj_version: "0.45.1",
    max_query_bytes: 4096,
    callables: [
      {
        name: "description",
        signature: "description(pattern)",
        insert_text: "description()",
        description: "Commits matching description text.",
        kind: "function",
      },
      {
        name: "all",
        signature: "all()",
        insert_text: "all()",
        description: "Visible commits.",
        kind: "function",
      },
      {
        name: "mutable",
        signature: "mutable()",
        insert_text: "mutable()",
        description: "Commits outside immutable ancestry.",
        kind: "alias",
      },
    ],
    aliases: ["mutable()"],
    operators: ["::", "|", "&"],
    operation_fields: [
      "description",
      "user",
      "host",
      "workspace",
      "id",
      "after",
      "before",
      "snapshot",
      "from",
    ],
    unsupported: [],
    default_user_email: "fixture@example.invalid",
  };
  const source = {
    repository,
    profile: "native",
    queryInfo: async () => info,
    graph: async (_kind, input) => {
      if (input.revset === "bad(" || input.operationQuery === 'description:"')
        throw new Error("invalid fixture query");
      return {
        snapshot: {
          version: 1,
          repo_id: repository,
          operation_id: operation,
          epoch: "33".repeat(16),
          published_seq: 1,
          manifest_revision: 1,
          code_state_version: 1,
          heads: [operation],
        },
        next_cursor: null,
        data: {
          version: 1,
          operation,
          data: { graph: "log", nodes: [] },
          next: null,
          cache: {
            state: "built",
            evicted: 0,
            retained_entries: 0,
            retained_bytes: 0,
          },
          stats: {
            object_reads: 0,
            object_bytes: 0,
            work: 0,
            metadata_nodes: 0,
            predicate_file_bytes: 0,
            predicate_lines: 0,
          },
        },
      };
    },
  } satisfies Pick<
    StoreDataSource,
    "repository" | "profile" | "graph" | "queryInfo"
  >;
  let version = 0;
  const query = createHistoryQuery({
    data,
    history,
    source: () => source as unknown as StoreDataSource,
    version: () => version,
    begin: () => ++version,
    current: (ticket) => ticket === version,
  });
  const controller = {
    historyQuery: query,
    state: {
      get route() {
        return history.route;
      },
      bookmarks: [],
      workingCopies: [
        {
          workspace: "default",
          changeId: "12".repeat(16),
          message: "Workspace commit",
        },
      ],
      repository: { name: "Query fixture" },
      status: data.status,
    },
    actions: {
      selectWorkspace: async (workspace: string | null) => {
        history.replace({ ...history.route, workspace });
      },
    },
  } as unknown as StoreController;
  onDestroy(() => {
    query.dispose();
    history.destroy();
  });
</script>

<section class="history-query-fixture" aria-label="History search example">
  <StoreHistoryQuery {controller} />
  <output aria-label="Applied history query"
    >{history.route.historyQuery ?? "All history"}</output
  >
  <button type="button" onclick={() => history.refresh("44".repeat(64))}
    >Refresh current head</button
  >
  <output aria-label="Displayed history operation"
    >{history.route.snapshotOperationId}</output
  >
</section>

<style>
  .history-query-fixture {
    max-width: 40rem;
  }
  output {
    display: block;
    padding: 1rem;
  }
</style>

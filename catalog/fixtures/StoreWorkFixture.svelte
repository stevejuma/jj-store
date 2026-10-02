<script lang="ts">
  import { untrack } from "svelte";
  import StoreWork from "../../web/src/lib/ui/store-work/StoreWork.svelte";
  import {
    emptyWorkState,
    type WorkController,
    type WorkRequest,
  } from "../../web/src/lib/controller/work-types.js";
  let {
    writable = true,
    unknown = false,
    initialTab = "tasks",
    selectTask = true,
    threadFragments = false,
  }: {
    writable?: boolean;
    unknown?: boolean;
    initialTab?: string;
    selectTask?: boolean;
    threadFragments?: boolean;
  } = $props();
  const view = $state(emptyWorkState());
  const actor = {
    created_by: "01".repeat(16),
    updated_by: "01".repeat(16),
    created_at_millis: 1000,
    updated_at_millis: 1000,
  };
  const project = {
    schema_version: 1,
    id: "11".repeat(16),
    title: "Native release",
    body: "Link code across two repositories.",
    version: 1,
    repositories: { ["41".repeat(16)]: true, ["42".repeat(16)]: false },
    actor,
  };
  const parent = {
    schema_version: 1,
    id: "21".repeat(16),
    project: project.id,
    title: "Storage work",
    body: "Implement the native model.",
    state: "in_progress" as const,
    repository: "41".repeat(16),
    parent: null,
    version: 1,
    actor,
  };
  const child = {
    ...parent,
    id: "22".repeat(16),
    title: "Browser work",
    body: "Present tasks and discussion.",
    state: "blocked" as const,
    repository: "42".repeat(16),
    parent: parent.id,
  };
  view.projects = [project];
  view.project = project;
  view.tasks = [parent, child];
  view.task = untrack(() => selectTask) ? child : null;
  view.comments = [
    {
      schema_version: 1,
      id: "31".repeat(16),
      task: child.id,
      reply_to: null,
      body: "First discussion",
      version: 1,
      actor,
    },
    {
      schema_version: 1,
      id: "32".repeat(16),
      task: child.id,
      reply_to: "31".repeat(16),
      body: "Threaded reply",
      version: 1,
      actor,
    },
  ];
  if (untrack(() => threadFragments))
    view.comments.push(
      {
        ...view.comments[0]!,
        id: "33".repeat(16),
        reply_to: "32".repeat(16),
        body: null,
        version: 2,
      },
      {
        ...view.comments[0]!,
        id: "34".repeat(16),
        reply_to: "33".repeat(16),
        body: "Reply survives removal",
      },
      {
        ...view.comments[0]!,
        id: "35".repeat(16),
        reply_to: "39".repeat(16),
        body: "Reply from another page",
      },
    );
  let pending: WorkRequest | null = null;
  const work: WorkController = {
    state: view,
    async restore() {},
    async refresh() {},
    reset() {},
    async more() {},
    async select(id, task) {
      view.project = id ? project : null;
      view.task = task === parent.id ? parent : task ? child : null;
    },
    async submit(command) {
      pending = { version: 1, request_id: "51".repeat(16), command };
      view.pending = unknown ? pending : null;
      view.message = unknown
        ? "Outcome unknown. Check status or retry this request."
        : "";
    },
    async check() {
      view.pending = null;
      view.message = "";
    },
    async retry() {
      view.pending = pending;
    },
  };
</script>

<StoreWork
  {work}
  {writable}
  {initialTab}
  principal={"01".repeat(16)}
  repositories={[
    { id: "41".repeat(16), name: "Parser repository", permission: "write" },
  ]}
/>

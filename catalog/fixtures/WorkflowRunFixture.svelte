<script lang="ts">
  import { untrack } from "svelte";
  import WorkflowRunDialog from "../../web/src/lib/ui/store-workflows/WorkflowRunDialog.svelte";
  import WorkflowFeedback from "../../web/src/lib/ui/store-workflows/WorkflowFeedback.svelte";
  import StoreNotificationHost from "../../web/src/lib/ui/store-notifications/StoreNotificationHost.svelte";
  import { ownNotifications } from "../../web/src/lib/notifications/notification-context.svelte.js";
  import { createWorkflows } from "../../web/src/lib/controller/workflows.svelte.js";
  import type {
    WorkflowVersion,
    WorkflowRequest,
  } from "../../web/src/lib/controller/workflow-types.js";
  import { workflowFixture, home } from "./workflows.js";
  import {
    workflowRunReader,
    type SourceFailure,
  } from "./workflow-run-source.js";
  let {
    unknown = false,
    failure,
  }: { unknown?: boolean; failure?: SourceFailure | "connection" } = $props();
  const notifications = ownNotifications();
  const transport = untrack(() => workflowFixture(unknown));
  let workflow = $state<WorkflowVersion>(),
    open = $state(true),
    request = $state<WorkflowRequest>();
  const work = createWorkflows({
    ...transport,
    async apply(next) {
      request = next;
      return transport.apply(next);
    },
  });
  $effect(() => {
    void transport.page({ limit: 1 }).then((page) => {
      workflow = page.workflows[0];
    });
  });
  let acquisitionFailed = false;
  function acquireRepository(repository: string) {
    if (failure === "connection" && !acquisitionFailed) {
      acquisitionFailed = true;
      throw Error("Connection lookup failed");
    }
    return workflowRunReader(
      repository,
      failure === "connection" ? undefined : failure,
    );
  }
</script>

{#if workflow && open}<WorkflowRunDialog
    {workflow}
    {work}
    {acquireRepository}
    repositoryName="Parser"
    onClose={() => {
      open = false;
    }}
  />{/if}
<WorkflowFeedback owners={[[home, work]]} />
<StoreNotificationHost owner={notifications} />
{#if request}<output aria-label="Recorded workflow request"
    >{JSON.stringify(request)}</output
  >{/if}

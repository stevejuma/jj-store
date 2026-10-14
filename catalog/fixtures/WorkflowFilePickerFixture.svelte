<script lang="ts">
  import WorkflowFilePicker from "../../web/src/lib/ui/store-workflows/WorkflowFilePicker.svelte";
  import { ownNotifications } from "../../web/src/lib/notifications/notification-context.svelte.js";
  import StoreNotificationHost from "../../web/src/lib/ui/store-notifications/StoreNotificationHost.svelte";
  import type { StoreDataSource } from "../../web/src/lib/controller/native-data-source.js";
  let { fail = false }: { fail?: boolean } = $props();
  const { context } = ownNotifications();
  let value = $state(".github/workflows/ci.yml");
  const repository = "01".repeat(16),
    operation = "02".repeat(64),
    commit = "03".repeat(64),
    snapshot = { repository, operation };
  const acquire = () =>
    ({
      repository,
      close() {},
      async snapshot() {
        if (fail)
          throw Error(
            "Workflow source read failed. Your selection is retained.",
          );
        return { repository, heads: [operation] };
      },
      async graph() {
        return {
          snapshot,
          data: { data: { graph: "log", nodes: [{ commit: { id: commit } }] } },
        };
      },
      async paths() {
        return {
          snapshot,
          data: {
            entries: [
              { path: ".github/workflows/ci.yml" },
              { path: ".github/workflows/check.yaml" },
            ],
          },
          next_cursor: null,
        };
      },
    }) as unknown as StoreDataSource;
</script>

<div style="padding: 2rem">
  <label for="workflow-file-test">Workflow file</label><WorkflowFilePicker
    id="workflow-file-test"
    {repository}
    acquireRepository={acquire}
    bind:value
  />
</div>
<StoreNotificationHost service={context.service} />

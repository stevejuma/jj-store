<script lang="ts">
  import { onMount, onDestroy, untrack } from "svelte";
  import * as ColumnCanvas from "@lapismd/design-core/shadcn/column-canvas";
  import StoreForgePanel from "../../web/src/lib/ui/store-forge/StoreForgePanel.svelte";
  import StoreNotificationHost from "../../web/src/lib/ui/store-notifications/StoreNotificationHost.svelte";
  import { ownNotifications } from "../../web/src/lib/notifications/notification-context.svelte.js";
  import { ForgeController } from "../../web/src/lib/controller/forge/controller.svelte.js";
  import type {
    ForgeConnection,
    ForgePr,
    ForgeResponse,
    ForgeTransport,
  } from "../../web/src/lib/controller/forge/types.js";
  import "../../web/src/lib/ui/store-application/store-application.css";
  let {
    failure = false,
    mergeMethods = ["merge", "squash", "rebase"],
  }: {
    failure?: boolean;
    mergeMethods?: string[];
  } = $props();
  const notifications = ownNotifications();
  const canvas = ColumnCanvas.createColumnCanvasController({
    columns: {
      "forge-pr": { defaultWidth: 500, minWidth: 360, maxWidth: 800 },
    },
  });
  const connection: ForgeConnection = {
    provider: "github",
    api_url: "https://api.github.com",
    repository: "team/repo",
    source_repository: "team/repo",
    remote: "origin",
    target_remote: "origin",
    target: "main",
    native_stacks: false,
  };
  let pr: ForgePr = {
    number: 7,
    url: "https://github.com/team/repo/pull/7",
    title: "Add API",
    body: "## Review summary\n\nAuthored **description** with [review guidance](https://example.com/review).",
    head: "feature",
    sha: "a".repeat(40),
    base: "main",
    draft: true,
    state: "open",
    mergeable: true,
    native_stack: null,
    updated_at: "2026-10-15T10:00:00Z",
  };
  const response = (): ForgeResponse => ({
    notice: null,
    connection,
    capabilities: {
      edit: true,
      draft: true,
      merge: true,
      auto_merge: false,
      checks: true,
      logs: true,
      rerun: true,
      rerun_all: true,
      merge_methods: mergeMethods,
      cancel: true,
      native_stacks: false,
    },
    pull_requests: [{ ...pr }],
    plan: null,
    receipt: null,
    restack: null,
    checks: null,
    log: null,
    merge: null,
    merges: {},
    operation: null,
  });
  const transport: ForgeTransport = async ({ action }) => {
    const result = response();
    if (action.kind === "pr") {
      if (action.action.kind === "edit") {
        if (failure)
          throw new Error("The forge refused this description update");
        pr = { ...pr, title: action.action.title, body: action.action.body };
        result.pull_requests = [{ ...pr }];
      } else if (action.action.kind === "checks")
        result.checks = {
          sha: pr.sha,
          complete: true,
          checks: [
            {
              id: "10",
              name: "Build and test",
              state: "running",
              sha: pr.sha,
              url: "https://github.com/team/repo/actions/runs/10",
              run_id: "10",
              log_id: null,
            },
          ],
        };
      else if (action.action.kind === "jobs")
        result.checks = {
          sha: pr.sha,
          complete: true,
          checks: [
            {
              id: "11",
              name: "Unit tests",
              state: "failed",
              sha: pr.sha,
              url: null,
              run_id: "10",
              log_id: "11",
            },
          ],
        };
      else if (action.action.kind === "logs")
        result.log = "Run unit tests\nAssertion failed: expected 1, got 2";
    }
    return result;
  };
  const controller = untrack(() => new ForgeController(transport));
  onMount(async () => {
    controller.notifications = notifications.context;
    await controller.refresh();
    controller.select("feature");
  });
  onDestroy(() => controller.close());
</script>

<div
  style="height: 780px; width: 500px; background: var(--background); color: var(--foreground)"
>
  <ColumnCanvas.Root controller={canvas} displayMode="fixed">
    <ColumnCanvas.Column id="forge-pr" aria-label="Pull request details">
      <StoreForgePanel {controller} />
    </ColumnCanvas.Column>
  </ColumnCanvas.Root>
</div>
<StoreNotificationHost owner={notifications} />

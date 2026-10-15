<script lang="ts">
  import { onMount, untrack } from "svelte";
  import StoreRepositoryApplication from "../../web/src/lib/ui/store-application/StoreRepositoryApplication.svelte";
  import { StackWorkspaceController } from "../../web/src/lib/controller/stack-workspace/controller.svelte.js";
  import { ForgeController } from "../../web/src/lib/controller/forge/controller.svelte.js";
  import { createInMemoryStoreController } from "../controller/in-memory-store-controller.svelte.js";
  import { MockStackWorkspaceAdapter } from "./stack-workspace/adapter.js";
  import { workspaceStoryFixture } from "./stack-workspace/data.js";
  import { createMemoryStoryStorage } from "./memory-storage.js";
  import { ownAppearance } from "../../web/src/lib/appearance/runtime.svelte.js";
  import { createAppearanceStore } from "../../web/src/lib/appearance/store.svelte.js";
  import { workspaceForgeTransport } from "./forge/workspace-transport.js";
  import type { ForgeCheck } from "../../web/src/lib/controller/forge/types.js";
  let {
    panelOpen = true,
    workspacePullRequest = false,
    apiPullRequests = true,
    failPreview = false,
    unknownSubmit = false,
    pushState = "changes",
    apiCheckState = "running",
    apiChecksComplete = true,
    manyChecks = false,
    colorMode,
  }: {
    panelOpen?: boolean;
    workspacePullRequest?: boolean;
    apiPullRequests?: boolean;
    failPreview?: boolean;
    unknownSubmit?: boolean;
    pushState?: "changes" | "synced" | "unknown";
    apiCheckState?: ForgeCheck["state"];
    apiChecksComplete?: boolean;
    manyChecks?: boolean;
    colorMode?: "light" | "dark";
  } = $props();
  const controller = untrack(() => {
    const value = createInMemoryStoreController("stacks");
    value.actions.selectStacksView("workspace");
    return value;
  });
  const layoutStorage = untrack(createMemoryStoryStorage);
  const requestedColorMode = untrack(() => colorMode);
  if (requestedColorMode) {
    const appearance = createAppearanceStore(layoutStorage);
    appearance.mode(requestedColorMode);
    ownAppearance(appearance, layoutStorage);
  }
  const stackWorkspace = untrack(() => {
    const snapshot = workspaceStoryFixture();
    snapshot.repository = controller.state.route.repository;
    snapshot.folder = "/Users/demo/code/repository";
    snapshot.workspaces[0]!.applied = ["ui", "api"];
    const apiHead =
      pushState === "unknown"
        ? "api2"
        : (pushState === "synced" ? "2" : "4").repeat(40);
    const heads = new Map([
      ["api1", "1".repeat(40)],
      ["api2", apiHead],
      ["ui1", "3".repeat(40)],
    ]);
    for (const stack of snapshot.stacks) {
      for (const commit of stack.commits)
        commit.id = heads.get(commit.id) ?? commit.id;
      for (const head of stack.heads)
        head.commit = heads.get(head.commit) ?? head.commit;
    }
    const fixture = workspaceForgeTransport(
      workspacePullRequest,
      apiPullRequests,
      failPreview,
      unknownSubmit,
      {
        apiCheckState,
        apiChecksComplete,
        apiLocalHead: pushState === "unknown" ? undefined : apiHead,
      },
    );
    const forge = new ForgeController(async (request) => {
      const response = await fixture.transport(request);
      if (
        manyChecks &&
        response.checks &&
        request.action.kind === "pr" &&
        request.action.action.kind === "checks"
      ) {
        const original = response.checks.checks[0]!;
        response.checks = {
          ...response.checks,
          checks: Array.from({ length: 36 }, (_, index) => ({
            ...original,
            id: `check-${index}`,
            run_id: String(100 + index),
            url: `https://github.com/team/repo/actions/runs/${100 + index}`,
            name: `Workflow / job ${index + 1}`,
            started_at: "2026-10-15T10:00:00Z",
            completed_at:
              index < 2
                ? "2026-10-15T10:00:05Z"
                : index >= 8
                  ? "2026-10-15T10:00:04Z"
                  : null,
            state:
              index < 2
                ? "failed"
                : index < 4
                  ? "running"
                  : index === 4
                    ? "manual"
                    : index === 5
                      ? "unknown"
                      : index === 6
                        ? "skipped"
                        : index === 7
                          ? "cancelled"
                          : "passed",
          })),
        };
      }
      return response;
    });
    forge.data = fixture.initial;
    forge.ci = fixture.ci;
    if (panelOpen) forge.select("feature/api", "inline");
    const adapter = Object.assign(new MockStackWorkspaceAdapter(snapshot), {
      forge,
      close: () => forge.close(),
    });
    return new StackWorkspaceController(adapter);
  });
  onMount(() => {
    void stackWorkspace.refresh().then(() => {
      const source = stackWorkspace.snapshot?.stacks.find(
        (stack) => stack.id === "api",
      )?.heads[0]?.commit;
      if (panelOpen && source)
        stackWorkspace.selectFile(source, "src/api.ts", undefined, "api");
    });
    return () => {
      stackWorkspace.close();
      controller.actions.destroy();
    };
  });
</script>

<div style="height: 100vh; min-height: 36rem">
  <StoreRepositoryApplication
    {controller}
    {stackWorkspace}
    {layoutStorage}
    displayMode="desktop"
    initiallyCollapsed={true}
  />
</div>

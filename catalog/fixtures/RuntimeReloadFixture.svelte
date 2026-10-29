<script lang="ts">
  import FixtureControls from "./FixtureControls.svelte";
  import { onDestroy, untrack } from "svelte";
  import StoreRepositoryApplication from "./ApplicationStory.svelte";
  import { createInMemoryStoreController } from "../controller/in-memory-store-controller.svelte.js";
  import type { AppShellDisplayMode } from "@lapismd/design-core/shell";
  import type { StorePage } from "../../web/src/lib/ui/community-navigation/model.js";
  import { RuntimeReload } from "../../web/src/lib/controller/runtime-reload.svelte.js";
  import StoreRuntimeReload from "../../web/src/lib/ui/store-settings/StoreRuntimeReload.svelte";
  import "../../web/src/lib/ui/store-settings/StoreSettings.css";
  let {
    lost = false,
    deferred = false,
    denied = false,
    application = false,
    displayMode = "auto",
    initialPage = "workflows",
  }: {
    lost?: boolean;
    deferred?: boolean;
    denied?: boolean;
    application?: boolean;
    displayMode?: AppShellDisplayMode;
    initialPage?: StorePage;
  } = $props();
  let original: Record<string, unknown> | null = null;
  let posts = $state(0),
    checks = $state(0);
  const manager = new RuntimeReload(
    () => true,
    () => null,
    async (path, init) => {
      const body = init?.body ? JSON.parse(String(init.body)) : null;
      if (denied)
        return Response.json(
          { code: "configuration_reload_denied" },
          { status: 403 },
        );
      if (String(path).endsWith("/outcome")) {
        checks++;
        return Response.json({
          request_id: body.request_id,
          generation: 1,
          state: deferred ? "deferred" : "applied",
          results: body.domains.map((domain: string) => ({
            domain,
            state: domain === "agents" && deferred ? "deferred" : "applied",
            reason:
              domain === "agents" && deferred
                ? "Active sessions retain their captured launch settings."
                : null,
          })),
        });
      }
      if (body) {
        posts++;
        original = body;
        if (lost) throw Error("lost acknowledgement");
        return Response.json({
          request_id: body.request_id,
          generation: 1,
          state: deferred ? "deferred" : "applied",
          results: body.domains.map((domain: string) => ({
            domain,
            state: domain === "agents" && deferred ? "deferred" : "applied",
            reason:
              domain === "agents" && deferred
                ? "Active sessions retain their captured launch settings."
                : null,
          })),
        });
      }
      return Response.json({
        saved_revision: "aa".repeat(32),
        supported: [
          "service",
          "protection",
          "capture",
          "sync",
          "agents",
          "workflows",
        ],
        pending: ["service"],
        generation: 0,
        original: null,
        outcome: null,
      });
    },
  );
  onDestroy(() => manager.destroy());
  const controller = {
    ...untrack(() => createInMemoryStoreController(initialPage)),
    reload: manager,
  };
</script>

<div class="review" class:application>
  {#if application}<StoreRepositoryApplication
      {controller}
      {displayMode}
    />{:else}<StoreRuntimeReload {manager} menu />{/if}
  <FixtureControls
    ><p data-story-controls>
      Submitted {posts} · Checked {checks}
    </p></FixtureControls
  >
</div>

<style>
  .review.application {
    height: 100vh;
  }
</style>

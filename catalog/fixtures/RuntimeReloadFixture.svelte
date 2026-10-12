<script lang="ts">
  import { onDestroy } from "svelte";
  import { RuntimeReload } from "../../web/src/lib/controller/runtime-reload.svelte.js";
  import StoreRuntimeReload from "../../web/src/lib/ui/store-settings/StoreRuntimeReload.svelte";
  import "../../web/src/lib/ui/store-settings/StoreSettings.css";
  let {
    lost = false,
    deferred = false,
    denied = false,
  }: { lost?: boolean; deferred?: boolean; denied?: boolean } = $props();
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
        supported: ["service", "protection", "capture", "sync", "agents"],
        pending: ["service"],
        generation: 0,
        original: null,
        outcome: null,
      });
    },
  );
  onDestroy(() => manager.destroy());
</script>

<div class="review">
  <StoreRuntimeReload {manager} menu />
  <p>Submitted {posts} · Checked {checks}</p>
</div>

<style>
  .review {
    padding: 1rem;
  }
</style>

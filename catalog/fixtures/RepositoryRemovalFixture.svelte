<script lang="ts">
  import { untrack } from "svelte";
  import { createInMemoryStoreController } from "../controller/in-memory-store-controller.svelte.js";
  import type { StoreController } from "../../web/src/lib/controller/contracts.js";
  import StoreRepositoryRemoval from "../../web/src/lib/ui/store-settings/StoreRepositoryRemoval.svelte";
  let {
    fail = false,
    stale = false,
    reader = false,
    native = false,
    dirty = false,
    slow = false,
    unmount = false,
    uncertain = false,
  }: {
    fail?: boolean;
    stale?: boolean;
    reader?: boolean;
    native?: boolean;
    dirty?: boolean;
    slow?: boolean;
    unmount?: boolean;
    uncertain?: boolean;
  } = $props();
  const base = untrack(() => createInMemoryStoreController("files"));
  let attempts = $state(0),
    reads = $state(0),
    disconnected = $state(false),
    refreshed = $state(false);
  const repository = "ab".repeat(16);
  let destination = $state("");
  let surface = $state(true);
  const fetcher: typeof fetch = async (url, init) => {
    if (String(url).endsWith("bootstrap"))
      return new Response(
        JSON.stringify({
          installation_id: "11".repeat(16),
          capabilities: { local_native_repository_deletion: true },
        }),
        { headers: { "content-type": "application/json" } },
      );
    const isDelete = String(url).endsWith("/delete");
    if (!String(url).endsWith("/remove") && !isDelete) {
      reads++;
      return new Response(
        JSON.stringify({
          version: 1,
          revision: reads,
          bindings: [{ repository }],
        }),
        { headers: { "content-type": "application/json" } },
      );
    }
    attempts++;
    if (isDelete && uncertain && attempts === 1)
      return new Response(
        JSON.stringify({ code: "repository_deletion_restart_required" }),
        { status: 503, headers: { "content-type": "application/json" } },
      );
    if (slow) await new Promise<void>((resolve) => setTimeout(resolve, 1000));
    if (fail && attempts === 1)
      throw new TypeError("Removal connection interrupted");
    if (stale)
      return new Response('{"code":"stale_local_revision"}', {
        status: 409,
        headers: { "content-type": "application/json" },
      });
    if (isDelete) {
      if (unmount) surface = false;
      const request = JSON.parse(String(init?.body));
      return new Response(
        JSON.stringify({
          version: 1,
          repository,
          request_id: request.request_id,
          state: attempts === 1 ? "pending" : "deleted",
        }),
        { headers: { "content-type": "application/json" } },
      );
    }
    if (JSON.parse(String(init?.body)).expected_revision !== 1)
      throw new Error("Removal silently rebased");
    return new Response('{"version":1,"revision":2,"bindings":[]}', {
      headers: { "content-type": "application/json" },
    });
  };
  const controller: StoreController = {
    ...base,
    get state(): StoreController["state"] {
      return {
        ...base.state,
        accessMode: "local_trusted",
        nativeRepositoryDeletion: native,
        repository: {
          ...base.state.repository,
          id: repository,
          name: "My local repo",
          profile: native ? "native" : "git_sha1_jj045",
          permission: reader ? "read" : "admin",
        },
        settings: {
          ...base.state.settings,
          workspacePath: dirty ? "/unsaved" : "",
        },
      };
    },
    actions: {
      ...base.actions,
      disconnect() {
        disconnected = true;
      },
    },
    projects: {
      ...base.projects!,
      async openHub(tab) {
        destination = tab ?? "projects";
      },
      async refresh() {
        refreshed = true;
      },
    },
  };
</script>

{#if surface}<StoreRepositoryRemoval {controller} {fetcher} />{/if}
<output aria-label="Removal requests">{reads} reads; {attempts} writes</output>
<output aria-label="Removal destination"
  >{disconnected ? "disconnected" : "connected"}:{destination}:{refreshed
    ? "refreshed"
    : "retained"}</output
>

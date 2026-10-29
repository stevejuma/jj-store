<script lang="ts">
  import { onDestroy, untrack } from "svelte";
  import { createInMemoryStoreController } from "../controller/in-memory-store-controller.svelte.js";
  import type { StoreController } from "../../web/src/lib/controller/contracts.js";
  import StoreRepositoryDeletionRecovery from "../../web/src/lib/ui/store-settings/StoreRepositoryDeletionRecovery.svelte";
  let { fail = false }: { fail?: boolean } = $props();
  const base = untrack(() => createInMemoryStoreController("files"));
  const key = "jj-store.native-repository-deletion.v1";
  const repository = "ab".repeat(16),
    request_id = "dd".repeat(16);
  untrack(() =>
    localStorage.setItem(
      key,
      JSON.stringify({
        repository,
        name: "Interrupted repo",
        request: {
          version: 1,
          request_id,
          expected_epoch: "cd".repeat(16),
          expected_installation: "11".repeat(16),
        },
      }),
    ),
  );
  onDestroy(() => localStorage.removeItem(key));
  let requests = $state(0),
    destination = $state("");
  const fetcher: typeof fetch = async (url, init) => {
    if (!String(url).endsWith("/delete"))
      throw new Error("Unexpected recovery request");
    const body = JSON.parse(String(init?.body));
    if (body.request_id !== request_id)
      throw new Error("Replaced original request");
    requests++;
    if (fail && requests === 1) throw new TypeError("Lost acknowledgement");
    return new Response(
      JSON.stringify({ version: 1, repository, request_id, state: "deleted" }),
      { headers: { "content-type": "application/json" } },
    );
  };
  const controller: StoreController = {
    ...base,
    get state(): StoreController["state"] {
      return {
        ...base.state,
        accessMode: "local_trusted",
        nativeRepositoryDeletion: true,
      };
    },
    projects: {
      ...base.projects!,
      async openHub(tab) {
        destination = tab ?? "projects";
      },
      async refresh() {},
    },
  };
</script>

<StoreRepositoryDeletionRecovery {controller} {fetcher} />
<output aria-label="Recovery requests">{requests}</output>
<output aria-label="Recovery destination">{destination}</output>

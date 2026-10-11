<script lang="ts">
  import { untrack } from "svelte";
  import type { StoreController } from "../../web/src/lib/controller/contracts.js";
  import StoreRepositorySettings from "../../web/src/lib/ui/store-settings/StoreRepositorySettings.svelte";
  import { createInMemoryStoreController } from "../controller/in-memory-store-controller.svelte.js";

  let {
    initialRepository = "",
    retainedRepository = false,
    blocked = false,
    failSelection = false,
  }: {
    initialRepository?: string;
    retainedRepository?: boolean;
    blocked?: boolean;
    failSelection?: boolean;
  } = $props();
  let selected = $state(untrack(() => initialRepository));
  const base = untrack(() => createInMemoryStoreController());
  const repositories = [
    {
      id: "jj-store",
      name: "Managed repository",
      profile: "native",
      permission: "admin",
    },
    {
      id: "local-jj",
      name: "Folder repository",
      profile: "git_sha1_jj045",
      permission: "write",
    },
  ] as const;
  const controller: StoreController = {
    get state() {
      return {
        ...base.state,
        repositories,
        repository:
          repositories.find((repository) => repository.id === selected) ??
          (retainedRepository
            ? repositories[0]
            : { id: "", name: "", permission: null }),
        route: {
          ...base.state.route,
          repository: selected,
          projectsHub: !selected,
        },
        settings: {
          ...base.state.settings,
          workspacePath: blocked ? "/retained/draft" : "",
        },
      };
    },
    actions: {
      ...base.actions,
      async openRepository(repository) {
        if (failSelection) throw new Error("Repository access was denied.");
        selected = repository;
      },
    },
  };
</script>

<main><StoreRepositorySettings {controller} /></main>

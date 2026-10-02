<script lang="ts">
  import { Button } from "@lapismd/design-core/shadcn/button";
  import GitBranchIcon from "@lucide/svelte/icons/git-branch";
  import StoreWorkCollection from "./StoreWorkCollection.svelte";
  import StoreWorkEmpty from "./StoreWorkEmpty.svelte";
  import StoreRepositoryDirectoryRow from "./StoreRepositoryDirectoryRow.svelte";
  import ListTodoIcon from "@lucide/svelte/icons/list-todo";
  import CommunitySkeleton from "../community-skeleton/CommunitySkeleton.svelte";
  import type { StoreController } from "../../controller/contracts.js";
  import "./community-work-rows.css";
  let {
    controller,
    layout = "list",
  }: { controller: StoreController; layout?: "grid" | "list" } = $props();
  const repositories = $derived(
    controller.state.repositories.filter((repo) =>
      `${repo.name} ${controller.projects!.repositories.entries.find((entry) => entry.repository.id === repo.id)?.summary?.description ?? ""}`
        .toLowerCase()
        .includes((controller.projects!.route.workQuery ?? "").toLowerCase()),
    ),
  );
</script>

{#if controller.projects!.loading && !controller.projects!.repositories.entries.some((entry) => entry.summary)}<div
    class="store-projects__loading"
  >
    <CommunitySkeleton layout="timeline" label="Loading repositories" />
  </div>{/if}

<div class="community-projects__collection-groups">
  {#each [true, false] as mine}
    {@const rows = repositories.filter(
      (repo) => (repo.permission === "admin") === mine,
    )}
    {#if rows.length}<StoreWorkCollection
        id={`repositories:${mine ? "mine" : "other"}`}
        title={mine ? "Mine" : "Other repositories"}
        count={rows.length}
        Icon={GitBranchIcon}
      >
        <div
          class="community-projects__list"
          data-layout={layout}
          role="list"
          aria-label={mine ? "My repositories" : "Other repositories"}
        >
          {#each rows as repo (repo.id)}<div role="listitem">
              <StoreRepositoryDirectoryRow
                repository={repo}
                entries={controller.projects!.repositories.entries}
                onOpen={controller.actions.openRepository}
              >
                {#snippet actions()}<Button
                    variant="ghost"
                    size="icon-sm"
                    aria-label="Projects and tasks"
                    title="Projects and tasks"
                    onclick={() =>
                      controller.projects!.filter({
                        workRepository: repo.id,
                        workTab: "projects",
                      })}><ListTodoIcon aria-hidden="true" /></Button
                  >{/snippet}
              </StoreRepositoryDirectoryRow>
            </div>{/each}
        </div>
      </StoreWorkCollection>{/if}
  {/each}
</div>
{#if !repositories.length}<StoreWorkEmpty
    kind="repositories"
    title="No accessible repositories match"
    description="Change the search or register a repository with the jj-store CLI."
  />{/if}

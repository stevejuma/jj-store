<script lang="ts">
  import StoreActivityFilter from "./StoreActivityFilter.svelte";
  import StoreRepositoryPicker from "../store-application/StoreRepositoryPicker.svelte";
  import StoreWorkStateFilter from "./StoreWorkStateFilter.svelte";
  import * as Field from "@lapismd/design-core/shadcn/field";
  import StoreWorkField from "./StoreWorkField.svelte";
  import * as InputGroup from "@lapismd/design-core/shadcn/input-group";
  import SearchIcon from "@lucide/svelte/icons/search";
  import type { StoreController } from "../../controller/contracts.js";
  import type { Model } from "../../api/client.js";
  let {
    controller,
    project,
  }: { controller: StoreController; project?: Model<"ProjectV1"> | null } =
    $props();
  const hub = $derived(controller.projects!),
    route = $derived(hub.route),
    tab = $derived(route.workTab ?? (project ? "tasks" : "projects")),
    fieldId = $props.id();
  let search = $state("");
  $effect(() => {
    search = route.workQuery ?? "";
  });
</script>

<form
  class="store-projects__filters"
  onsubmit={(event) => {
    event.preventDefault();
    void hub.filter({ workQuery: search });
  }}
>
  <Field.Group class="community-settings__option-group">
    {#if tab !== "reviews"}<StoreWorkField
        label={`Search ${tab}`}
        hideLabel
        class="store-projects__search-field"
        id={`${fieldId}-search`}
        compact
      >
        <InputGroup.Root>
          <InputGroup.Input
            id={`${fieldId}-search`}
            bind:value={search}
            maxlength={512}
            placeholder={`Search ${tab}`}
          />
          <InputGroup.Addon align="inline-end"
            ><InputGroup.Button
              type="submit"
              size="icon-xs"
              aria-label="Search"
              title="Search"
              disabled={hub.loading}
              ><SearchIcon aria-hidden="true" /></InputGroup.Button
            ></InputGroup.Addon
          >
        </InputGroup.Root>
      </StoreWorkField>{/if}
    {#if !project && tab !== "repositories" && tab !== "reviews"}<StoreWorkField
        label="Home repository"
        id={`${fieldId}-home`}
        compact
      >
        <StoreRepositoryPicker
          id={`${fieldId}-home`}
          label="Home repository"
          repositories={controller.state.repositories}
          value={route.workHome ?? ""}
          allowEmpty
          emptyLabel="All accessible homes"
          onChange={(value) => void hub.filter({ workHome: value })}
        />
      </StoreWorkField>{/if}
    {#if tab === "activity"}<StoreWorkField label="Activity type" compact
        ><StoreActivityFilter
          value={route.workActivity ?? "all"}
          onChange={(workActivity) => void hub.filter({ workActivity })}
        /></StoreWorkField
      >{/if}
    {#if tab === "tasks"}<StoreWorkField
        label="Task state"
        id={`${fieldId}-state`}
        compact
      >
        <StoreWorkStateFilter
          id={`${fieldId}-state`}
          value={route.workState ?? "active"}
          onChange={(workState) => hub.filter({ workState })}
        />
      </StoreWorkField>{/if}
    {#if tab !== "repositories"}<StoreWorkField
        label="Repository filter"
        id={`${fieldId}-repository`}
        compact
      >
        <StoreRepositoryPicker
          id={`${fieldId}-repository`}
          label="Repository filter"
          repositories={controller.state.repositories.filter(
            (r) => !project || r.id in project.repositories,
          )}
          value={route.workRepository ?? ""}
          allowEmpty
          emptyLabel="All repositories"
          onChange={(value) => void hub.filter({ workRepository: value })}
        />
      </StoreWorkField>{/if}
  </Field.Group>
</form>

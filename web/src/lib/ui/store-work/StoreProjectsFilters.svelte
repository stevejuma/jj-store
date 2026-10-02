<script lang="ts">
  import StoreRepositoryPicker from "../store-application/StoreRepositoryPicker.svelte";
  import StoreWorkStateFilter from "./StoreWorkStateFilter.svelte";
  import * as Field from "@lapismd/design-core/shadcn/field";
  import StoreWorkField from "./StoreWorkField.svelte";
  import type { StoreController } from "../../controller/contracts.js";
  import type { Model } from "../../api/client.js";
  let {
    controller,
    project,
  }: {
    controller: StoreController;
    project?: Model<"ProjectV1"> | null;
  } = $props();
  const hub = $derived(controller.projects!),
    route = $derived(hub.route),
    tab = $derived(route.workTab ?? (project ? "tasks" : "projects")),
    fieldId = $props.id();
</script>

<Field.Group class="community-settings__option-group">
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

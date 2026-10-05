<script lang="ts">
  import * as Field from "@lapismd/design-core/shadcn/field";
  import StoreWorkField from "../store-work/StoreWorkField.svelte";
  import StoreRepositoryPicker from "../store-application/StoreRepositoryPicker.svelte";
  import type { StackWorkspaceController } from "../../controller/stack-workspace/controller.svelte.js";
  let { controller }: { controller: StackWorkspaceController } = $props();
  const targets = $derived(controller.snapshot?.targetOptions ?? []);
</script>

<Field.Group class="community-settings__option-group">
  <StoreWorkField
    label="Workspace target branch"
    description="Choose a branch to browse stacks. Creating a workspace is a separate action."
    disabled={controller.locked}
  >
    <StoreRepositoryPicker
      label="Workspace target branch"
      emptyLabel="Choose target branch"
      repositories={targets.map((target) => ({
        id: JSON.stringify([target.name, target.commit]),
        name: target.name,
        description: target.message.split("\n")[0],
      }))}
      searchLabel="Search target branches"
      groupLabel="Branches"
      emptyResult="No unambiguous target branches available."
      scrollStrategy="styled"
      controlled
      disabled={controller.locked || !controller.adapter.chooseTarget}
      onChange={(value) => {
        const target = targets.find(
          (target) => JSON.stringify([target.name, target.commit]) === value,
        );
        if (target) void controller.chooseTarget(target);
      }}
    />
  </StoreWorkField>
</Field.Group>

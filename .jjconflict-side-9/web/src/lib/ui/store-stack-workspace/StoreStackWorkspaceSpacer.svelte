<script lang="ts">
  import * as Empty from "@lapismd/design-core/shadcn/empty";
  import { Button } from "@lapismd/design-core/shadcn/button";
  import GitBranchPlus from "@lucide/svelte/icons/git-branch-plus";
  import Plus from "@lucide/svelte/icons/plus";
  import StoreStackDropGuide from "./StoreStackDropGuide.svelte";
  import type { StackWorkspaceController } from "../../controller/stack-workspace/controller.svelte.js";
  import type { WorkspaceCommand } from "../../controller/stack-workspace/types.js";
  let {
    controller,
    onCommand,
  }: {
    controller: StackWorkspaceController;
    onCommand: (command: WorkspaceCommand) => void;
  } = $props();
  const target = { kind: "new-stack" as const };
  const available = $derived(
    controller.can("create_stack") && !controller.snapshot?.conflicts.length,
  );
</script>

<!-- svelte-ignore a11y_no_static_element_interactions -->
<div
  class="stack-workspace__branch-spacer stack-workspace__drop-target"
  role="group"
  aria-label="Create branch spacer"
  data-drop-target-state={controller.drag.state(target)}
  ondragover={(event) =>
    controller.drag.over(event, target, controller.snapshot, controller.locked)}
  ondragleave={(event) => controller.drag.leave(event, target)}
  ondrop={(event) => {
    const command = controller.drag.drop(
      event,
      target,
      controller.snapshot,
      controller.locked,
    );
    if (command) onCommand(command);
  }}
>
  <StoreStackDropGuide feedback={controller.drag.for(target)} />
  <Empty.Root class="stack-workspace__branch-empty">
    <Empty.Header>
      <Empty.Media variant="icon"
        ><GitBranchPlus aria-hidden="true" /></Empty.Media
      >
      <Empty.Description>Drop files to start a branch</Empty.Description>
    </Empty.Header>
    <Empty.Content>
      <Button
        variant="link"
        size="sm"
        disabled={!available}
        onclick={() => onCommand({ kind: "create_stack", name: "" })}
      >
        Create a new branch <Plus data-icon="inline-end" aria-hidden="true" />
      </Button>
    </Empty.Content>
  </Empty.Root>
</div>

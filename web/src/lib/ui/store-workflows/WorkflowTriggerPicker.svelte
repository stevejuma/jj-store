<script lang="ts">
  import WorkflowSelect from "./WorkflowSelect.svelte";
  import Play from "@lucide/svelte/icons/play";
  import Smile from "@lucide/svelte/icons/smile";
  import GitCommit from "@lucide/svelte/icons/git-commit-horizontal";
  import GitBranch from "@lucide/svelte/icons/git-branch";
  import Message from "@lucide/svelte/icons/message-square";
  import type { WorkflowDefinition } from "../../controller/workflow-types.js";
  let {
    id,
    definition = $bindable(),
    disabled = false,
  }: {
    id: string;
    definition: WorkflowDefinition;
    disabled?: boolean;
  } = $props();
</script>

<WorkflowSelect
  id={`${id}-trigger`}
  label="When"
  value={definition.trigger.on}
  {disabled}
  options={[
    { value: "manual", label: "Run manually", Icon: Play },
    { value: "code_published", label: "Code published", Icon: GitCommit },
    { value: "bookmark_changed", label: "Bookmark changed", Icon: GitBranch },
    { value: "message_posted", label: "Thread message posted", Icon: Message },
    { value: "reaction_added", label: "Reaction added", Icon: Smile },
  ]}
  onChange={(on) => {
    definition.conditions = {};
    definition.trigger =
      on === "bookmark_changed"
        ? { on, bookmark: "main" }
        : {
            on: on as
              "manual" | "code_published" | "message_posted" | "reaction_added",
          };
  }}
/>

<script lang="ts">
  import * as Dialog from "@lapismd/design-core/shadcn/dialog";
  import { DialogFrame } from "@lapismd/design-core/forms/dialog";
  import { Button } from "@lapismd/design-core/shadcn/button";
  import { Input } from "@lapismd/design-core/shadcn/input";
  import * as Field from "@lapismd/design-core/shadcn/field";
  import type { WorkflowVersion } from "../../controller/workflow-types.js";
  import type { WorkflowController } from "../../controller/workflows.svelte.js";
  let {
    workflow,
    work,
    onClose,
  }: { workflow: WorkflowVersion; work: WorkflowController; onClose(): void } =
    $props();
  const id = $props.id();
  let operation = $state(""),
    commit = $state(""),
    bookmark = $state("");
  const valid = $derived(
    /^[0-9a-f]{128}$/.test(operation) && /^[0-9a-f]{128}$/.test(commit),
  );
  async function run() {
    if (
      await work.submit({
        type: "run",
        workflow: workflow.id,
        expected: workflow.version,
        source: {
          repository: workflow.definition.repository,
          operation,
          commit,
          bookmark: bookmark || null,
        },
      })
    )
      onClose();
  }
</script>

<Dialog.Root
  open
  onOpenChange={(open) => {
    if (!open && !work.state.writing) onClose();
  }}
  ><DialogFrame
    size="form"
    title={`Run ${workflow.definition.name}`}
    description="Choose the exact recorded source revision."
    busy={work.state.writing}
  >
    <Field.Group
      ><Field.Field
        ><Field.Label for={`${id}-operation`}>Operation</Field.Label><Input
          id={`${id}-operation`}
          bind:value={operation}
          placeholder="Full operation ID"
          maxlength={128}
        /></Field.Field
      ><Field.Field
        ><Field.Label for={`${id}-commit`}>Commit</Field.Label><Input
          id={`${id}-commit`}
          bind:value={commit}
          placeholder="Full commit ID"
          maxlength={128}
        /></Field.Field
      ><Field.Field
        ><Field.Label for={`${id}-bookmark`}>Bookmark (optional)</Field.Label
        ><Input id={`${id}-bookmark`} bind:value={bookmark} /></Field.Field
      ></Field.Group
    >
    {#snippet footer()}<Button
        variant="outline"
        disabled={work.state.writing}
        onclick={onClose}>Cancel</Button
      ><Button
        disabled={!valid || work.state.writing || !!work.state.pending}
        onclick={run}>Run workflow</Button
      >{/snippet}
  </DialogFrame></Dialog.Root
>

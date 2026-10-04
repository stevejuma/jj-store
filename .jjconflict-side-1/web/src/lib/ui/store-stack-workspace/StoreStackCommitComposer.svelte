<script lang="ts">
  // Changeyard StackStartCommitPanel: owned inline draft and staged-file composition.
  import * as Card from "@lapismd/design-core/shadcn/card";
  import * as Field from "@lapismd/design-core/shadcn/field";
  import { Input } from "@lapismd/design-core/shadcn/input";
  import { Button } from "@lapismd/design-core/shadcn/button";
  import StoreStackDropGuide from "./StoreStackDropGuide.svelte";
  import CommunityChatComposer from "../community-chat-composer/CommunityChatComposer.svelte";
  import StoreStackStagedFiles from "./StoreStackStagedFiles.svelte";
  import type { StackWorkspaceController } from "../../controller/stack-workspace/controller.svelte.js";
  import type {
    WorkspaceStack,
    WorkspaceSelection,
  } from "../../controller/stack-workspace/types.js";
  let {
    controller,
    stack,
    onOtherDrop,
  }: {
    controller: StackWorkspaceController;
    stack: WorkspaceStack;
    onOtherDrop: (event: DragEvent) => void;
  } = $props();
  const id = $props.id();
  const workspace = $derived(controller.snapshot!.active);
  const draft = $derived(controller.commitDrafts.get(workspace, stack.id));
  function open(selection?: WorkspaceSelection) {
    if (controller.locked || !controller.snapshot) return;
    if (!controller.commitDrafts.open(controller.snapshot, stack.id, selection))
      controller.validation =
        "The selected changes are unavailable or immutable.";
    else controller.selectedStack = stack.id;
  }
  function drop(event: DragEvent) {
    const command = controller.drag.drop(
      event,
      { kind: "stage", id: stack.id },
      controller.snapshot,
      controller.locked,
    );
    if (command?.kind === "create_commit") {
      for (const selection of command.selections ?? [command.selection])
        open(selection);
    } else if (command) onOtherDrop(event);
  }
  function preview() {
    if (!draft || controller.locked) return;
    void controller.prepare({
      ...(draft.commit
        ? { kind: "edit_commit" as const, commit: draft.commit }
        : { kind: "create_commit" as const, stack: stack.id }),
      message:
        draft.title.trim() +
        (draft.body.trim() ? `\n\n${draft.body.trim()}` : ""),
      ...(draft.selections.length ? { selections: draft.selections } : {}),
    });
  }
</script>

<Card.Root
  class="stack-workspace__stage-card stack-workspace__drop-target"
  data-drop-target-state={controller.drag.state({
    kind: "stage",
    id: stack.id,
  })}
  ondragover={(event) =>
    controller.drag.over(
      event,
      { kind: "stage", id: stack.id },
      controller.snapshot,
      controller.locked,
    )}
  ondragleave={(event) =>
    controller.drag.leave(event, { kind: "stage", id: stack.id })}
  ondrop={drop}
  role="group"
  aria-label={`Commit composer for ${stack.name}`}
>
  <StoreStackDropGuide
    feedback={controller.drag.for({ kind: "stage", id: stack.id })}
  />
  {#if draft}
    <form
      aria-label={`${draft.commit ? "Edit" : "Create"} commit in ${stack.name}`}
      onsubmit={(event) => {
        event.preventDefault();
        preview();
      }}
    >
      <StoreStackStagedFiles
        name={stack.name}
        selections={draft.selections}
        onSelect={(selection) =>
          controller.selectFile(
            selection.source,
            selection.path,
            selection.hunks,
          )}
      />
      <Card.Content
        class="stack-workspace__stage-content stack-workspace__stage-form"
      >
        <Field.Group>
          <Field.Field>
            <Field.Label for={`${id}-title`} class="sr-only"
              >Commit title</Field.Label
            >
            <Input
              id={`${id}-title`}
              bind:value={draft.title}
              style="height: 1.75rem;"
              placeholder="Commit title (required)"
              disabled={controller.locked}
            />
          </Field.Field>
          <Field.Field>
            <Field.Title class="sr-only">Commit description</Field.Title>
            <CommunityChatComposer
              mode="form"
              ariaLabel="Commit description"
              bind:value={draft.body}
              placeholder="Commit message"
              disabled={controller.locked}
            />
          </Field.Field>
        </Field.Group>
        <Card.Footer class="stack-workspace__commit-actions">
          <Button
            type="button"
            variant="outline"
            disabled={controller.locked}
            onclick={() => controller.commitDrafts.clear(workspace, stack.id)}
            >Cancel</Button
          >
          <Button
            type="submit"
            disabled={controller.locked || !draft.title.trim()}
            >{draft.commit ? "Save commit" : "Create commit"}</Button
          >
        </Card.Footer>
      </Card.Content>
    </form>
  {:else}
    <Card.Content
      class="stack-workspace__stage-content stack-workspace__stage-drop"
    >
      <Button
        variant="outline"
        class="stack-workspace__drop"
        disabled={controller.locked}
        aria-label={`Assign to ${stack.name}`}
        onclick={() => open(controller.selection ?? undefined)}
        >Drop files to stage or commit directly</Button
      >
    </Card.Content>
    <Card.Content
      class="stack-workspace__stage-content stack-workspace__stage-start"
    >
      <Button
        variant="outline"
        disabled={controller.locked}
        aria-label={`Create commit in ${stack.name}`}
        onclick={() => open()}>Start a commit…</Button
      >
    </Card.Content>
  {/if}
</Card.Root>

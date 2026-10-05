<script lang="ts">
  import * as Dialog from "@lapismd/design-core/shadcn/dialog";
  import * as Field from "@lapismd/design-core/shadcn/field";
  import { Button } from "@lapismd/design-core/shadcn/button";
  import { Input } from "@lapismd/design-core/shadcn/input";
  import { Textarea } from "@lapismd/design-core/shadcn/textarea";
  import { DialogFrame } from "@lapismd/design-core/forms/dialog";
  import * as ToggleGroup from "@lapismd/design-core/shadcn/toggle-group";
  import StoreRepositoryPicker from "../store-application/StoreRepositoryPicker.svelte";
  import StoreWorkField from "../store-work/StoreWorkField.svelte";
  import { commandTitle } from "../../controller/stack-workspace/presentation.js";
  import type { StackWorkspaceController } from "../../controller/stack-workspace/controller.svelte.js";
  import {
    cloneWorkspace,
    type WorkspaceCommand,
  } from "../../controller/stack-workspace/types.js";
  let {
    controller,
    command,
    open = $bindable(false),
  }: {
    controller: StackWorkspaceController;
    command: WorkspaceCommand | null;
    open?: boolean;
  } = $props();
  let name = $state(""),
    message = $state(""),
    target = $state(""),
    relationship = $state("independent"),
    parent = $state("");
  const id = $props.id();
  const selections = $derived(
    command && "selections" in command && command.selections
      ? command.selections
      : command && "selection" in command && command.selection
        ? [command.selection]
        : [],
  );
  $effect(() => {
    const seed = command;
    relationship =
      seed?.kind === "create_stack" && seed.parent
        ? "dependent"
        : "independent";
    parent = seed?.kind === "create_stack" ? (seed.parent ?? "") : "";
    name = seed && "name" in seed ? seed.name : "";
    message = seed && "message" in seed ? seed.message : "";
    target =
      seed && "target" in seed
        ? seed.target
        : seed?.kind === "move_commit" || seed?.kind === "create_commit"
          ? seed.stack
          : "";
  });
  const directCommit = $derived(
    command?.kind === "transfer"
      ? controller.snapshot?.stacks
          .flatMap((stack) => stack.commits)
          .find((commit) => commit.id === command.target)
      : undefined,
  );
  const items = $derived(
    command?.kind === "move_commit" || command?.kind === "create_commit"
      ? (controller.snapshot?.stacks.map((stack) => ({
          id: stack.id,
          name: stack.name,
          destination: stack.id,
        })) ?? [])
      : [
          ...(command?.kind === "transfer" &&
          command.selection.source !== "unassigned"
            ? [
                {
                  id: "unassigned",
                  name: "Unstaged",
                  destination: "unassigned",
                },
              ]
            : []),
          ...(controller.snapshot?.stacks.flatMap((stack) =>
            stack.heads.flatMap((head) => {
              const commit = stack.commits.find(
                (item) => item.id === head.commit,
              );
              return commit
                ? [
                    {
                      id: JSON.stringify([stack.id, head.name]),
                      name: head.name,
                      destination: head.commit,
                      description: commit.message.split("\n")[0],
                    },
                  ]
                : [];
            }),
          ) ?? []),
          ...(directCommit &&
          !controller.snapshot?.stacks.some((stack) =>
            stack.heads.some((head) => head.commit === directCommit.id),
          )
            ? [
                {
                  id: directCommit.id,
                  name: directCommit.message.split("\n")[0] || "Empty commit",
                  destination: directCommit.id,
                  description: `Commit · ${directCommit.id.slice(0, 8)}`,
                },
              ]
            : []),
        ],
  );
  async function submit() {
    if (!command) return;
    const value = cloneWorkspace(command);
    if (value.kind === "create_stack") {
      if (relationship === "dependent") {
        if (!parent) {
          controller.validation = "Choose an existing stack.";
          return;
        }
        value.parent = parent;
      } else delete value.parent;
    }
    if ("name" in value) value.name = name;
    if ("message" in value) value.message = message;
    const destination =
      items.find((item) => item.id === target)?.destination ?? target;
    if ("target" in value) value.target = destination;
    if (value.kind === "move_commit" || value.kind === "create_commit")
      value.stack = destination;
    await controller.prepare(value);
    if (controller.preview) open = false;
  }
</script>

<Dialog.Root bind:open>
  <DialogFrame
    title={command ? commandTitle(command) : "Workspace command"}
    description="Preview the change before applying it to this workspace."
    preventScroll={false}
    size="form"
  >
    {#if command}<Field.Group class="community-settings__option-group">
        {#if "name" in command}<StoreWorkField label="Name" id={`${id}-name`}
            ><Input
              id={`${id}-name`}
              bind:value={name}
              disabled={controller.busy}
            /></StoreWorkField
          >{/if}
        {#if command.kind === "create_stack"}
          <StoreWorkField
            label="Branch relationship"
            description={relationship === "dependent"
              ? "Create a branch that depends on a selected stack."
              : "Create an independent branch in a new stack."}
            disabled={controller.busy}
          >
            <ToggleGroup.Root
              type="single"
              variant="outline"
              value={relationship}
              onValueChange={(value) => {
                if (value) relationship = value;
              }}
              aria-label="Branch relationship"
              disabled={controller.busy}
            >
              <ToggleGroup.Item value="independent"
                >Independent</ToggleGroup.Item
              >
              <ToggleGroup.Item value="dependent">Dependent</ToggleGroup.Item>
            </ToggleGroup.Root>
          </StoreWorkField>
          {#if relationship === "dependent"}<StoreWorkField
              label="Add to stack"
              description="Choose the existing stack for this branch."
              disabled={controller.busy}
            >
              <StoreRepositoryPicker
                label="Add to stack"
                value={parent}
                controlled
                repositories={controller.snapshot?.stacks.map((stack) => ({
                  id: stack.id,
                  name: stack.name,
                  description: `${stack.heads.length} branches · ${stack.commits.length} commits`,
                })) ?? []}
                emptyLabel="Choose existing stack"
                searchLabel="Search stacks"
                groupLabel="Stacks"
                emptyResult="No matching stacks."
                scrollStrategy="styled"
                disabled={controller.busy}
                onChange={(value) => {
                  parent = value;
                  controller.validation = "";
                }}
              />
            </StoreWorkField>{/if}
        {/if}
        {#if "message" in command}<StoreWorkField
            label="Commit message"
            id={`${id}-message`}
            ><Textarea
              id={`${id}-message`}
              bind:value={message}
              disabled={controller.busy}
            /></StoreWorkField
          >{/if}
        {#if command.kind === "move_commit" || command.kind === "create_commit" || command.kind === "transfer"}<StoreWorkField
            label="Destination"
            description={command.kind === "transfer"
              ? "Choose the branch head or selected commit to receive these changes."
              : "Choose the stack that will receive this commit."}
            disabled={controller.busy}
            ><StoreRepositoryPicker
              label="Destination"
              value={items.find((item) => item.id === target)?.id ??
                items.find((item) => item.destination === target)?.id ??
                target}
              controlled
              repositories={items}
              emptyLabel={command.kind === "transfer"
                ? "Choose destination"
                : "Choose stack"}
              searchLabel={command.kind === "transfer"
                ? "Search destinations"
                : "Search stacks"}
              groupLabel={command.kind === "transfer"
                ? "Destinations"
                : "Stacks"}
              emptyResult="No matching destinations."
              scrollStrategy="styled"
              disabled={controller.busy}
              onChange={(value) => (target = value)}
            /></StoreWorkField
          >{/if}
        {#if selections.length}<StoreWorkField
            label="Selected changes"
            description={command.kind === "create_stack"
              ? "These files or hunks will be staged for the first commit."
              : "Only these files or hunks will move."}
            ><ul aria-label="Selected changes">
              {#each selections as selection}<li>
                  {selection.path}{selection.hunks
                    ? ` · ${selection.hunks.length} hunk(s)`
                    : " · entire file"}
                </li>{/each}
            </ul></StoreWorkField
          >{/if}
      </Field.Group>{/if}
    {#if controller.validation}<Field.Error>{controller.validation}</Field.Error
      >{/if}
    {#snippet footer()}<Button
        variant="outline"
        disabled={controller.busy}
        onclick={() => (open = false)}>Cancel</Button
      ><Button disabled={controller.busy} onclick={submit}
        >Preview command</Button
      >{/snippet}
  </DialogFrame>
</Dialog.Root>

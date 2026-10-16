<script lang="ts">
  import * as Menu from "@lapismd/design-core/shadcn/dropdown-menu";
  import { Button } from "@lapismd/design-core/shadcn/button";
  import GitBranch from "@lucide/svelte/icons/git-branch";
  import Pencil from "@lucide/svelte/icons/pencil";
  import Trash from "@lucide/svelte/icons/trash-2";
  import LogIn from "@lucide/svelte/icons/log-in";
  import Eject from "@lucide/svelte/icons/eject";
  import Ellipsis from "@lucide/svelte/icons/ellipsis";
  import StoreForgeButton from "../store-forge/StoreForgeButton.svelte";
  import type { StackWorkspaceController } from "../../controller/stack-workspace/controller.svelte.js";
  import type {
    WorkspaceStack,
    WorkspaceCommand,
  } from "../../controller/stack-workspace/types.js";
  let {
    controller,
    stack,
    applied,
    onCommand,
  }: {
    controller: StackWorkspaceController;
    stack: WorkspaceStack;
    applied: boolean;
    onCommand: (command: WorkspaceCommand) => void;
  } = $props();
</script>

<Menu.Root
  ><Menu.Trigger
    >{#snippet child({ props })}<Button
        {...props}
        variant="ghost"
        size="icon-sm"
        class="stack-workspace__menu-trigger"
        aria-label={`Actions for ${stack.name}`}
        disabled={controller.locked}><Ellipsis aria-hidden="true" /></Button
      >{/snippet}</Menu.Trigger
  >
  <Menu.Content
    align="end"
    preventScroll={false}
    class="stack-workspace__action-menu"
    ><Menu.Group>
      <StoreForgeButton {controller} {stack} menuItem />
      <Menu.Item
        disabled={!controller.can(applied ? "unapply_stack" : "apply_stack")}
        onSelect={() =>
          onCommand({
            kind: applied ? "unapply_stack" : "apply_stack",
            stack: stack.id,
            ...(controller.page === "branches" ? { branch: stack.name } : {}),
          })}
        >{#if applied}<Eject aria-hidden="true" />{:else}<LogIn
            aria-hidden="true"
          />{/if}{applied ? "Unapply stack" : "Apply stack"}</Menu.Item
      >
      <Menu.Item
        disabled={!controller.can("rename_stack")}
        title={controller.eligibility("rename_stack").reason || undefined}
        onSelect={() =>
          onCommand({
            kind: "rename_stack",
            stack: stack.id,
            name: stack.name,
          })}><Pencil aria-hidden="true" />Rename stack</Menu.Item
      >
      <Menu.Item
        disabled={!controller.can("rebase")}
        title={controller.eligibility("rebase").reason || undefined}
        onSelect={() => onCommand({ kind: "rebase", stack: stack.id })}
        ><GitBranch aria-hidden="true" />Rebase onto base</Menu.Item
      >
      <Menu.Item
        disabled={!controller.can("delete_stack")}
        title={controller.eligibility("delete_stack").reason || undefined}
        onSelect={() => onCommand({ kind: "delete_stack", stack: stack.id })}
        ><Trash aria-hidden="true" />Delete stack</Menu.Item
      >
    </Menu.Group></Menu.Content
  >
</Menu.Root>

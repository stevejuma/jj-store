<script lang="ts">
  import * as Menu from "@lapismd/design-core/shadcn/dropdown-menu";
  import ArrowUp from "@lucide/svelte/icons/arrow-up";
  import ArrowDown from "@lucide/svelte/icons/arrow-down";
  import GitBranch from "@lucide/svelte/icons/git-branch";
  import GitCommit from "@lucide/svelte/icons/git-commit-horizontal";
  import type { WorkspaceCommand } from "../../controller/stack-workspace/types.js";
  let {
    stack,
    commit,
    immutable = false,
    empty = false,
    below,
    enabled = () => true,
    reason = () => "",
    onCommand,
  }: {
    stack: string;
    commit: string;
    immutable?: boolean;
    empty?: boolean;
    below?: string;
    enabled?: (kind: WorkspaceCommand["kind"]) => boolean;
    reason?: (kind: WorkspaceCommand["kind"]) => string;
    onCommand: (command: WorkspaceCommand) => void;
  } = $props();
</script>

<Menu.Sub
  ><Menu.SubTrigger
    title={reason(empty ? "create_commit" : "add_empty_commit") || undefined}
    disabled={!enabled(empty ? "create_commit" : "add_empty_commit")}
    ><GitCommit aria-hidden="true" />Add empty commit</Menu.SubTrigger
  >
  <Menu.SubContent class="stack-workspace__action-menu"
    ><Menu.Group>
      <Menu.Item
        title={reason(empty ? "create_commit" : "add_empty_commit") ||
          undefined}
        disabled={!enabled(empty ? "create_commit" : "add_empty_commit")}
        onSelect={() =>
          onCommand(
            empty
              ? { kind: "create_commit", stack, message: "" }
              : {
                  kind: "add_empty_commit",
                  commit,
                  placement: "above",
                  message: "",
                },
          )}><ArrowUp aria-hidden="true" />Add empty commit above</Menu.Item
      >
      <Menu.Item
        title={reason("add_empty_commit") || undefined}
        disabled={immutable || empty || !enabled("add_empty_commit")}
        onSelect={() =>
          onCommand({
            kind: "add_empty_commit",
            commit,
            placement: "below",
            message: "",
          })}><ArrowDown aria-hidden="true" />Add empty commit below</Menu.Item
      >
    </Menu.Group></Menu.SubContent
  >
</Menu.Sub>
<Menu.Sub
  ><Menu.SubTrigger
    title={reason("create_branch") || undefined}
    disabled={!enabled("create_branch")}
    ><GitBranch aria-hidden="true" />Create branch</Menu.SubTrigger
  >
  <Menu.SubContent class="stack-workspace__action-menu"
    ><Menu.Group>
      <Menu.Item
        title={reason("create_branch") || undefined}
        disabled={!enabled("create_branch")}
        onSelect={() =>
          onCommand({
            kind: "create_branch",
            stack,
            commit,
            placement: "above",
            name: "",
          })}><ArrowUp aria-hidden="true" />Create branch above</Menu.Item
      >
      <Menu.Item
        title={reason("create_branch") || undefined}
        disabled={!enabled("create_branch")}
        onSelect={() =>
          onCommand({
            kind: "create_branch",
            stack,
            commit,
            placement: below ? "above" : "below",
            ...(below ? { commit: below } : {}),
            name: "",
          })}><ArrowDown aria-hidden="true" />Create branch below</Menu.Item
      >
    </Menu.Group></Menu.SubContent
  >
</Menu.Sub>

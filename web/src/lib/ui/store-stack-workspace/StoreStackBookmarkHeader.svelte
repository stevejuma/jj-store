<script lang="ts">
  import * as Card from "@lapismd/design-core/shadcn/card";
  import * as ColumnCanvas from "@lapismd/design-core/shadcn/column-canvas";
  import GitBranch from "@lucide/svelte/icons/git-branch";
  import StoreForgeButton from "../store-forge/StoreForgeButton.svelte";
  import StoreForgeBranchStatus from "../store-forge/StoreForgeBranchStatus.svelte";
  import StoreStackDropGuide from "./StoreStackDropGuide.svelte";
  import CommunityChip from "../community-chip/CommunityChip.svelte";
  import CommunityRelativeTime from "../community-relative-time/CommunityRelativeTime.svelte";
  import CommunityProjectsGitAuthorAvatar from "../community-projects/CommunityProjectsGitAuthorAvatar.svelte";
  import { toggleStackSource } from "../../controller/stack-workspace/selection-actions.js";
  import type { stackCardSelection } from "../../controller/stack-workspace/card-selection.js";
  import type { StackWorkspaceController } from "../../controller/stack-workspace/controller.svelte.js";
  import type {
    WorkspaceStack,
    StackCommit,
    WorkspaceCommand,
  } from "../../controller/stack-workspace/types.js";
  let {
    controller,
    stack,
    name,
    aliases,
    shared,
    tip,
    card,
    onCommand,
    top = false,
  }: {
    controller: StackWorkspaceController;
    stack: WorkspaceStack;
    name: string;
    aliases: string[];
    shared: boolean;
    tip?: StackCommit;
    card: ReturnType<typeof stackCardSelection>;
    onCommand: (command: WorkspaceCommand) => void;
    top?: boolean;
  } = $props();
  const headerTarget = $derived({
    kind: "stack" as const,
    id: card.owner,
    branch: card.branch,
  });
  const statusRow = $derived(
    Boolean(controller.forge?.ready) &&
      (top || Boolean(controller.forge?.pr(name))),
  );
</script>

<Card.Header
  class="stack-workspace__bookmark-header stack-workspace__drop-target"
  data-selected={card.branchSelected || undefined}
  data-drop-target-state={controller.drag.state(headerTarget)}
  ondragover={(event) =>
    controller.drag.over(
      event,
      headerTarget,
      controller.snapshot,
      controller.locked,
    )}
  ondragleave={(event) => controller.drag.leave(event, headerTarget)}
  ondrop={(event) => {
    const command = controller.drag.drop(
      event,
      headerTarget,
      controller.snapshot,
      controller.locked,
    );
    if (command) onCommand(command);
  }}
  onclick={() => {
    if (!controller.locked)
      toggleStackSource(controller, card.owner, { branch: card.branch });
  }}
>
  <StoreStackDropGuide feedback={controller.drag.for(headerTarget)} />
  <GitBranch aria-hidden="true" />
  <div>
    <div class="stack-workspace__branch-title-row">
      <Card.Title
        ><button
          type="button"
          class="stack-workspace__branch-title"
          disabled={controller.locked}
          aria-label={`Show all changes in ${name}`}
          aria-pressed={card.branchSelected}
          ><ColumnCanvas.Title>{name}</ColumnCanvas.Title></button
        ></Card.Title
      >
      {#if controller.forge?.pr(name) && !statusRow}<StoreForgeButton
          {controller}
          bookmark={name}
        />{/if}
    </div>
    {#if statusRow}<StoreForgeBranchStatus
        {controller}
        branch={name}
        localHead={stack.heads.find(
          (head) => head.name === name || head.aliases?.includes(name),
        )?.commit}
      />{/if}
    {#if tip && controller.page === "workspace"}<Card.Description
        ><CommunityProjectsGitAuthorAvatar
          name={tip.author}
          email={tip.email}
          size="xsmall"
        />{tip.author} · <CommunityRelativeTime
          timestamp={tip.createdAt}
        /></Card.Description
      >{/if}
    <div class="stack-workspace__branch-chips">
      <CommunityChip variant="muted">Local bookmark</CommunityChip>
      {#if controller.forge?.pr(name) && !statusRow}<CommunityChip
          variant="muted">{controller.forge.checkSummary(name)}</CommunityChip
        >{/if}
      {#if shared}<CommunityChip variant="muted">Shared</CommunityChip>{/if}
      {#each aliases as alias}<CommunityChip variant="muted"
          >{alias}</CommunityChip
        >{/each}
    </div>
  </div>
</Card.Header>

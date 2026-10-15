<script lang="ts">
  import { tick } from "svelte";
  import * as Collapsible from "@lapismd/design-core/shadcn/collapsible";
  import * as Tooltip from "@lapismd/design-core/shadcn/tooltip";
  import { Button } from "@lapismd/design-core/shadcn/button";
  import ChevronDown from "@lucide/svelte/icons/chevron-down";
  import ChevronRight from "@lucide/svelte/icons/chevron-right";
  import Pencil from "@lucide/svelte/icons/pencil";
  import PictureInPicture2 from "@lucide/svelte/icons/picture-in-picture-2";
  import ListChecks from "@lucide/svelte/icons/list-checks";
  import StoreForgeIcon from "./StoreForgeIcon.svelte";
  import CommunityChip from "../community-chip/CommunityChip.svelte";
  import StoreDetachablePanel from "../store-panels/StoreDetachablePanel.svelte";
  import StoreForgePullRequest from "./StoreForgePullRequest.svelte";
  import StoreForgePullRequestActions from "./StoreForgePullRequestActions.svelte";
  import type { ForgeController } from "../../controller/forge/controller.svelte.js";
  import X from "@lucide/svelte/icons/x";
  import "./forge-inline.css";
  import "./store-forge.css";
  let {
    controller,
    branch,
    onClose,
  }: { controller: ForgeController; branch: string; onClose: () => void } =
    $props();
  let editing = $state(false);
  let checksOpen = $state(false);
  const selected = $derived(
    controller.surface === "inline" && controller.selected?.head === branch,
  );
  const pr = $derived(selected ? controller.selected : controller.pr(branch));
  const expanded = $derived(
    Boolean(selected && pr && controller.detailsExpanded),
  );
  const blocked = $derived(controller.busy || Boolean(controller.pending));
  const identity = $derived(pr?.number);
  $effect(() => {
    identity;
    checksOpen = false;
  });
  $effect(() => controller.watch(branch));
  function expand(open: boolean) {
    if (open && pr) {
      if (!selected) controller.select(pr.head, "inline");
      controller.detailsExpanded = true;
    } else {
      controller.retainDraft();
      controller.detailsExpanded = false;
    }
  }
</script>

{#if pr}
  {#key pr.number}
    <StoreDetachablePanel title={`PR #${pr.number} · ${pr.head}`}>
      {#snippet children({ floating, toggle })}
        <section
          class="forge-inline"
          class:forge-inline--floating={floating}
          data-forge-summary
          aria-label={`Pull request for ${branch}`}
        >
          <Collapsible.Root
            open={expanded}
            onOpenChange={expand}
            disabled={floating}
          >
            <header class="forge-inline__summary">
              <StoreForgeIcon
                provider={controller.data?.connection?.provider}
              />
              <h2
                class="forge-inline__title"
                aria-label={selected ? controller.title || pr.title : pr.title}
              >
                <Collapsible.Trigger>
                  {#snippet child({ props })}
                    <Button
                      {...props}
                      variant="ghost"
                      size="sm"
                      aria-label={`${expanded ? "Collapse" : "Expand"} pull request #${pr.number}`}
                    >
                      {#if expanded}<ChevronDown
                          aria-hidden="true"
                        />{:else}<ChevronRight aria-hidden="true" />{/if}
                      <span
                        >{selected
                          ? controller.title || pr.title
                          : pr.title}</span
                      >
                    </Button>
                  {/snippet}
                </Collapsible.Trigger>
              </h2>
              <a
                class="forge-pr__number"
                href={pr.url}
                target="_blank"
                rel="noreferrer">#{pr.number}</a
              >
              <CommunityChip variant="muted"
                >{pr.draft ? "Draft" : pr.state}</CommunityChip
              >
              <div class="forge-inline__actions forge-header-actions">
                <StoreForgePullRequestActions {controller} {branch} />
                <Tooltip.Provider>
                  <Tooltip.Root
                    ><Tooltip.Trigger
                      >{#snippet child({ props })}
                        <Button
                          {...props}
                          variant="ghost"
                          size="icon-sm"
                          aria-label="Edit details"
                          aria-pressed={editing}
                          disabled={blocked ||
                            !controller.data?.capabilities.edit}
                          onclick={async () => {
                            if (!expanded) {
                              expand(true);
                              await tick();
                            }
                            editing = !editing;
                          }}><Pencil aria-hidden="true" /></Button
                        >
                      {/snippet}</Tooltip.Trigger
                    ><Tooltip.Content>Edit details</Tooltip.Content
                    ></Tooltip.Root
                  >
                  <Tooltip.Root
                    ><Tooltip.Trigger
                      >{#snippet child({ props })}
                        <Button
                          {...props}
                          variant="ghost"
                          size="icon-sm"
                          aria-label={floating
                            ? "Dock PR details"
                            : "Float PR details"}
                          onclick={() => {
                            if (!expanded) expand(true);
                            toggle();
                          }}><PictureInPicture2 aria-hidden="true" /></Button
                        >
                      {/snippet}</Tooltip.Trigger
                    ><Tooltip.Content
                      >{floating
                        ? "Dock PR details"
                        : "Float PR details"}</Tooltip.Content
                    ></Tooltip.Root
                  >
                  <Tooltip.Root
                    ><Tooltip.Trigger
                      >{#snippet child({ props })}
                        <Button
                          {...props}
                          variant="ghost"
                          size="icon-sm"
                          aria-label="Inspect checks"
                          aria-expanded={expanded && checksOpen}
                          disabled={blocked}
                          onclick={() => {
                            const open = !expanded || !checksOpen;
                            if (!expanded) expand(true);
                            checksOpen = open;
                          }}><ListChecks aria-hidden="true" /></Button
                        >
                      {/snippet}</Tooltip.Trigger
                    ><Tooltip.Content>Inspect checks</Tooltip.Content
                    ></Tooltip.Root
                  >
                </Tooltip.Provider>
                <Button
                  variant="ghost"
                  size="icon-sm"
                  aria-label="Close details column"
                  onclick={onClose}><X aria-hidden="true" /></Button
                >
              </div>
            </header>
            <Collapsible.Content>
              {#if expanded}
                <div class="forge-inline__details">
                  <StoreForgePullRequest
                    {controller}
                    bind:editing
                    showChecks={checksOpen}
                  />
                </div>
              {/if}
            </Collapsible.Content>
          </Collapsible.Root>
        </section>
      {/snippet}
    </StoreDetachablePanel>
  {/key}
{/if}

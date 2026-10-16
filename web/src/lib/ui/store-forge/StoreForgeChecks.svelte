<script lang="ts">
  import { forgeMenuLayout } from "./menu-layout.js";
  import { Button } from "@lapismd/design-core/shadcn/button";
  import { ScrollArea } from "@lapismd/design-core/shadcn/scroll-area";
  import * as Menu from "@lapismd/design-core/shadcn/dropdown-menu";
  import RefreshCw from "@lucide/svelte/icons/refresh-cw";
  import ChevronDown from "@lucide/svelte/icons/chevron-down";
  import ChevronRight from "@lucide/svelte/icons/chevron-right";
  import ListFilter from "@lucide/svelte/icons/list-filter";
  import List from "@lucide/svelte/icons/list";
  import Settings2 from "@lucide/svelte/icons/settings-2";
  import StoreForgeChecksRing from "./StoreForgeChecksRing.svelte";
  import StoreForgeCheckRow from "./StoreForgeCheckRow.svelte";
  import { checkGroups } from "../../controller/forge/checks-presentation.svelte.js";
  import type { ForgeController } from "../../controller/forge/controller.svelte.js";
  let { controller }: { controller: ForgeController } = $props();
  const blocked = $derived(
    controller.busy ||
      Boolean(controller.pending) ||
      (controller.accounts.managed && !controller.ready),
  );
  const pr = $derived(controller.selected);
  const checks = $derived(
    controller.checks ?? (pr ? controller.ci[pr.head] : undefined),
  );
  const current = $derived(
    checks?.sha === pr?.sha && checks?.checks.every((c) => c.sha === pr?.sha),
  );
  const groups = $derived(checkGroups(checks?.checks ?? []));
  const status = $derived(
    !current || !checks?.complete || controller.stale
      ? "unknown"
      : checks.checks.some((c) => c.state === "failed")
        ? "failed"
        : checks.checks.some((c) => ["running", "queued"].includes(c.state))
          ? "running"
          : checks.checks.some((c) =>
                ["unknown", "manual", "cancelled"].includes(c.state),
              )
            ? "unknown"
            : checks.checks.some((c) => c.state === "passed")
              ? "passed"
              : "skipped",
  );
  const summary = $derived(
    !controller.data?.capabilities.checks
      ? "Checks unavailable"
      : !checks
        ? controller.stale
          ? "Checks unavailable"
          : "Loading checks…"
        : !current || controller.stale
          ? "Checks need refresh"
          : !checks.complete
            ? "Checks incomplete"
            : !checks.checks.length
              ? "No checks reported"
              : status === "failed"
                ? "Some checks were not successful"
                : status === "running"
                  ? "Checks are running"
                  : status === "passed"
                    ? "All checks have passed"
                    : status === "skipped"
                      ? "Checks skipped"
                      : "Checks need attention",
  );
  const counts = $derived(
    groups.map((g) => `${g.checks.length} ${g.label}`).join(" · "),
  );
  const view = $derived(controller.checksView);
  const id = $props.id();
  let viewport = $state<HTMLElement | null>(null);
  $effect(() => {
    const element = viewport;
    if (!element) return;
    element.scrollTop = view.scrollTop;
    const save = () => {
      view.scrollTop = element.scrollTop;
    };
    element.addEventListener("scroll", save, { passive: true });
    return () => element.removeEventListener("scroll", save);
  });
</script>

{#if pr && controller.data}
  <section class="forge-checks" aria-label="CI checks">
    <div class="forge-checks__summary">
      <Button
        variant="ghost"
        class="forge-checks__disclosure"
        aria-label="Toggle check results"
        aria-expanded={view.expanded}
        aria-controls={`${id}-results`}
        onclick={() => (view.expanded = !view.expanded)}
      >
        <StoreForgeChecksRing checks={checks?.checks ?? []} state={status} />
        <span class="forge-checks__name"
          ><span role="heading" aria-level={3}
            >{controller.activeRun
              ? `Jobs for run ${controller.activeRun}`
              : summary}</span
          ><span>{counts}</span></span
        >
      </Button>
      <Button
        variant="ghost"
        size="icon-xs"
        aria-label="Refresh checks"
        disabled={blocked || !controller.data.capabilities.checks}
        onclick={() => controller.refreshChecks()}
        ><RefreshCw aria-hidden="true" /></Button
      >
      <Menu.Root
        ><Menu.Trigger
          >{#snippet child({ props })}<Button
              {...props}
              variant="ghost"
              size="icon-xs"
              aria-label="Check display options"
              ><Settings2 aria-hidden="true" /></Button
            >{/snippet}</Menu.Trigger
        >
        <Menu.Content align="end" preventScroll={false} style={forgeMenuLayout}
          ><Menu.RadioGroup
            value={view.grouped ? "status" : "none"}
            onValueChange={(value) => (view.grouped = value === "status")}
          >
            <Menu.RadioItem value="status"
              ><ListFilter aria-hidden="true" />Group by status</Menu.RadioItem
            ><Menu.RadioItem value="none"
              ><List aria-hidden="true" />No grouping</Menu.RadioItem
            >
          </Menu.RadioGroup></Menu.Content
        >
      </Menu.Root>
      <Button
        variant="ghost"
        size="icon-xs"
        class="forge-checks__chevron"
        aria-label={view.expanded ? "Collapse checks" : "Expand checks"}
        aria-expanded={view.expanded}
        aria-controls={`${id}-results`}
        onclick={() => (view.expanded = !view.expanded)}
        >{#if view.expanded}<ChevronDown
            aria-hidden="true"
          />{:else}<ChevronRight aria-hidden="true" />{/if}</Button
      >
    </div>
    <div id={`${id}-results`} hidden={!view.expanded}>
      {#if checks}
        <p class="forge-checks__revision">
          {current ? "Revision" : "Observed revision"}
          <code>{checks.sha.slice(0, 8)}</code>{checks.complete
            ? ""
            : " · Results incomplete"}
        </p>
        {#if controller.activeRun}<Button
            variant="ghost"
            size="xs"
            disabled={blocked}
            onclick={() => controller.act({ kind: "checks" })}
            >Back to all checks</Button
          >{/if}
        <ScrollArea
          scrollStrategy="styled"
          bind:viewportRef={viewport}
          class="forge-checks__scroll"
          aria-label="Check results"
        >
          {#if view.grouped}
            {#each groups as group (group.id)}
              <div class="forge-checks__group">
                <Button
                  variant="ghost"
                  size="sm"
                  class="forge-checks__group-heading"
                  aria-expanded={!view.collapsed[group.id]}
                  aria-controls={`${id}-${group.id}`}
                  onclick={() =>
                    (view.collapsed[group.id] = !view.collapsed[group.id])}
                >
                  {group.checks.length}
                  {group.label}
                  {group.checks.length === 1 ? "check" : "checks"}
                  {#if view.collapsed[group.id]}<ChevronRight
                      aria-hidden="true"
                    />{:else}<ChevronDown aria-hidden="true" />{/if}
                </Button>
                <ul
                  id={`${id}-${group.id}`}
                  hidden={view.collapsed[group.id]}
                  class="forge-checks__list"
                >
                  {#each group.checks as check (check.id)}<StoreForgeCheckRow
                      {controller}
                      {check}
                    />{/each}
                </ul>
              </div>
            {/each}
          {:else}<ul class="forge-checks__list">
              {#each checks.checks as check (check.id)}<StoreForgeCheckRow
                  {controller}
                  {check}
                />{/each}
            </ul>{/if}
        </ScrollArea>
      {/if}
      {#if controller.logs}<pre
          aria-label="Job logs">{controller.logs}</pre>{/if}
    </div>
  </section>
{/if}

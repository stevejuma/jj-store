<script lang="ts">
  import { Button } from "@lapismd/design-core/shadcn/button";
  import RefreshCw from "@lucide/svelte/icons/refresh-cw";
  import StoreForgeCheckIcon from "./StoreForgeCheckIcon.svelte";
  import StoreForgeCheckRow from "./StoreForgeCheckRow.svelte";
  import type { ForgeController } from "../../controller/forge/controller.svelte.js";
  let { controller }: { controller: ForgeController } = $props();
  const blocked = $derived(controller.busy || Boolean(controller.pending));
  const pr = $derived(controller.selected);
  const checks = $derived(
    controller.checks ?? (pr ? controller.ci[pr.head] : undefined),
  );
  const current = $derived(
    checks?.sha === pr?.sha &&
      checks?.checks.every((check) => check.sha === pr?.sha),
  );
  const state = $derived(
    !current || !checks?.complete
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
    !checks
      ? controller.stale
        ? "Checks unavailable"
        : "Loading checks…"
      : !current
        ? "Checks need refresh"
        : !checks.complete
          ? "Checks incomplete"
          : !checks.checks.length
            ? "No checks reported"
            : state === "failed"
              ? "Checks failed"
              : state === "running"
                ? "Checks running"
                : state === "passed"
                  ? "Checks passed"
                  : state === "skipped"
                    ? "Checks skipped"
                    : "Checks need attention",
  );
  const counts = $derived(
    checks
      ? [
          [
            checks.checks.filter((c) => c.state === "passed").length,
            "successful",
          ],
          [
            checks.checks.filter((c) => ["running", "queued"].includes(c.state))
              .length,
            "in progress",
          ],
          [checks.checks.filter((c) => c.state === "failed").length, "failed"],
          [
            checks.checks.filter((c) =>
              ["unknown", "manual", "cancelled", "skipped"].includes(c.state),
            ).length,
            "other",
          ],
        ]
          .filter(([count]) => count)
          .map(([count, label]) => `${count} ${label}`)
          .join(" · ")
      : "",
  );
</script>

{#if pr && controller.data}
  <section class="forge-checks" aria-label="CI checks">
    <div class="forge-checks__summary">
      <StoreForgeCheckIcon {state} />
      <div class="forge-checks__name">
        <h3>
          {controller.activeRun
            ? `Jobs for run ${controller.activeRun}`
            : controller.data.capabilities.checks
              ? summary
              : "Checks unavailable"}
        </h3>
        <p>
          {controller.data.capabilities.checks
            ? counts
            : "This forge does not expose checks."}
        </p>
      </div>
      <Button
        variant="ghost"
        size="icon-sm"
        aria-label="Refresh checks"
        disabled={blocked || !controller.data.capabilities.checks}
        onclick={() => controller.act({ kind: "checks" })}
        ><RefreshCw aria-hidden="true" /></Button
      >
    </div>
    {#if checks}
      <p class="forge-checks__revision">
        {current ? "Revision" : "Observed revision"}
        <code>{checks.sha.slice(0, 8)}</code>{checks.complete
          ? ""
          : " · Results incomplete"}
      </p>
      {#if controller.activeRun}<Button
          variant="ghost"
          size="sm"
          disabled={blocked}
          onclick={() => controller.act({ kind: "checks" })}
          >Back to all checks</Button
        >{/if}
      <ul class="forge-checks__list">
        {#each checks.checks as check (check.id)}<StoreForgeCheckRow
            {controller}
            {check}
          />{/each}
      </ul>
    {/if}
    {#if controller.logs}<pre aria-label="Job logs">{controller.logs}</pre>{/if}
  </section>
{/if}

<script lang="ts">
  import { Button } from "@lapismd/design-core/shadcn/button";
  import type { ForgeController } from "../../controller/forge/controller.svelte.js";
  let { controller }: { controller: ForgeController } = $props();
  const blocked = $derived(controller.busy || Boolean(controller.pending));
  import type { ForgeCheck } from "../../controller/forge/types.js";
  function jobs(check: ForgeCheck) {
    if (check.run_id) void controller.act({ kind: "jobs", run: check.run_id });
  }
</script>

{#if controller.checks && controller.data}<section aria-label="CI checks">
    <h3>
      {controller.activeRun
        ? `Jobs for run ${controller.activeRun}`
        : "Checks and runs"}
    </h3>
    {#if !controller.checks.checks.length}<p>
        No checks reported for this revision.
      </p>{/if}
    <ul>
      {#each controller.checks.checks as check}<li>
          <strong>{check.name}</strong> · {check.state}
          {#if check.url}<a href={check.url} target="_blank" rel="noreferrer"
              >Open on forge</a
            >{/if}
          {#if check.log_id}<Button
              variant="ghost"
              size="sm"
              disabled={blocked || !controller.data.capabilities.logs}
              onclick={() =>
                controller.act({
                  kind: "logs",
                  run: check.run_id ?? controller.activeRun,
                  job: check.log_id!,
                })}>View logs</Button
            >
          {:else if check.run_id}<Button
              variant="ghost"
              size="sm"
              disabled={blocked}
              onclick={() => jobs(check)}>Jobs</Button
            >
            {#if controller.data.capabilities.rerun}{#if controller.data.capabilities.rerun_all}<Button
                  variant="ghost"
                  size="sm"
                  disabled={blocked}
                  onclick={() =>
                    controller.act({
                      kind: "rerun",
                      run: check.run_id!,
                      failed_only: false,
                    })}>Rerun all</Button
                >{/if}<Button
                variant="ghost"
                size="sm"
                disabled={blocked}
                onclick={() =>
                  controller.act({
                    kind: "rerun",
                    run: check.run_id!,
                    failed_only: true,
                  })}>Rerun failed</Button
              >{/if}
            {#if controller.data.capabilities.cancel && ["queued", "running"].includes(check.state)}<Button
                variant="ghost"
                size="sm"
                disabled={blocked}
                onclick={() =>
                  controller.act({ kind: "cancel", run: check.run_id! })}
                >Cancel run</Button
              >{/if}
          {/if}
        </li>{/each}
    </ul>
  </section>{/if}

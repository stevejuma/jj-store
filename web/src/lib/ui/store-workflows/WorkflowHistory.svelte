<script lang="ts">
  import { onDestroy } from "svelte";
  import * as Dialog from "@lapismd/design-core/shadcn/dialog";
  import { DialogFrame } from "@lapismd/design-core/forms/dialog";
  import { Button } from "@lapismd/design-core/shadcn/button";
  import CommunityChip from "../community-chip/CommunityChip.svelte";
  import StoreNotice from "../store-notifications/StoreNotice.svelte";
  import StoreWorkEmpty from "../store-work/StoreWorkEmpty.svelte";
  import type { WorkflowController } from "../../controller/workflows.svelte.js";
  import type {
    WorkflowVersion,
    WorkflowPage,
  } from "../../controller/workflow-types.js";
  let {
    work,
    workflow,
    writable = true,
    onClose,
  }: {
    work: WorkflowController;
    workflow: WorkflowVersion;
    writable?: boolean;
    onClose(): void;
  } = $props();
  let page = $state<WorkflowPage | null>(null),
    detail = $state<WorkflowPage | null>(null),
    selected = $state(""),
    pin = $state<number | bigint | undefined>(),
    after = $state<string | undefined>(),
    error = $state(""),
    loading = $state(false);
  const run = $derived(detail?.runs[0]),
    attempt = $derived(run?.attempts.at(-1));
  let active = true;
  async function read(more = false) {
    loading = true;
    try {
      const next = await work.transport.page({
        workflow: workflow.id,
        limit: 50,
        ...(more && after ? { after } : {}),
      });
      if (active) {
        page =
          more && page ? { ...next, runs: [...page.runs, ...next.runs] } : next;
        after = next.next ?? undefined;
        error = "";
      }
    } catch (cause) {
      if (active)
        error = cause instanceof Error ? cause.message : "History unavailable";
    } finally {
      if (active) loading = false;
    }
  }
  async function inspect(id: string, frozen?: number | bigint) {
    try {
      const next = await work.transport.page({
        run: id,
        at: frozen,
        limit: 256,
      });
      if (active && selected === id) {
        detail = next;
        error = "";
      }
    } catch (cause) {
      if (active)
        error = cause instanceof Error ? cause.message : "Run unavailable";
    }
  }
  async function select(id: string) {
    selected = id;
    detail = null;
    const item = page?.runs.find((r) => r.id === id);
    pin =
      item?.attempts.at(-1)?.status &&
      ["completed", "failed", "cancelled"].includes(
        item.attempts.at(-1)!.status,
      )
        ? page?.position
        : undefined;
    await inspect(id, pin);
  }
  async function moreEvidence() {
    if (!detail?.next || !selected) return;
    try {
      const next = await work.transport.page({
        run: selected,
        at: detail.position,
        after: detail.next,
        limit: 256,
      });
      if (selected === next.runs[0]?.id)
        detail = { ...next, evidence: [...detail.evidence, ...next.evidence] };
    } catch (cause) {
      error = cause instanceof Error ? cause.message : "Evidence unavailable";
    }
  }
  async function command(kind: "cancel" | "retry") {
    if (!run || !attempt) return;
    await work.submit(
      kind === "cancel"
        ? { type: "cancel", run: run.id }
        : { type: "retry", run: run.id, expected: attempt.id },
    );
    await read();
    pin = undefined;
    await inspect(selected);
  }
  function download(
    name: string,
    mediaType: string,
    bytes: readonly (number | bigint)[],
  ) {
    const blob = new Blob([new Uint8Array(bytes.map(Number))], {
      type: mediaType,
    });
    const url = URL.createObjectURL(blob);
    const link = document.createElement("a");
    link.href = url;
    link.download = name.split("/").at(-1) ?? "artifact";
    link.click();
    setTimeout(() => URL.revokeObjectURL(url), 1000);
  }
  void read();
  const timer = setInterval(() => {
    if (selected && pin === undefined) void inspect(selected);
    void read();
  }, 3000);
  onDestroy(() => {
    active = false;
    clearInterval(timer);
  });
</script>

<Dialog.Root
  open
  onOpenChange={(open) => {
    if (!open) onClose();
  }}
  ><DialogFrame
    size="wide"
    title={`Run history · ${workflow.definition.name}`}
    description="Inspect the recorded definition, source, steps and outputs."
  >
    <StoreNotice
      noticeKey="workflow-history"
      title="Workflow history unavailable"
      message={error}
    />
    <div class="workflow-history">
      {#if !page && !loading}<StoreWorkEmpty
          kind="activity"
          title="No recorded runs"
          description="Run this workflow or wait for a matching event."
        />{/if}
      {#each page?.runs ?? [] as item (item.id)}{@const current =
          item.attempts.at(-1)!}
        <div class="workflow-history-row">
          <Button
            variant="ghost"
            onclick={() => select(item.id)}
            aria-pressed={selected === item.id}
            >{new Date(Number(item.created_at)).toLocaleString()} · {item.id.slice(
              0,
              8,
            )}</Button
          ><CommunityChip
            size="compact"
            tone={current.status === "completed"
              ? "success"
              : current.status === "failed"
                ? "danger"
                : "neutral"}
            >{current.status.replaceAll("_", " ")}</CommunityChip
          >
        </div>{/each}
      {#if after}<Button
          variant="outline"
          disabled={loading}
          onclick={() => read(true)}>More runs</Button
        >{/if}
      {#if run && attempt}<section
          class="workflow-run-detail"
          aria-label="Run detail"
        >
          <div class="workflow-run-actions">
            <CommunityChip
              tone={attempt.status === "completed"
                ? "success"
                : attempt.status === "failed"
                  ? "danger"
                  : "neutral"}
              >{attempt.status.replaceAll("_", " ")}</CommunityChip
            ><Button
              variant="outline"
              onclick={() => {
                pin = detail!.position;
              }}
              disabled={pin !== undefined}>Pin this history</Button
            >{#if pin !== undefined}<Button
                variant="outline"
                onclick={() => {
                  pin = undefined;
                  void inspect(selected);
                }}>Follow latest</Button
              >{/if}{#if writable && !["completed", "failed", "cancelled"].includes(attempt.status)}<Button
                variant="outline"
                disabled={work.state.writing ||
                  !!work.state.pending ||
                  attempt.status === "cancel_requested"}
                onclick={() => command("cancel")}>Cancel run</Button
              >{:else if writable}<Button
                variant="outline"
                disabled={work.state.writing || !!work.state.pending}
                onclick={() => command("retry")}>Retry run</Button
              >{/if}
          </div>
          <dl>
            <dt>Host</dt>
            <dd>{attempt.host ?? "Waiting for an eligible host"}</dd>
            <dt>Definition</dt>
            <dd>{run.version}</dd>
            <dt>Repository</dt>
            <dd>{run.source.repository}</dd>
            {#if run.source.conversation}
              <dt>Thread</dt>
              <dd>{run.source.conversation.thread}</dd>
              {#if run.source.conversation.message}<dt>Message</dt>
                <dd>{run.source.conversation.message}</dd>{/if}
              <dt>Content revision</dt>
              <dd>{run.source.conversation.revision.id}</dd>
              <dt>Source publication</dt>
              <dd>{run.source.conversation.position}</dd>
            {:else}
              <dt>Operation</dt>
              <dd>{run.source.operation}</dd>
              <dt>Commit</dt>
              <dd>{run.source.commit}</dd>
            {/if}
          </dl>
          {#each detail?.evidence ?? [] as evidence (evidence.id)}
            {#if evidence.data.type === "step"}<div>
                <strong>{evidence.data.step}</strong> · {evidence.data.status.replaceAll(
                  "_",
                  " ",
                )}
              </div>
            {:else if evidence.data.type === "log"}<pre
                aria-label={`${evidence.data.stream} log`}>{evidence.data
                  .text}</pre>
            {:else if evidence.data.type === "artifact"}<Button
                variant="outline"
                onclick={() =>
                  download(
                    evidence.data.type === "artifact" ? evidence.data.name : "",
                    evidence.data.type === "artifact"
                      ? evidence.data.media_type
                      : "",
                    evidence.data.type === "artifact"
                      ? evidence.data.bytes
                      : [],
                  )}>Download {evidence.data.name}</Button
              >
            {:else if evidence.data.type === "agent"}<a
                href={`#/agents/sessions/${encodeURIComponent(evidence.data.session)}`}
                >Agent transcript · {evidence.data.session.slice(0, 8)}</a
              >{/if}
          {/each}
          {#if detail?.next}<Button variant="outline" onclick={moreEvidence}
              >More evidence</Button
            >{/if}
          {#if attempt.result}<p>{attempt.result}</p>{/if}
        </section>{/if}
    </div>
  </DialogFrame></Dialog.Root
>

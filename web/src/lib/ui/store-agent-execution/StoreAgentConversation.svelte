<script lang="ts">
  import { untrack } from "svelte";
  import { Button } from "@lapismd/design-core/shadcn/button";
  import { Input } from "@lapismd/design-core/shadcn/input";
  import { Textarea } from "@lapismd/design-core/shadcn/textarea";
  import {
    Layout,
    Composer,
    MessageList,
    MessageBubble,
  } from "@lapismd/design-core/ai/chat";
  import { CodeBlock } from "@lapismd/design-core/shadcn/code-block";
  import CommunityMarkdownContent from "../community-markdown-content/CommunityMarkdownContent.svelte";
  import StoreAgentPermission from "./StoreAgentPermission.svelte";
  import {
    activityRows,
    runtimeChoices,
    queuedRequests,
  } from "./presentation.js";
  import type { AgentExecution } from "../../controller/agent-execution.svelte.js";
  import { object, text } from "../../controller/agent-execution-types.js";
  let {
    manager,
    onReview,
  }: {
    manager: AgentExecution;
    onReview?: (repository: string, commit: string, operation: string) => void;
  } = $props();
  const formId = $props.id();
  const session = $derived(manager.session);
  const facts = $derived(object(session?.capabilities));
  const negotiated = $derived(object(facts.session));
  const modes = $derived(runtimeChoices(negotiated, "mode"));
  const models = $derived(runtimeChoices(negotiated, "model"));
  const efforts = $derived(runtimeChoices(negotiated, "thought_level"));
  const rows = $derived(activityRows(manager.events));
  const queued = $derived(queuedRequests(manager.events));
  const active = $derived(
    [...manager.events]
      .reverse()
      .find((e) => object(e.payload).type === "started"),
  );
  let model = $state("");
  let effort = $state("");
  let mode = $state("");
  $effect(() => {
    manager.selected;
    untrack(() => {
      const current = manager.session;
      if (current) {
        model = current.preferences.model ?? "";
        effort = current.preferences.effort ?? "";
        mode = current.preferences.mode ?? "";
      }
    });
  });
  const idle = $derived(
    session?.state === "idle" || session?.state === "running",
  );
  async function send() {
    const id = manager.selected;
    const draft = manager.drafts[id];
    if (draft?.trim())
      await manager.command(
        { action: "prompt", text: draft, max_duration_ms: 900000 },
        id,
      );
  }
</script>

{#if session}
  <div class="agent-conversation">
    <header>
      <div>
        <h3>{session.avatar} {session.title}</h3>
        <p role="status">
          {session.runner} · {session.state} · Published {session.published_sequence}/{session.last_sequence}
        </p>
      </div>
      <div class="agent-execution-actions">
        {#if idle}<Button
            variant="outline"
            size="sm"
            disabled={manager.busy || !!manager.pending}
            onclick={() => manager.command({ action: "end" })}
            >End session</Button
          >{:else if session.state !== "cleaned" && session.state !== "preparing" && session.state !== "opening"}
          {#if object(facts.initialize).loadSession === true}<Button
              size="sm"
              disabled={manager.busy || !!manager.pending}
              onclick={() => manager.command({ action: "resume" })}
              >Resume session</Button
            >{/if}
          <Button
            variant="outline"
            size="sm"
            disabled={manager.busy || !!manager.pending}
            onclick={() => manager.command({ action: "continue_with_context" })}
            >New session with saved context</Button
          >
        {/if}
      </div>
    </header>
    <details>
      <summary>Workspace and runtime</summary>
      <div class="agent-execution-form">
        <p class="agent-execution-meta">Workspace: {session.workspace}</p>
        <p class="agent-execution-meta">
          Starting revision: {session.base_commit}
        </p>
        <div class="agent-execution-actions">
          <Button
            variant="outline"
            size="sm"
            onclick={() =>
              onReview?.(
                session.repository,
                session.review_commit,
                session.operation,
              )}>Review changes</Button
          >
          {#if !idle && session.state !== "cleaned"}<Button
              variant="outline"
              size="sm"
              disabled={manager.busy || !!manager.pending}
              onclick={() => manager.command({ action: "cleanup" })}
              >Clean up workspace</Button
            >{/if}
        </div>
        <p>
          Cleanup retains a JJ bookmark and refuses ignored or untracked files.
        </p>
        <label for={`${formId}-model`}>Model</label><Input
          id={`${formId}-model`}
          bind:value={model}
          list={`${formId}-models`}
        /><datalist id={`${formId}-models`}
          >{#each models as option}<option value={option.id}
              >{option.name}</option
            >{/each}</datalist
        >
        <label for={`${formId}-effort`}>Effort</label><Input
          id={`${formId}-effort`}
          bind:value={effort}
          list={`${formId}-efforts`}
        /><datalist id={`${formId}-efforts`}
          >{#each efforts as option}<option value={option.id}
              >{option.name}</option
            >{/each}</datalist
        >
        <label for={`${formId}-mode`}>Execution mode</label><select
          id={`${formId}-mode`}
          bind:value={mode}
          ><option value="">Select a mode</option>{#each modes as option}<option
              value={option.id}>{option.name}</option
            >{/each}</select
        >
        <Button
          size="sm"
          disabled={!idle || !mode || manager.busy || !!manager.pending}
          onclick={() =>
            manager.command({
              action: "configure",
              preferences: {
                model: model || null,
                effort: effort || null,
                mode: mode || null,
              },
            })}>Apply runtime preferences</Button
        >
        {#if facts.configuration}<CodeBlock
            code={JSON.stringify(facts.configuration, null, 2)}
            language="json"
            title="Applied preferences"
            size="sm"
          />{/if}
      </div>
    </details>
    {#if session.last_error}<p role="status">
        {session.last_error.replaceAll("_", " ")}. Workspace and saved history
        are retained.
      </p>{/if}
    <Layout
      conversationKey={session.id}
      aria-label="Agent conversation"
      isEmpty={!rows.length}
    >
      {#snippet composer()}<Composer
          onSubmit={() => void send()}
          disabled={manager.busy ||
            !!manager.pending ||
            !idle ||
            facts.modeReady !== true}
        >
          {#snippet input()}<Textarea
              aria-label="Message to agent"
              placeholder={session.state === "running"
                ? "Queue a follow-up…"
                : "Message the agent…"}
              value={manager.drafts[session.id] ?? ""}
              oninput={(event) =>
                (manager.drafts[session.id] = event.currentTarget.value)}
              disabled={manager.busy || !!manager.pending || !idle}
              rows={3}
              onkeydown={(event) => {
                if (event.key === "Enter" && (event.metaKey || event.ctrlKey)) {
                  event.preventDefault();
                  if (facts.modeReady === true) void send();
                }
              }}
            />{/snippet}
          {#snippet sendButton()}<div class="agent-execution-actions">
              {#if session.state === "running" && active}<Button
                  variant="outline"
                  size="sm"
                  disabled={manager.busy || !!manager.pending}
                  onclick={() =>
                    manager.command({
                      action: "cancel",
                      target: text(object(active.payload).requestId),
                    })}>Stop turn</Button
                >{/if}<Button
                disabled={manager.busy ||
                  !!manager.pending ||
                  !idle ||
                  facts.modeReady !== true ||
                  !manager.drafts[session.id]?.trim()}
                onclick={() => void send()}
                >{session.state === "running"
                  ? "Queue follow-up"
                  : "Send"}</Button
              >
            </div>{/snippet}
          {#snippet footerActions()}{#if facts.modeReady !== true}<span
                >Choose and apply an execution mode before sending.</span
              >{/if}{/snippet}
        </Composer>{/snippet}
      <MessageList
        latestMessageId={rows.at(-1)?.id}
        isStreaming={session.state === "running"}
      >
        {#each rows as row (row.id)}
          {#if row.kind === "user" || row.kind === "assistant"}<MessageBubble
              sender={row.kind === "user" ? "user" : "assistant"}
              ><CommunityMarkdownContent content={row.text} />
              {#if row.kind === "user" && queued.has(row.request) && idle}<Button
                  variant="outline"
                  size="sm"
                  disabled={manager.busy || !!manager.pending}
                  onclick={() =>
                    manager.command({ action: "cancel", target: row.request })}
                  >Cancel queued prompt</Button
                >{/if}
            </MessageBubble>
          {:else if row.kind === "permission" && session.state === "running"}<StoreAgentPermission
              {manager}
              value={row.value}
              title={row.text}
            />
          {:else if row.kind === "tool"}<details>
              <summary
                >{row.text || "Tool call"} · {text(row.value.status)}</summary
              ><CodeBlock
                code={JSON.stringify(row.value, null, 2)}
                language="json"
                title="Tool activity"
                size="sm"
                width="full"
                isWrapped
                maxHeight="20rem"
              />
            </details>
          {:else if row.kind === "plan"}<section aria-label="Agent plan">
              <h4>Plan</h4>
              <CodeBlock
                code={JSON.stringify(row.value.entries ?? [], null, 2)}
                language="json"
                size="sm"
                width="full"
                isWrapped
              />
            </section>
          {:else if row.kind === "record"}<p role="status">
              Turn {row.text.replaceAll("_", " ")}
            </p>{/if}
        {/each}
      </MessageList>
    </Layout>
  </div>
{/if}

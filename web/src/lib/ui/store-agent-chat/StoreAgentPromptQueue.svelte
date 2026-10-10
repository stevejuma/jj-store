<script lang="ts">
  import * as Collapsible from "@lapismd/design-core/shadcn/collapsible";
  import XIcon from "@lucide/svelte/icons/x";
  import RefreshIcon from "@lucide/svelte/icons/refresh-cw";
  import CommunityMemberAvatar from "../community-member-avatar/CommunityMemberAvatar.svelte";
  import QueuedPromptText from "./QueuedPromptText.svelte";
  import "../store-agent-chat-body/comment-card.css";
  import ChatDisclosureIcon from "../store-agent-chat-body/ChatDisclosureIcon.svelte";
  import { Button } from "@lapismd/design-core/shadcn/button";
  import type { AgentExecution } from "../../controller/agent-execution.svelte.js";
  import type { AgentSession } from "../../controller/agent-execution-types.js";
  let {
    manager,
    sessions,
  }: { manager: AgentExecution; sessions: readonly AgentSession[] } = $props();
  const entries = $derived(manager.promptQueue.entries(sessions));
  const key = $derived(
    sessions
      .map((s) => s.id)
      .sort()
      .join(":"),
  );
  const open = $derived(manager.promptQueue.disclosures[key] ?? true);
</script>

{#if entries.length}
  <section class="prompt-queue" aria-label="Queued messages">
    <Collapsible.Root
      {open}
      onOpenChange={(value) => (manager.promptQueue.disclosures[key] = value)}
    >
      <Collapsible.Trigger
        class="chat-context-trigger prompt-queue__heading"
        aria-label={`${open ? "Collapse" : "Expand"} ${entries.length} queued ${entries.length === 1 ? "message" : "messages"}`}
      >
        <span
          >{entries.length} queued {entries.length === 1
            ? "message"
            : "messages"}</span
        >
        <ChatDisclosureIcon />
      </Collapsible.Trigger>
      <Collapsible.Content>
        <ol>
          {#each entries as entry (entry.submission.id)}
            {@const author = entry.submission.author ??
              entry.submission.comments[0]?.author ?? {
                id: "local-user",
                name: "You",
                initials: "YO",
                handle: "",
              }}
            <li>
              <div class="chat-comment-list">
                <article
                  class="chat-comment-card"
                  aria-label={`Pending message from ${author.name}`}
                >
                  <CommunityMemberAvatar
                    member={author}
                    size="xsmall"
                    contrast="strong"
                  />
                  <div class="prompt-queue__content">
                    {#if entry.text}
                      <QueuedPromptText
                        text={entry.text}
                        expanded={manager.promptQueue.disclosures[
                          `prompt:${entry.submission.id}`
                        ] ?? false}
                        onExpandedChange={(value) =>
                          (manager.promptQueue.disclosures[
                            `prompt:${entry.submission.id}`
                          ] = value)}
                      />
                    {/if}
                    {#each entry.submission.comments as comment (comment.commentId)}
                      <blockquote>
                        <span>{comment.author.name}: </span>{comment.text}
                      </blockquote>
                    {/each}
                  </div>
                  <div class="chat-comment-links">
                    {#each entry.recipients as recipient (recipient.request)}
                      {@const key = `${recipient.session}:${recipient.request}`}
                      {#if recipient.state === "queued"}<Button
                          variant="ghost"
                          size="icon-xs"
                          disabled={manager.promptQueue.cancelling[key] ||
                            manager.busy ||
                            !!manager.pending}
                          aria-label={`Cancel queued message for ${recipient.label}`}
                          title={manager.promptQueue.cancelling[key]
                            ? "Cancelling…"
                            : `Cancel queued message for ${recipient.label}`}
                          onclick={() =>
                            void manager.promptQueue.cancel(
                              recipient.session,
                              recipient.request,
                            )}><XIcon /></Button
                        >{/if}
                      {#if recipient.state !== "queued" || manager.promptQueue.errors[key]}<Button
                          variant="ghost"
                          size="icon-xs"
                          aria-label="Check message status"
                          title={`Check message status for ${recipient.label}`}
                          onclick={() =>
                            void manager.promptQueue.review(
                              recipient.session,
                              recipient.request,
                            )}><RefreshIcon /></Button
                        >{/if}
                    {/each}
                  </div>
                </article>
              </div>
              <div class="prompt-queue__metadata chat-comment-status">
                {#each entry.recipients as recipient (recipient.request)}
                  {@const key = `${recipient.session}:${recipient.request}`}
                  <div>
                    {recipient.label} · {recipient.state === "queued"
                      ? "Queued"
                      : "Delivery needs attention"}
                  </div>
                  {#if manager.promptQueue.errors[key]}<p role="alert">
                      {manager.promptQueue.errors[key]}
                    </p>{/if}
                {/each}
              </div>
            </li>
          {/each}
        </ol>
      </Collapsible.Content>
    </Collapsible.Root>
  </section>
{/if}

<style>
  .prompt-queue {
    margin-block-end: 0.75rem;
    padding-inline: 0.75rem;
  }
  ol {
    display: grid;
    gap: 0.5rem;
    list-style: none;
    margin-block: 0.5rem 0;
    padding: 0;
  }
  li {
    opacity: 0.7;
  }
  .prompt-queue__content {
    display: flex;
    flex-direction: column;
    justify-content: center;
    flex: 1;
    min-width: 0;
    min-height: 1.5rem;
    font-family: var(--font-sans, system-ui);
    font-size: var(--conversation-prose-size, 0.875rem);
    line-height: 1.65;
  }
  blockquote {
    white-space: pre-wrap;
    overflow-wrap: anywhere;
    margin: 0;
  }
  blockquote {
    margin-block-start: 0.25rem;
    border-inline-start: 2px solid var(--border);
    padding-inline-start: 0.5rem;
  }
  blockquote:first-child {
    margin-block-start: 0;
  }
  .prompt-queue__metadata {
    color: var(--foreground);
  }
  .prompt-queue__metadata p {
    margin: 0;
  }
</style>

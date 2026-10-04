<script lang="ts">
  import { sectionDisclosure } from "../disclosure-layout.svelte.js";
  // CommunityProjectsAgentContextDisclosure (Apache-2.0), native sanitized capture.
  import * as Collapsible from "@lapismd/design-core/shadcn/collapsible";
  import ChevronRightIcon from "@lucide/svelte/icons/chevron-right";
  import type { NativeAgentContext } from "../../controller/native-activity-types.js";
  import type { StoreController } from "../../controller/contracts.js";
  import {
    agentMember,
    agentSessionTiming,
  } from "../../collaboration/presentation.js";
  import CommunityChip from "../community-chip/CommunityChip.svelte";
  import StoreIdentityCopy from "./StoreIdentityCopy.svelte";
  import StoreAgentTranscript from "./StoreAgentTranscript.svelte";
  let {
    context,
    controller,
    patches = {},
  }: {
    context: NativeAgentContext;
    controller: StoreController;
    patches?: Readonly<Record<string, string>>;
  } = $props();
  const disclosure = sectionDisclosure(
    () => [
      "review",
      controller.state.reviewThread?.thread.thread_id ?? "",
      "agent-context",
    ],
    false,
  );
  let resolved = $state<NativeAgentContext>({ sessions: [], error: "" });
  let loading = $state(false);
  $effect(() => {
    const initial = context;
    let current = true;
    resolved = initial;
    if (initial.discover)
      void initial.discover().then((value) => {
        if (current) resolved = value;
      });
    return () => {
      current = false;
    };
  });
  $effect(() => {
    const load = disclosure.open ? resolved.load : undefined;
    if (!load) return;
    let current = true;
    loading = true;
    void load().then((value) => {
      if (current) {
        resolved = value;
        loading = false;
      }
    });
    return () => {
      current = false;
      loading = false;
    };
  });
</script>

{#if resolved.sessions.length || resolved.relatedSessionIds?.length || resolved.error}
  <Collapsible.Root
    bind:open={disclosure.open}
    class="community-projects-agent-context-disclosure"
    data-ui-component="community-projects-agent-context-disclosure"
  >
    <div class="community-projects-agent-context-disclosure__header">
      <Collapsible.Trigger
        class="community-projects-agent-context-disclosure__trigger"
        ><ChevronRightIcon aria-hidden="true" /><span>Agent context</span
        ></Collapsible.Trigger
      >
      {#each resolved.sessions as session (session.session.session_id)}
        {@const timing = agentSessionTiming(session.session)}
        {#if timing}<CommunityChip
            variant="subtle"
            size="compact"
            ariaLabel={`Session duration ${timing.durationLabel}`}
            >{timing.durationLabel}</CommunityChip
          >{/if}
        <StoreIdentityCopy
          value={session.session.session_id}
          label="agent session ID"
        />
      {/each}
    </div>
    <Collapsible.Content>
      {#if disclosure.open}
        {#if loading}<p role="status">Loading agent context…</p>{/if}
        {#if resolved.error}<p role="status">{resolved.error}</p>{/if}
        {#each resolved.sessions as session (session.session.session_id)}
          <section aria-label={`${agentMember(session.session).name} context`}>
            {#if session.activity_truncated || session.code_truncated}<p
                role="status"
              >
                This bounded capture omits older activity or code associations.
              </p>{/if}
            <StoreAgentTranscript
              activity={session.activity}
              sessionId={session.session.session_id}
              agent={agentMember(session.session)}
              member={{
                ...controller.state.member,
                kind:
                  controller.state.member.kind === "agent"
                    ? "agent"
                    : undefined,
              }}
              {controller}
              {patches}
              interactive={false}
            />
          </section>
        {/each}
      {/if}
    </Collapsible.Content>
  </Collapsible.Root>
{/if}

<style>
  :global(.community-projects-agent-context-disclosure) {
    overflow: hidden;
    border: 1px solid var(--border);
    border-radius: var(--radius);
    background: var(--background);
  }
  :global(.community-projects-agent-context-disclosure__trigger) {
    display: flex;
    flex: 1;
    min-width: 0;
    min-height: 2.75rem;
    align-items: center;
    gap: 0.5rem;
    padding: 0.65rem 0.75rem;
    text-align: start;
    font-size: var(--community-text-secondary);
    font-weight: 600;
    background: transparent;
  }
  .community-projects-agent-context-disclosure__header {
    display: flex;
    flex-wrap: wrap;
    align-items: center;
    gap: 0.35rem;
    padding-inline-end: 0.4rem;
    background: var(--background);
  }
  :global(
    .community-projects__activity-item:hover
      .community-projects-agent-context-disclosure__header
  ),
  :global(
    .community-projects-agent-context-disclosure[data-state="open"]
      .community-projects-agent-context-disclosure__header
  ) {
    background: color-mix(in srgb, var(--muted) 94%, var(--foreground));
  }
  :global(.community-projects-agent-context-disclosure__trigger:focus-visible) {
    outline: 2px solid var(--ring);
    outline-offset: -2px;
  }
  :global(.community-projects-agent-context-disclosure__trigger svg) {
    width: 1rem;
    height: 1rem;
    flex-shrink: 0;
    transition: transform 120ms ease;
  }
  :global(
    .community-projects-agent-context-disclosure__trigger[data-state="open"] svg
  ) {
    transform: rotate(90deg);
  }
  p {
    margin: 0;
    padding: 0.75rem;
    font-size: var(--community-text-secondary);
    color: var(--muted-foreground);
  }
</style>

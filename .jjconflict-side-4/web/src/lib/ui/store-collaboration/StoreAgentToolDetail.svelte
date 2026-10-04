<script lang="ts">
  import * as Chat from "@lapismd/design-core/ai/chat";
  import { FileDiff } from "@lapismd/design-core/diff";
  import type { Model } from "../../api/client.js";
  import CommunityProjectsRevisionGraphIds from "../community-projects/CommunityProjectsRevisionGraphIds.svelte";

  let {
    tool,
    patches = {},
  }: {
    tool: Model<"AgentToolActivity">;
    patches?: Readonly<Record<string, string>>;
  } = $props();

  const input = $derived(parseRecord(tool.input?.content));
  const paths = $derived(
    unique([
      ...tool.code.flatMap((association) => association.paths),
      ...tool.observed_paths,
      string(input?.path),
      string(input?.file),
      string(input?.filePath),
    ]),
  );
  const oldText = $derived(
    string(input?.old_string) ?? string(input?.oldString) ?? null,
  );
  const newText = $derived(
    string(input?.new_string) ??
      string(input?.newString) ??
      string(input?.text) ??
      null,
  );

  function parseRecord(value?: string): Record<string, unknown> | undefined {
    if (!value) return;
    try {
      const parsed: unknown = JSON.parse(value);
      if (parsed && typeof parsed === "object" && !Array.isArray(parsed))
        return parsed as Record<string, unknown>;
    } catch {
      return;
    }
  }
  function string(value: unknown): string | undefined {
    return typeof value === "string" && value ? value : undefined;
  }
  function unique(values: (string | undefined)[]): string[] {
    return [
      ...new Set(values.filter((value): value is string => Boolean(value))),
    ];
  }
  function patchFor(path: string): string | undefined {
    return (
      patches[path] ??
      Object.entries(patches).find(([candidate]) =>
        path.endsWith(`/${candidate}`),
      )?.[1]
    );
  }
</script>

<div class="store-agent-tool-detail">
  <div class="store-agent-tool-detail__identities">
    {#each tool.code as association (association.association_id)}<CommunityProjectsRevisionGraphIds
        commitId={association.after_commits[0] ??
          association.before_commits[0] ??
          ""}
        changeId={association.changes[0]}
        placement="inline"
      />{/each}
  </div>
  {#if paths.length > 0 && (tool.kind === "edit" || tool.kind === "delete" || tool.kind === "move")}
    {#each paths as path (path)}
      <section
        class="store-agent-tool-detail__file"
        aria-label={`Changed file ${path}`}
      >
        <header>{path}</header>
        {#if patchFor(path)}
          <FileDiff {path} patch={patchFor(path)!} viewMode="unified" wrap />
        {:else if newText !== null || oldText !== null}
          <FileDiff
            {path}
            {oldText}
            newText={newText ?? ""}
            viewMode="unified"
            wrap
          />
        {:else}
          <p>Changed file recorded by the agent.</p>
        {/if}
      </section>
    {/each}
  {/if}
  <Chat.ToolCallDetail
    input={tool.input?.content}
    output={tool.state === "failed" ? undefined : tool.output?.content}
    error={tool.state === "failed"
      ? (tool.output?.content ?? "Tool failed")
      : undefined}
    toolName={tool.native_name}
  />
</div>

<style>
  .store-agent-tool-detail {
    display: grid;
    min-width: 0;
    gap: 0.75rem;
  }

  .store-agent-tool-detail__identities {
    display: flex;
    min-width: 0;
    flex-wrap: wrap;
    gap: 0.35rem;
  }

  .store-agent-tool-detail__file {
    min-width: 0;
    overflow: hidden;
    border: 1px solid var(--border);
    border-radius: var(--radius);
  }

  .store-agent-tool-detail__file > header {
    padding: 0.4rem 0.6rem;
    border-bottom: 1px solid var(--border);
    background: var(--jj-store-row-background);
    font-family: var(--font-mono, var(--community-font-mono));
    font-size: var(--community-text-secondary);
  }

  .store-agent-tool-detail__file > p {
    margin: 0;
    padding: 0.65rem;
    color: var(--muted-foreground);
    font-size: var(--community-text-secondary);
  }
</style>

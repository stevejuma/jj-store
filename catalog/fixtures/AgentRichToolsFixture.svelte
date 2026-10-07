<script lang="ts">
  import CommunityAgentTranscriptTools from "../../web/src/lib/ui/community-agent-activity/CommunityAgentTranscriptTools.svelte";
  import StoreAgentToolDetail from "../../web/src/lib/ui/store-collaboration/StoreAgentToolDetail.svelte";
  import type { ActivityRow } from "../../web/src/lib/ui/store-agent-execution/presentation.js";
  import { groupedAgentActivity } from "./agent-tool-groups.js";
  let { recorded = false }: { recorded?: boolean } = $props();
  const tool = groupedAgentActivity.find((a) => a.type === "tool")!;
  const rows: ActivityRow[] = [
    {
      title: "Read example.ts",
      kind: "read",
      locations: [{ path: "src/example.ts" }],
      rawOutput: {
        formatted_output:
          'export const answer = 42;\nexport const html = "\x3cscript>not executed\x3c/script>";\n',
      },
    },
    {
      title: "Edit example.ts",
      kind: "edit",
      content: [
        {
          type: "diff",
          path: "src/example.ts",
          oldText: "export const answer = 41;\n",
          newText: "export const answer = 42;\n",
        },
      ],
    },
    {
      title: "Command",
      kind: "execute",
      rawInput: { command: "pnpm test" },
      rawOutput: { formatted_output: "7 tests passed", stderr: "" },
    },
    {
      title: "Search",
      kind: "search",
      rawInput: { command: 'rg "answer" src' },
      rawOutput: {
        formatted_output: "src/example.ts:1:export const answer = 42;",
      },
    },
    {
      title: "Unsupported",
      kind: "other",
      rawInput: { custom: true },
      content: [
        { type: "image", url: "https://invalid.invalid/untrusted.png" },
      ],
    },
    {
      title: "Failed read",
      kind: "read",
      status: "failed",
      locations: [{ path: "private.ts" }],
      rawOutput: { formatted_output: "Permission denied" },
    },
    {
      title: "Partial edit",
      kind: "edit",
      truncated: true,
      content: [{ path: "partial.rs", newText: "fn captured() {}\n" }],
    },
  ].map((value, index) => ({
    id: `turn/tool/${index}`,
    kind: "tool",
    text: value.title,
    value: { status: "completed", ...value },
    request: "turn",
    sequence: index,
  }));
</script>

<section aria-label="Rich agent tools">
  {#if recorded && tool.type === "tool"}
    {#each rows.slice(0, 2) as row (row.id)}
      <section aria-label={row.text}>
        <StoreAgentToolDetail
          tool={{
            ...tool.tool,
            native_name: row.text,
            kind: row.value.kind === "read" ? "read" : "edit",
            state: "succeeded",
            code: [],
            observed_paths: row.value.kind === "read" ? ["src/example.ts"] : [],
            input: null,
            output: {
              media_type: "text",
              content:
                row.value.kind === "read"
                  ? "export const answer = 42;\n"
                  : JSON.stringify(row.value.content),
            },
          }}
        />
      </section>
    {/each}
  {:else}
    {#each rows as row (row.id)}<section aria-label={row.text}>
        <CommunityAgentTranscriptTools item={row} />
      </section>{/each}
  {/if}
</section>

<style>
  section[aria-label="Rich agent tools"] {
    display: grid;
    gap: 1rem;
    padding: 1rem;
    max-width: 50rem;
  }
</style>

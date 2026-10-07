<script lang="ts">
  import { Button } from "@lapismd/design-core/shadcn/button";
  import type { CommunityMemberModel } from "../community-workspace/model.js";
  import type { createComposerAgentAudience } from "./composer-agent-audience.svelte.js";
  let {
    audience,
    members,
    disabled = false,
  }: {
    audience: ReturnType<typeof createComposerAgentAudience>;
    members: readonly CommunityMemberModel[];
    disabled?: boolean;
  } = $props();
</script>

<div class="community-composer-audience">
  <label
    ><input
      type="checkbox"
      {disabled}
      checked={audience.enabled}
      onchange={(e) => audience.toggle(e.currentTarget.checked)}
    /> Automatically mention agents</label
  >
  {#each audience.pinned as id (id)}<Button
      type="button"
      variant="ghost"
      size="sm"
      {disabled}
      aria-label={`Stop always mentioning ${members.find((m) => m.acpAgentId === id)?.name ?? id}`}
      onclick={() => audience.unpin(id)}
      >@{members.find((m) => m.acpAgentId === id)?.name ?? id} ×</Button
    >{/each}
</div>

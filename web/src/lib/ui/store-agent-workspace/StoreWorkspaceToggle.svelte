<script lang="ts">
  import { Button } from "@lapismd/design-core/shadcn/button";
  import { useConversationInspection } from "../../conversations/inspection.svelte.js";
  import CommunitySidebarToggleIcon from "../community-app-shell/CommunitySidebarToggleIcon.svelte";
  const inspection = useConversationInspection();
  const expanded = $derived(
    !!inspection?.workspace && !inspection.workspaceRail.closed,
  );
</script>

<span class="community-app-shell__sidebar-action">
  <Button
    variant="default"
    size="icon-sm"
    aria-label="Toggle workspace files"
    title={inspection?.inboxWorkspace
      ? "Workspace files"
      : "No workspace available for this session"}
    disabled={!inspection?.inboxWorkspace}
    aria-expanded={expanded}
    onclick={() => inspection?.toggleWorkspace()}
    ><CommunitySidebarToggleIcon side="right" {expanded} /></Button
  >
</span>

<script lang="ts">
  import Folder from "@lucide/svelte/icons/folder";
  import CommunityChip from "../community-chip/CommunityChip.svelte";
  import { ownNotifications } from "../../notifications/notification-context.svelte.js";
  let { path }: { path: string } = $props();
  const { context } = ownNotifications();
  async function copy() {
    try {
      await navigator.clipboard.writeText(path);
      context.service.resolve(context.scope, "stack-folder-copy");
      context.service.publish({
        scope: context.scope,
        key: "stack-folder-copy",
        title: "Copied",
        message: "Folder path copied",
        severity: "info",
        lifetime: "timed",
      });
    } catch {
      context.service.publish({
        scope: context.scope,
        key: "stack-folder-copy",
        title: "Copy failed",
        message: "Could not copy the folder path. Try again.",
        severity: "error",
        lifetime: "dismiss",
      });
    }
  }
</script>

<CommunityChip
  variant="muted"
  shape="square"
  class="stack-workspace__folder"
  ariaLabel={`Copy folder path: ${path}`}
  title="Copy folder path"
  onclick={() => void copy()}
>
  {#snippet leading()}<Folder size={14} aria-hidden="true" />{/snippet}
  {path}
</CommunityChip>

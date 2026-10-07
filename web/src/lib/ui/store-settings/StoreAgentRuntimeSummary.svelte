<script lang="ts">
  import * as Field from "@lapismd/design-core/shadcn/field";
  import CheckIcon from "@lucide/svelte/icons/check";
  import CommunityChip from "../community-chip/CommunityChip.svelte";
  import CommunityMemberAvatar from "../community-member-avatar/CommunityMemberAvatar.svelte";
  import { agentAvatarUrl } from "../../collaboration/agent-avatar.js";
  import type { Runtime } from "../../controller/agent-runtimes.svelte.js";
  let { runtime, status }: { runtime: Runtime; status: Runtime["state"] } =
    $props();
  const labels: Record<string, string> = {
    ready: "Ready",
    cli_missing: "CLI needed",
    adapter_needed: "Adapter needed",
    update_available: "Update available",
    node_needed: "Node.js needed",
    installing: "Installing…",
    unknown: "Unknown",
  };
</script>

<div class="store-runtime__summary">
  <CommunityMemberAvatar
    member={{
      id: runtime.id,
      name: runtime.label,
      handle: runtime.id,
      initials: runtime.label.slice(0, 2),
      kind: "agent",
      avatarUrl: agentAvatarUrl(runtime.id === "cursor" ? "cursor" : "codex"),
    }}
    size="small"
    decorative
  />
  <div class="store-runtime__copy">
    <div class="store-runtime__identity">
      <Field.Label>{runtime.label}</Field.Label>
      <CommunityChip
        variant={status === "ready" ? "subtle" : "muted"}
        tone={status === "ready" ? "success" : "neutral"}
        size="compact"
      >
        {#snippet leading()}{#if status === "ready"}<CheckIcon
              aria-hidden="true"
            />{/if}{/snippet}
        {labels[status] ?? "Unknown"}
      </CommunityChip>
    </div>
    <Field.Description class="store-runtime__description">
      {runtime.detail}
    </Field.Description>
    {#if runtime.adapter_version || runtime.cli_version}
      <Field.Description class="store-runtime__description">
        {runtime.adapter_version ? `ACP ${runtime.adapter_version} · ` : ""}
        {runtime.cli_version
          ? `CLI ${runtime.cli_version} · `
          : ""}{runtime.managed ? "Managed by jj-store" : "Device installation"}
      </Field.Description>
    {/if}
  </div>
</div>

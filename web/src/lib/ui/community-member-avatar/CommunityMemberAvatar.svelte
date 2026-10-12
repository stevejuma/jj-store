<script lang="ts">
  import { useProfileMembers } from "../../profiles/context.svelte.js";
  import { useAgentExecution } from "../store-agent-execution/context.js";
  import {
    agentLiveState,
    agentLiveLabels,
  } from "../../controller/agent-live-status.js";
  import { communityMemberAvatarContent } from "../../community/avatar-content.js";
  import type { CommunityMemberModel } from "../community-workspace/model.js";
  import {
    communityMemberAvatarFallbackTone,
    communityMemberAvatarShape,
  } from "./model.js";
  import "./community-member-avatar.css";

  let {
    member: suppliedMember,
    size = "medium",
    showPresence = false,
    decorative = false,
    contrast = "default",
    presence: presenceOverride,
    shape,
  }: {
    member: CommunityMemberModel;
    size?: "inline" | "xsmall" | "small" | "medium" | "large" | "xlarge";
    shape?: "circle" | "rounded";
    showPresence?: boolean;
    decorative?: boolean;
    contrast?: "default" | "strong";
    presence?: CommunityMemberModel["presence"] | "unknown";
  } = $props();
  const profiles = useProfileMembers(() => [suppliedMember]);
  const member = $derived(profiles.values[0]!);

  const execution = useAgentExecution();
  const liveState = $derived(
    member.acpAgentId
      ? agentLiveState(
          execution?.value ?? null,
          member.acpAgentId,
          !!execution?.connectionError,
        )
      : undefined,
  );
  const indicatorVisible = $derived(showPresence || !!member.acpAgentId);
  const hidden = $derived(decorative && !member.acpAgentId);
  const resolvedShape = $derived(shape ?? communityMemberAvatarShape(member));
  const presence = $derived(presenceOverride ?? member.presence ?? "offline");
  let failed = $state("");
  const content = $derived(
    communityMemberAvatarContent(
      member.avatarUrl === failed
        ? { ...member, avatarUrl: undefined }
        : member,
    ),
  );
  const fallbackTone = $derived(communityMemberAvatarFallbackTone(member.name));
  const presenceLabel = $derived(
    liveState
      ? agentLiveLabels[liveState]
      : presence === "online"
        ? "Online"
        : presence === "away"
          ? "Away"
          : presence === "unknown"
            ? "Presence unknown"
            : "Offline",
  );
  const accessibleLabel = $derived(
    hidden
      ? undefined
      : indicatorVisible
        ? `${member.name}, ${presenceLabel}`
        : `${member.name} avatar`,
  );
</script>

<span
  class="community-member-avatar"
  data-ui-component="community-member-avatar"
  data-size={size}
  data-contrast={contrast === "strong" ? "strong" : undefined}
  data-shape={resolvedShape}
  title={liveState ? `${member.name}, ${presenceLabel}` : undefined}
  aria-hidden={hidden ? "true" : undefined}
  aria-label={accessibleLabel}
  role={hidden ? undefined : "img"}
>
  <span
    class="community-member-avatar__image"
    data-ui-part="image"
    data-content={content.kind}
    data-fallback-tone={content.kind === "initials" ? fallbackTone : undefined}
  >
    {#if content.kind === "image"}
      <img
        src={content.src}
        alt=""
        onerror={() => {
          if (content.kind === "image") failed = content.src;
        }}
      />
    {:else if content.kind === "emoji"}
      <span data-ui-part="emoji" aria-hidden="true">{content.emoji}</span>
    {:else}
      <span aria-hidden="true">{content.initials}</span>
    {/if}
  </span>
  {#if indicatorVisible}
    <span
      class="community-member-avatar__presence"
      data-ui-part="presence"
      data-presence={presence}
      data-runtime={liveState}
      aria-hidden="true"
    ></span>
  {/if}
</span>

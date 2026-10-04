<script lang="ts">
  import type { ComposerStatus } from "@lapismd/design-core/ai/chat";

  import type {
    CommunityCustomEmoji,
    CommunityTopicDraft,
  } from "../../community/contracts.js";
  import CommunityChatComposer from "../community-chat-composer/CommunityChatComposer.svelte";
  import type {
    CommunityMemberModel,
    CommunityRoomModel,
  } from "../community-workspace/model.js";

  let {
    open = $bindable(false),
    content = $bindable(""),
    forumName,
    members = [],
    rooms = [],
    communityEmojis = [],
    disabled = false,
    disabledReason = "Posting is unavailable",
    composerStatus,
    onRetryComposerStatus,
    onDismissComposerStatus,
    pending = false,
    composer = $bindable(null),
    onPublish,
  }: {
    open?: boolean;
    content?: string;
    forumName: string;
    members?: readonly CommunityMemberModel[];
    rooms?: readonly CommunityRoomModel[];
    communityEmojis?: readonly CommunityCustomEmoji[];
    disabled?: boolean;
    disabledReason?: string;
    composerStatus?: ComposerStatus;
    onRetryComposerStatus?: () => void | Promise<void>;
    onDismissComposerStatus?: () => void;
    pending?: boolean;
    composer?: HTMLDivElement | null;
    onPublish: (draft: CommunityTopicDraft) => Promise<void>;
  } = $props();

  function cancel(): void {
    open = false;
    content = "";
  }

  async function publish(value: string): Promise<void> {
    const normalized = value.trim();
    if (!normalized || pending) return;
    const firstLine = normalized.split(/\r?\n/u)[0] ?? normalized;
    await onPublish({
      title:
        firstLine.length > 120
          ? `${firstLine.slice(0, 119).trimEnd()}…`
          : firstLine,
      content: normalized,
      tags: [],
    });
    open = false;
  }
</script>

<CommunityChatComposer
  bind:ref={composer}
  bind:value={content}
  {members}
  {rooms}
  {communityEmojis}
  {disabled}
  {disabledReason}
  busy={pending}
  ariaLabel={`New post in ${forumName}`}
  submitLabel="Publish post"
  placeholder="Write your post…"
  {...composerStatus === undefined ? {} : { composerStatus }}
  {...onRetryComposerStatus === undefined ? {} : { onRetryComposerStatus }}
  {...onDismissComposerStatus === undefined ? {} : { onDismissComposerStatus }}
  onCancel={cancel}
  onSubmit={(submission) => publish(submission.content)}
/>

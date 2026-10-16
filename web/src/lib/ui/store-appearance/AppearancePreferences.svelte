<script lang="ts">
  import StoreSettingsCard from "../store-settings/StoreSettingsCard.svelte";
  import SectionIcon from "@lucide/svelte/icons/sliders-horizontal";
  import LinkIcon from "@lucide/svelte/icons/link";
  import MessageCircleIcon from "@lucide/svelte/icons/message-circle";
  import Eye from "@lucide/svelte/icons/eye";
  import { useAppearance } from "../../appearance/store.svelte.js";
  import type {
    FontSize,
    ConversationDensity,
    LinkStyle,
    ThreadLayout,
    ConversationView,
  } from "../../appearance/preferences.js";
  import CommunityMarkdownContent from "../community-markdown-content/CommunityMarkdownContent.svelte";
  import AppearanceChoice from "./AppearanceChoice.svelte";
  import AppearanceRow from "./AppearanceRow.svelte";
  const store = useAppearance()!;
  let threads = $state<ThreadLayout | null>(null);
</script>

<StoreSettingsCard
  id="appearance-preferences"
  title="Preferences"
  Icon={SectionIcon}
>
  <div class="appearance-card appearance-card--settings">
    <AppearanceRow
      title="Conversation view"
      description="Document or Classic for agent conversations, threads and replies"
    >
      <AppearanceChoice
        label="Conversation view"
        value={store.saved.conversationView}
        options={[
          { value: "document", label: "Document" },
          { value: "classic", label: "Classic" },
        ]}
        onChange={(v) =>
          store.update({ conversationView: v as ConversationView })}
      />
    </AppearanceRow>
    <AppearanceRow
      title="Font size"
      description="Applies across conversations and interface text"
    >
      <AppearanceChoice
        label="Font size"
        value={store.saved.fontSize}
        options={[
          { value: "smaller", label: "Smaller" },
          { value: "default", label: "Default" },
          { value: "larger", label: "Larger" },
        ]}
        onChange={(v) => store.update({ fontSize: v as FontSize })}
        onPreview={(v) =>
          v ? store.preview({ fontSize: v as FontSize }) : store.resetPreview()}
      />
    </AppearanceRow>
    <AppearanceRow
      title="Conversation density"
      description="Spacing in conversations and Markdown content across jj-store"
    >
      <AppearanceChoice
        label="Conversation density"
        value={store.saved.density}
        options={[
          { value: "compact", label: "Compact" },
          { value: "comfortable", label: "Comfy" },
          { value: "spacious", label: "Spacious" },
        ]}
        onChange={(v) => store.update({ density: v as ConversationDensity })}
        onPreview={(v) =>
          v
            ? store.preview({ density: v as ConversationDensity })
            : store.resetPreview()}
      />
    </AppearanceRow>
    <div class="appearance-preview" aria-hidden="true">
      <div class="appearance-preview__surface">
        <span class="appearance-preview__label"><Eye />Preview</span>
        <div class="appearance-conversation-preview">
          {#each [{ initial: "M", author: "Maya", time: "9:41", content: "The revised conversation layout is ready to review." }, { initial: "T", author: "Theo", time: "9:43", content: "I added a longer message so you can compare line height and text spacing.\n\nThe same rhythm carries through threads, reviews, and task comments." }] as message}
            <div class="appearance-sample-message">
              <span class="appearance-sample-avatar">{message.initial}</span>
              <div>
                <p>
                  <strong>{message.author}</strong><small>{message.time}</small>
                </p>
                <CommunityMarkdownContent content={message.content} />
              </div>
            </div>
          {/each}
        </div>
      </div>
    </div>
  </div>
</StoreSettingsCard>
<StoreSettingsCard id="appearance-links" title="Link previews" Icon={LinkIcon}>
  <div class="appearance-card appearance-card--settings">
    <AppearanceRow
      title="Display style"
      description={store.live.links === "inline"
        ? "Small site icons beside links; no preview cards"
        : `${store.live.links === "rich" ? "Larger" : "Compact"} cards for recognised standalone links; prose links stay inline`}
    >
      <AppearanceChoice
        label="Link previews"
        value={store.saved.links}
        options={[
          { value: "inline", label: "Inline" },
          { value: "compact", label: "Compact" },
          { value: "rich", label: "Rich" },
        ]}
        onChange={(v) => store.update({ links: v as LinkStyle })}
        onPreview={(v) =>
          v ? store.preview({ links: v as LinkStyle }) : store.resetPreview()}
      />
    </AppearanceRow>
    <div
      class="appearance-preview"
      role="group"
      aria-label="Link preview examples"
    >
      <div class="appearance-preview__surface">
        <span class="appearance-preview__label" aria-hidden="true"
          ><Eye />Preview</span
        >
        <div class="appearance-link-examples">
          <CommunityMarkdownContent
            content={"Review [#42](https://github.com/team/repo/pull/42), [GitLab](https://gitlab.com/team/repo) and [Forgejo](https://code.forgejo.org/team/repo) before merging.\n\nhttps://github.com/team/repo/pull/42"}
          />
        </div>
      </div>
    </div>
  </div>
</StoreSettingsCard>
<StoreSettingsCard
  id="appearance-threads"
  title="Thread layout"
  Icon={MessageCircleIcon}
>
  <div class="appearance-card appearance-card--settings">
    <AppearanceRow
      title="Discussion placement"
      description={(threads ?? store.saved.threads) === "focus"
        ? "Discussions open over the conversation"
        : "Discussions open in a side panel next to the conversation"}
    >
      <AppearanceChoice
        label="Thread layout"
        value={store.saved.threads}
        options={[
          { value: "focus", label: "Focus" },
          { value: "split", label: "Split" },
        ]}
        onChange={(v) => store.update({ threads: v as ThreadLayout })}
        onPreview={(v) => (threads = v as ThreadLayout | null)}
      />
    </AppearanceRow>
    <div class="appearance-preview" aria-hidden="true">
      <div class="appearance-preview__surface">
        <span class="appearance-preview__label"><Eye />Preview</span>
        <div
          class="appearance-thread-diagram"
          data-layout={threads ?? store.saved.threads}
        >
          <div><i></i><i></i></div>
          <div><i></i><i></i></div>
        </div>
      </div>
    </div>
  </div>
</StoreSettingsCard>

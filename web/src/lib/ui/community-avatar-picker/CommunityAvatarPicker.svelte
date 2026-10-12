<script lang="ts">
  import "./avatar-picker.css";
  import * as Popover from "@lapismd/design-core/shadcn/popover";
  import * as Tabs from "@lapismd/design-core/shadcn/tabs";
  import { Input } from "@lapismd/design-core/shadcn/input";
  import { onDestroy } from "svelte";
  import PlusIcon from "@lucide/svelte/icons/plus";
  import UploadCloudIcon from "@lucide/svelte/icons/upload-cloud";
  import LinkIcon from "@lucide/svelte/icons/link-2";
  import CommunityEmojiPicker from "../community-emoji-picker/CommunityEmojiPicker.svelte";
  import { communityMemberAvatarFallbackTone } from "../community-member-avatar/model.js";
  import {
    COMMUNITY_AVATAR_IMAGE_ACCEPT,
    resolveCommunityAvatarImage,
  } from "./image-files.js";
  import type { CommunityCustomEmoji } from "../../community/contracts.js";

  let {
    value = $bindable(""),
    open = $bindable(false),
    label = "Choose avatar",
    disabled = false,
    communityEmojis = [],
    shape = "rounded",
    size = "large",
    fallbackName = "",
    onUploadImage,
  }: {
    value?: string;
    open?: boolean;
    label?: string;
    disabled?: boolean;
    communityEmojis?: readonly CommunityCustomEmoji[];
    shape?: "circle" | "rounded";
    size?: "medium" | "large";
    fallbackName?: string;
    onUploadImage?: ((file: File) => Promise<string>) | undefined;
  } = $props();
  const emojis = [
    "😀",
    "😎",
    "🤓",
    "🥳",
    "🤖",
    "👻",
    "🐝",
    "🦊",
    "🐙",
    "🌈",
    "🔥",
    "✨",
    "🚀",
    "💎",
    "🎨",
    "💡",
  ];
  const colors = [
    "#f5d90a",
    "#ff8a4c",
    "#e5484d",
    "#ab4aba",
    "#6e56cf",
    "#0091ff",
    "#30a46c",
    "#687076",
  ];
  let mode = $state("image");
  let color = $state(colors[4]!);
  let selectedEmoji = $state("");
  let filename = $state("");
  let error = $state("");
  let uploading = $state(false);
  let reader: FileReader | undefined;
  const fallbackInitials = $derived(initials(fallbackName));
  onDestroy(() => reader?.abort());

  function initials(value: string): string {
    const parts = value
      .trim()
      .split(/[\s_-]+/u)
      .filter(Boolean);
    return (
      parts.length > 1
        ? `${parts[0]?.[0] ?? ""}${parts.at(-1)?.[0] ?? ""}`
        : (parts[0]?.slice(0, 2) ?? "")
    ).toUpperCase();
  }

  function emojiAvatar(emoji: string) {
    selectedEmoji = emoji;
    const safe = emoji
      .replaceAll("&", "&amp;")
      .replaceAll("<", "&lt;")
      .replaceAll(">", "&gt;");
    const radius = shape === "circle" ? 128 : 64;
    value = `data:image/svg+xml,${encodeURIComponent(`<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 256 256"><rect width="256" height="256" rx="${radius}" fill="${color}"/><text x="128" y="166" font-size="128" text-anchor="middle">${safe}</text></svg>`)}`;
    error = "";
  }
  async function readImage(file: File | undefined) {
    if (!file || uploading) return;
    error = "";
    uploading = true;
    reader?.abort();
    try {
      value = await resolveCommunityAvatarImage(file, {
        upload: onUploadImage,
        setReader: (next) => {
          reader = next;
        },
      });
      filename = file.name;
      selectedEmoji = "";
    } catch (cause) {
      error =
        cause instanceof Error
          ? cause.message
          : "The image could not be read. Try again.";
    } finally {
      uploading = false;
    }
  }
</script>

<Popover.Root bind:open>
  <Popover.Trigger>
    {#snippet child({ props })}
      <button
        {...props}
        type="button"
        class="community-avatar-picker__trigger"
        data-size={size}
        data-shape={shape}
        aria-label={label}
        disabled={disabled || uploading}
      >
        {#if value}<img
            src={value}
            alt="Selected avatar"
            width="256"
            height="256"
          />{:else if fallbackInitials}<span
            class="community-avatar-picker__fallback"
            data-fallback-tone={communityMemberAvatarFallbackTone(fallbackName)}
            aria-hidden="true">{fallbackInitials}</span
          >{:else}<PlusIcon aria-hidden="true" />{/if}
      </button>
    {/snippet}
  </Popover.Trigger>
  <Popover.Content
    side="bottom"
    align="start"
    sideOffset={12}
    class="community-avatar-picker__content"
    aria-label="Avatar image picker"
  >
    <Tabs.Root bind:value={mode}>
      <Tabs.List aria-label="Avatar type"
        ><Tabs.Trigger value="image">Image</Tabs.Trigger><Tabs.Trigger
          value="emoji">Emoji</Tabs.Trigger
        ></Tabs.List
      >
      <Tabs.Content value="image">
        <label
          class="community-avatar-picker__drop"
          ondragover={(event) => event.preventDefault()}
          ondrop={(event) => {
            event.preventDefault();
            void readImage(event.dataTransfer?.files[0]);
          }}
        >
          <UploadCloudIcon aria-hidden="true" /><span
            >{uploading ? "Uploading…" : filename || "Drop or browse"}</span
          >
          <input
            type="file"
            accept={COMMUNITY_AVATAR_IMAGE_ACCEPT}
            aria-label="Upload avatar image"
            disabled={uploading}
            onchange={(event) => void readImage(event.currentTarget.files?.[0])}
          />
        </label>
        <div class="community-avatar-picker__url">
          <LinkIcon aria-hidden="true" /><Input
            type="url"
            bind:value
            aria-label="Avatar image URL"
            placeholder="Paste a URL"
          />
        </div>
      </Tabs.Content>
      <Tabs.Content value="emoji">
        <div
          class="community-avatar-picker__colors"
          role="group"
          aria-label="Avatar background"
        >
          {#each colors as item}<button
              type="button"
              data-avatar-color={item}
              aria-label={`Use ${item} background`}
              aria-pressed={color === item}
              onclick={() => {
                color = item;
                if (selectedEmoji) emojiAvatar(selectedEmoji);
              }}
            ></button>{/each}
        </div>
        <div
          class="community-avatar-picker__emojis"
          role="group"
          aria-label="Emoji presets"
        >
          {#each emojis as emoji}<button
              type="button"
              aria-label={`Use ${emoji} as avatar`}
              onclick={() => emojiAvatar(emoji)}>{emoji}</button
            >{/each}
        </div>
        <div class="community-avatar-picker__browse">
          <CommunityEmojiPicker
            label="Browse all emoji"
            {communityEmojis}
            onSelect={(emoji, custom) => {
              if (custom) {
                value = custom.url;
                selectedEmoji = "";
              } else emojiAvatar(emoji);
            }}
          /><span>Browse all emoji</span>
        </div>
      </Tabs.Content>
    </Tabs.Root>
    {#if error}<p role="alert">{error}</p>{/if}
  </Popover.Content>
</Popover.Root>

<script lang="ts">
  import { Button } from "@lapismd/design-core/shadcn/button";
  import { Input } from "@lapismd/design-core/shadcn/input";
  import * as Field from "@lapismd/design-core/shadcn/field";
  import TrashIcon from "@lucide/svelte/icons/trash-2";
  import type { DirectoryManagement } from "../../controller/directory.svelte.js";
  import type { Model } from "../../api/client.js";
  import StoreNotice from "../store-notifications/StoreNotice.svelte";
  import "./profile-settings.css";
  import "./emoji-settings.css";
  let { management }: { management: DirectoryManagement | undefined } =
    $props();
  let uploading = $state(false),
    error = $state("");
  const model = $derived(management?.state);
  const draft = $derived(model?.emojiDraft);
  const owned = $derived(
    model?.emojis.filter(
      (e) => e.owner === model.profile?.principal && !e.retired,
    ) ?? [],
  );
  async function upload(file: File | undefined) {
    if (!file || !management || uploading) return;
    uploading = true;
    error = "";
    try {
      if (!draft) return;
      draft.filename = file.name;
      draft.shortcode ||= file.name
        .replace(/\.[^.]+$/u, "")
        .toLowerCase()
        .replace(/[^a-z0-9_-]+/gu, "-");
      await management.upload(file, "emoji");
    } catch (cause) {
      error = cause instanceof Error ? cause.message : "Image upload failed.";
    } finally {
      uploading = false;
    }
  }
  async function save() {
    if (!management) return;
    error = "";
    try {
      if (draft)
        await management.saveEmoji(
          draft.shortcode,
          draft.asset,
          draft.selected,
        );
    } catch {
      error =
        model?.validation ||
        model?.error ||
        "Emoji save failed. Your draft is retained.";
    }
  }
  async function remove(emoji: Model<"LibraryEmoji">) {
    try {
      await management?.retire(emoji);
    } catch {
      error = "Emoji could not be removed. Refresh and retry.";
    }
  }
</script>

<section class="community-settings" aria-labelledby="emoji-settings-title">
  <header class="community-settings__heading">
    <h1 id="emoji-settings-title">Custom emoji</h1>
    <p>Add your own custom emoji for everyone on this server to use.</p>
  </header>
  {#if !management?.available()}<p>
      Custom emoji storage requires an object store and an admitted user
      identity.
    </p>
  {:else if draft}
    <form
      class="community-settings__custom-emoji"
      onsubmit={(event) => {
        event.preventDefault();
        void save();
      }}
    >
      <section class="community-settings__option-section">
        <header><h2>{draft.selected ? "Edit emoji" : "Add emoji"}</h2></header>
        <div class="community-settings__emoji-builder">
          <div class="community-settings__emoji-row">
            <div>
              <strong>Image</strong>
              <p>PNG, JPEG, GIF or WebP; up to 2 MB and 512 × 512 pixels.</p>
            </div>
            <Input
              type="file"
              accept="image/png,image/jpeg,image/gif,image/webp"
              aria-label="Upload emoji image"
              disabled={uploading || model?.busy || !!model?.pending}
              onchange={(event) => void upload(event.currentTarget.files?.[0])}
            />
          </div>
          <div class="community-settings__emoji-row">
            <Field.Label for="emoji-shortcode">Shortcode</Field.Label><Input
              id="emoji-shortcode"
              bind:value={draft.shortcode}
              placeholder="party"
              maxlength={64}
              disabled={model?.busy || !!model?.pending}
            />
          </div>
          <div class="community-settings__emoji-actions">
            {#if draft.asset}<img
                src={model?.urls[draft.asset.replace(/^blob:/, "")]}
                alt={`Preview of :${draft.shortcode}:`}
                width="40"
                height="40"
              />{/if}<span>{draft.filename}</span><Button
              type="submit"
              disabled={!draft.asset ||
                !draft.shortcode ||
                uploading ||
                model?.busy ||
                !!model?.pending}>Save emoji</Button
            >{#if draft.selected}<Button
                type="button"
                variant="outline"
                onclick={() => {
                  draft.selected = undefined;
                  draft.shortcode = draft.asset = draft.filename = "";
                }}>Cancel edit</Button
              >{/if}
          </div>
        </div>
      </section>
    </form>
    <section
      class="community-settings__option-section community-settings__emoji-list"
      aria-label="Your custom emoji"
    >
      <header>
        <h2>My emoji</h2>
        <p>{owned.length} saved</p>
      </header>
      {#if !owned.length}<p>You haven't added any emoji yet.</p>{:else}<ul>
          {#each owned as emoji (emoji.id)}<li>
              <img src={model?.urls[emoji.preview]} alt="" /><span
                ><strong>:{emoji.shortcode}:</strong></span
              ><Button
                type="button"
                variant="ghost"
                disabled={model?.busy || !!model?.pending}
                onclick={() => {
                  draft.selected = emoji;
                  draft.shortcode = emoji.shortcode;
                  draft.asset = `blob:${emoji.asset}`;
                  void management.image(draft.asset);
                }}>Edit</Button
              ><Button
                type="button"
                variant="ghost"
                size="icon-sm"
                aria-label={`Remove :${emoji.shortcode}:`}
                disabled={model?.busy || !!model?.pending}
                onclick={() => void remove(emoji)}
                ><TrashIcon aria-hidden="true" /></Button
              >
            </li>{/each}
        </ul>{/if}
    </section>
    {#if model?.pending}<p>
        Outcome unknown. Check the original request before saving another emoji.
      </p>
      <Button type="button" onclick={() => void management.status()}
        >Check save status</Button
      ><Button
        type="button"
        variant="outline"
        onclick={() => void management.retry()}>Retry original save</Button
      >{/if}
  {/if}
  <StoreNotice
    noticeKey="custom-emoji-outcome"
    title="Custom emoji could not be saved"
    message={model?.pending ? "" : error || model?.error}
  />
  <StoreNotice
    noticeKey="custom-emoji-saved"
    title="Custom emoji saved"
    message={model?.outcome === "emoji" ? model.message : ""}
    severity="info"
    lifetime="timed"
  />
</section>

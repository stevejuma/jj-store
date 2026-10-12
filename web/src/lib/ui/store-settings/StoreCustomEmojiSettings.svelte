<script lang="ts">
  import { Button } from "@lapismd/design-core/shadcn/button";
  import * as InputGroup from "@lapismd/design-core/shadcn/input-group";
  import * as Field from "@lapismd/design-core/shadcn/field";
  import TrashIcon from "@lucide/svelte/icons/trash-2";
  import SmileIcon from "@lucide/svelte/icons/smile";
  import ImageIcon from "@lucide/svelte/icons/image";
  import HashIcon from "@lucide/svelte/icons/hash";
  import type { DirectoryManagement } from "../../controller/directory.svelte.js";
  import type { Model } from "../../api/client.js";
  import StoreNotice from "../store-notifications/StoreNotice.svelte";
  import StoreSettingsCard from "./StoreSettingsCard.svelte";
  import StoreSettingsFooter from "./StoreSettingsFooter.svelte";
  import StoreDirectoryField from "./StoreDirectoryField.svelte";
  import "./profile-settings.css";
  import "./emoji-settings.css";
  let { management }: { management: DirectoryManagement | undefined } =
    $props();
  let uploading = $state(false),
    error = $state(""),
    imageError = $state("");
  let imageInput = $state<HTMLInputElement>();
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
    imageError = "";
    try {
      if (!draft) return;
      draft.filename = file.name;
      draft.shortcode ||= file.name
        .replace(/\.[^.]+$/u, "")
        .toLowerCase()
        .replace(/[^a-z0-9_-]+/gu, "-");
      management.selectEmoji(file);
    } catch (cause) {
      imageError =
        cause instanceof Error ? cause.message : "Image upload failed.";
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
    } catch (cause) {
      error =
        model?.validation ||
        model?.error ||
        (cause instanceof Error ? cause.message : "") ||
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

<section class="store-directory-settings" aria-label="Custom emoji library">
  {#if !management?.available()}<p>
      Custom emoji storage requires an object store and an admitted user
      identity.
    </p>
  {:else if draft}
    <form
      onsubmit={(event) => {
        event.preventDefault();
        void save();
      }}
    >
      <StoreSettingsCard
        id="emoji-builder"
        title={draft.selected ? "Edit emoji" : "Add emoji"}
        description="Choose an image and a unique shortcode."
        Icon={SmileIcon}
      >
        <Field.Group class="store-settings__editable-rows">
          <StoreDirectoryField
            id="emoji-image"
            label="Image"
            Icon={ImageIcon}
            description="PNG, JPEG, GIF or WebP, up to 20 MB. Uploaded when you save; large images are resized and resized animations use their first frame."
          >
            <div class="store-directory-settings__emoji-upload">
              <span class="store-directory-settings__emoji-image">
                {#if draft.asset || draft.source}<img
                    src={draft.preview ||
                      model?.urls[draft.asset.replace(/^blob:/, "")]}
                    alt={`Preview of :${draft.shortcode}:`}
                    width="48"
                    height="48"
                  />{:else}<ImageIcon aria-hidden="true" />{/if}
              </span>
              <input
                bind:this={imageInput}
                id="emoji-image"
                name="emoji-image"
                type="file"
                hidden
                accept="image/png,image/jpeg,image/gif,image/webp"
                aria-label="Upload emoji image file"
                disabled={uploading || model?.busy || !!model?.pending}
                onchange={(event) => {
                  void upload(event.currentTarget.files?.[0]);
                  event.currentTarget.value = "";
                }}
              />
              <Button
                type="button"
                variant="outline"
                aria-label="Choose emoji image"
                disabled={uploading || model?.busy || !!model?.pending}
                onclick={() => imageInput?.click()}>Choose image</Button
              >
            </div>
          </StoreDirectoryField>
          <StoreDirectoryField
            id="emoji-shortcode"
            label="Shortcode"
            Icon={HashIcon}
            description="Use lowercase letters, numbers, underscores or hyphens."
          >
            <InputGroup.Root>
              <InputGroup.Addon align="inline-start" aria-hidden="true"
                ><InputGroup.Text>:</InputGroup.Text></InputGroup.Addon
              >
              <InputGroup.Input
                id="emoji-shortcode"
                name="shortcode"
                autocomplete="off"
                spellcheck={false}
                bind:value={draft.shortcode}
                placeholder="party"
                maxlength={64}
                disabled={model?.busy || !!model?.pending}
              />
              <InputGroup.Addon align="inline-end" aria-hidden="true"
                ><InputGroup.Text>:</InputGroup.Text></InputGroup.Addon
              >
            </InputGroup.Root>
          </StoreDirectoryField>
        </Field.Group>
        {#if imageError}<p
            role="alert"
            class="store-directory-settings__validation"
          >
            {imageError}
          </p>{/if}
        <StoreSettingsFooter>
          {#snippet description()}<div
              class="store-directory-settings__emoji-preview"
            >
              <span>{draft.filename}</span>
            </div>{/snippet}
          {#if draft.selected || draft.source || draft.asset}<Button
              type="button"
              variant="outline"
              disabled={model?.busy || !!model?.pending}
              onclick={() => {
                management.cancelEmoji();
                imageError = "";
              }}>{draft.selected ? "Cancel edit" : "Cancel"}</Button
            >{/if}
          <Button
            type="submit"
            disabled={(!draft.asset && !draft.source) ||
              !draft.shortcode ||
              uploading ||
              model?.busy ||
              !!model?.pending}>Save emoji</Button
          >
        </StoreSettingsFooter>
        {#if model?.pending}<StoreSettingsFooter>
            {#snippet description()}<p>
                Outcome unknown. Check the original request before saving
                another emoji.
              </p>{/snippet}
            <Button
              type="button"
              variant="outline"
              disabled={model.busy}
              onclick={() => void management.status()}>Check save status</Button
            >
            <Button
              type="button"
              disabled={model.busy}
              onclick={() => void management.retry()}
              >Retry original save</Button
            >
          </StoreSettingsFooter>{/if}
      </StoreSettingsCard>
    </form>
    <div class="community-settings__emoji-list">
      <StoreSettingsCard
        id="my-emoji"
        title="My emoji"
        description={`${owned.length} saved. Edit or remove the emoji you own.`}
        Icon={SmileIcon}
      >
        {#if !owned.length}<p class="store-directory-settings__empty">
            You haven't added any emoji yet.
          </p>
        {:else}{#each owned as emoji (emoji.id)}
            <StoreSettingsFooter>
              {#snippet description()}<div
                  class="store-directory-settings__emoji-preview"
                >
                  <img
                    src={model?.urls[emoji.preview]}
                    alt=""
                    width="40"
                    height="40"
                  /><strong>:{emoji.shortcode}:</strong>
                </div>{/snippet}
              <Button
                type="button"
                variant="outline"
                disabled={model?.busy || !!model?.pending}
                onclick={() => {
                  management.cancelEmoji();
                  draft.selected = emoji;
                  draft.shortcode = emoji.shortcode;
                  draft.asset = `blob:${emoji.asset}`;
                  void management.image(draft.asset);
                }}>Edit</Button
              >
              <Button
                type="button"
                variant="ghost"
                size="icon-sm"
                aria-label={`Remove :${emoji.shortcode}:`}
                disabled={model?.busy || !!model?.pending}
                onclick={() => void remove(emoji)}
                ><TrashIcon aria-hidden="true" /></Button
              >
            </StoreSettingsFooter>
          {/each}{/if}
      </StoreSettingsCard>
    </div>
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

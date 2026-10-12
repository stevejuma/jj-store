<script lang="ts">
  import { Button } from "@lapismd/design-core/shadcn/button";
  import * as Field from "@lapismd/design-core/shadcn/field";
  import { Input } from "@lapismd/design-core/shadcn/input";
  import PencilIcon from "@lucide/svelte/icons/pencil";
  import CheckIcon from "@lucide/svelte/icons/check";
  import type { DirectoryManagement } from "../../controller/directory.svelte.js";
  import CommunityAvatarPicker from "../community-avatar-picker/CommunityAvatarPicker.svelte";
  import CommunityChatComposer from "../community-chat-composer/CommunityChatComposer.svelte";
  import StoreNotice from "../store-notifications/StoreNotice.svelte";
  import StoreSettingsSkeleton from "./StoreSettingsSkeleton.svelte";
  import "./profile-settings.css";
  let { management }: { management: DirectoryManagement | undefined } =
    $props();
  let editing = $state(false),
    avatarOpen = $state(false),
    error = $state("");
  const model = $derived(management?.state);
  const avatar = $derived(
    model?.draft.avatar?.startsWith("blob:")
      ? (model.urls[model.draft.avatar.slice(5)] ?? "")
      : (model?.draft.avatar ?? ""),
  );
  function setAvatar(value: string) {
    if (!model) return;
    const digest = Object.keys(model.urls).find(
      (key) => model.urls[key] === value,
    );
    model.draft.avatar = digest ? `blob:${digest}` : value || null;
  }
  async function save() {
    error = "";
    try {
      await management?.save();
      if (!model?.pending && !model?.validation) {
        editing = false;
        avatarOpen = false;
      }
    } catch {
      error = model?.error || "Could not save profile. Your draft is retained.";
    }
  }
</script>

<section class="community-settings" aria-labelledby="profile-settings-title">
  <header class="community-settings__heading">
    <h1 id="profile-settings-title">Profile</h1>
    <p>Update how your name, avatar, and description appear across jj-store.</p>
  </header>
  {#if !management?.available()}<p>
      Profile storage is unavailable on this installation. An object store and
      an admitted user identity are required.
    </p>
  {:else if !model?.profile}<StoreSettingsSkeleton
      cards={[{ rows: ["preview", "input", "textarea", "input"], actions: 2 }]}
      label="Loading profile"
    />
  {:else}
    <form
      class="community-settings__profile"
      onsubmit={(event) => {
        event.preventDefault();
        void save();
      }}
    >
      <div class="community-settings__avatar-editor">
        <CommunityAvatarPicker
          bind:value={() => avatar, setAvatar}
          bind:open={avatarOpen}
          label="Edit profile photo"
          shape="circle"
          size="medium"
          fallbackName={model.draft.display_name}
          disabled={model.busy || !!model.pending}
          communityEmojis={management.emojis}
          onUploadImage={(file) => management.upload(file)}
        />
        <Button
          variant="ghost"
          type="button"
          disabled={model.busy || !!model.pending}
          onclick={() => {
            model.draft.avatar = null;
          }}>Clear avatar</Button
        >
      </div>
      {#if !avatarOpen}
        <section class="community-settings__option-section">
          <header>
            <h2>Profile info</h2>
            <Button
              type="button"
              size="sm"
              variant={editing ? "default" : "secondary"}
              disabled={model.busy || !!model.pending || model.conflict}
              onclick={() => {
                if (editing) void save();
                else editing = true;
              }}
              >{#if editing}<CheckIcon
                  aria-hidden="true"
                />Done{:else}<PencilIcon aria-hidden="true" />Edit{/if}</Button
            >
          </header>
          <Field.Group class="community-settings__option-group">
            <Field.Field class="community-settings__option-row"
              ><Field.Label for="profile-display-name">Display name</Field.Label
              >{#if editing}<Input
                  id="profile-display-name"
                  bind:value={model.draft.display_name}
                  disabled={model.busy || !!model.pending}
                  maxlength={256}
                />{:else}<p>
                  {model.draft.display_name || "Not set"}
                </p>{/if}</Field.Field
            >
            <Field.Field class="community-settings__option-row"
              ><Field.Label>Profile description</Field.Label
              >{#if editing}<CommunityChatComposer
                  mode="form"
                  disabled={model.busy || !!model.pending}
                  bind:value={model.draft.description}
                  ariaLabel="Profile description"
                />{:else}<p>
                  {model.draft.description || "Not set"}
                </p>{/if}</Field.Field
            >
            <Field.Field class="community-settings__option-row"
              ><Field.Label for="profile-email">Email</Field.Label
              >{#if editing}<Input
                  id="profile-email"
                  type="email"
                  value={model.draft.email ?? ""}
                  oninput={(event) => {
                    model.draft.email = event.currentTarget.value || null;
                  }}
                  disabled={model.busy || !!model.pending}
                  maxlength={254}
                />{:else}<p>
                  {model.draft.email || "Not set"}
                </p>{/if}<Field.Description
                >Only you can read this field. Used for Gravatar when no avatar
                is selected.</Field.Description
              ></Field.Field
            >
          </Field.Group>
        </section>
        <section class="community-settings__option-section">
          <header><h2>Identity</h2></header>
          <details class="community-settings__identity">
            <summary>Principal ID</summary>
            <p>{model.profile.principal}</p>
          </details>
        </section>
      {/if}
      {#if management.dirty}<div class="community-settings__avatar-actions">
          <Button
            type="button"
            variant="outline"
            disabled={model.busy || !!model.pending}
            onclick={() => {
              management.cancel();
              editing = false;
            }}>Cancel</Button
          ><Button
            type="submit"
            disabled={model.busy || !!model.pending || model.conflict}
            >{model.busy ? "Saving…" : "Save profile"}</Button
          >
        </div>{/if}
      {#if model.validation}<p role="alert">{model.validation}</p>{/if}
      {#if model.conflict}<Button
          type="button"
          variant="outline"
          onclick={() => {
            management.review();
          }}>Use my draft with the current version</Button
        >{/if}
      {#if model.pending}<p>
          Save outcome unknown. Your original request and draft are retained.
        </p>
        <Button type="button" onclick={() => void management.status()}
          >Check save status</Button
        ><Button
          type="button"
          variant="outline"
          onclick={() => void management.retry()}>Retry original save</Button
        >{/if}
    </form>
  {/if}
  <StoreNotice
    noticeKey="profile-settings-load"
    title="Profile unavailable"
    message={model?.pending ? "" : model?.error || error}
  />
  <StoreNotice
    noticeKey="profile-settings-saved"
    title="Profile saved"
    message={model?.outcome === "profile" ? model.message : ""}
    severity="info"
    lifetime="timed"
  />
</section>

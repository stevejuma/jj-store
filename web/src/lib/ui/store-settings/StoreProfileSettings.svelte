<script lang="ts">
  import { Button } from "@lapismd/design-core/shadcn/button";
  import * as Field from "@lapismd/design-core/shadcn/field";
  import { Input } from "@lapismd/design-core/shadcn/input";
  import UserIcon from "@lucide/svelte/icons/user";
  import ImageIcon from "@lucide/svelte/icons/image";
  import FileTextIcon from "@lucide/svelte/icons/file-text";
  import MailIcon from "@lucide/svelte/icons/mail";
  import FingerprintIcon from "@lucide/svelte/icons/fingerprint";
  import PencilIcon from "@lucide/svelte/icons/pencil";
  import XIcon from "@lucide/svelte/icons/x";
  import type { DirectoryManagement } from "../../controller/directory.svelte.js";
  import CommunityAvatarPicker from "../community-avatar-picker/CommunityAvatarPicker.svelte";
  import CommunityChatComposer from "../community-chat-composer/CommunityChatComposer.svelte";
  import StoreNotice from "../store-notifications/StoreNotice.svelte";
  import StoreSettingsSkeleton from "./StoreSettingsSkeleton.svelte";
  import StoreSettingsCard from "./StoreSettingsCard.svelte";
  import StoreSettingsFooter from "./StoreSettingsFooter.svelte";
  import StoreReadOnlyRow from "./StoreReadOnlyRow.svelte";
  import StoreDirectoryField from "./StoreDirectoryField.svelte";
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
      error = model?.validation
        ? ""
        : model?.error || "Could not save profile. Your draft is retained.";
    }
  }
</script>

<section class="store-directory-settings" aria-label="Profile details">
  {#if !management?.available()}<p>
      Profile storage requires an object store and an admitted user identity.
    </p>
  {:else if !model?.profile}<StoreSettingsSkeleton
      cards={[{ rows: ["preview", "input", "textarea", "input"], actions: 2 }]}
      label="Loading profile"
    />
  {:else}
    <form
      onsubmit={(event) => {
        event.preventDefault();
        void save();
      }}
    >
      <StoreSettingsCard
        id="profile-info"
        title="Profile info"
        description="Your public details and private contact preference."
        Icon={UserIcon}
      >
        <Field.Group class="store-settings__editable-rows">
          <StoreDirectoryField
            label="Profile photo"
            Icon={ImageIcon}
            description="PNG, JPEG, GIF or WebP, up to 20 MB. Large photos are resized; resized animations use their first frame."
          >
            <div class="store-directory-settings__avatar">
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
                class="store-directory-settings__avatar-action"
                size="icon-xs"
                type="button"
                aria-label={model.draft.avatar ? "Clear avatar" : "Edit avatar"}
                title={model.draft.avatar ? "Clear avatar" : "Edit avatar"}
                disabled={model.busy || !!model.pending}
                onclick={() => {
                  if (model.draft.avatar) {
                    model.draft.avatar = null;
                    avatarOpen = false;
                  } else avatarOpen = true;
                }}
              >
                {#if model.draft.avatar}<XIcon
                    aria-hidden="true"
                  />{:else}<PencilIcon aria-hidden="true" />{/if}
              </Button>
            </div>
          </StoreDirectoryField>
          {#if editing}
            <StoreDirectoryField
              id="profile-display-name"
              label="Display name"
              Icon={UserIcon}
            >
              <Input
                id="profile-display-name"
                name="display-name"
                autocomplete="name"
                bind:value={model.draft.display_name}
                disabled={model.busy || !!model.pending}
                maxlength={256}
              />
            </StoreDirectoryField>
            <StoreDirectoryField
              label="Profile description"
              Icon={FileTextIcon}
              description="Tell others about yourself. Markdown is supported."
            >
              <CommunityChatComposer
                mode="form"
                disabled={model.busy || !!model.pending}
                bind:value={model.draft.description}
                ariaLabel="Profile description"
              />
            </StoreDirectoryField>
            <StoreDirectoryField
              id="profile-email"
              label="Email"
              Icon={MailIcon}
              description="Only you can read this field. Used for Gravatar when no avatar is selected."
            >
              <Input
                id="profile-email"
                name="profile-email"
                autocomplete="email"
                spellcheck={false}
                type="email"
                value={model.draft.email ?? ""}
                oninput={(event) => {
                  model.draft.email = event.currentTarget.value || null;
                }}
                disabled={model.busy || !!model.pending}
                maxlength={254}
              />
            </StoreDirectoryField>
          {/if}
        </Field.Group>
        {#if !editing}<dl class="store-settings__rows">
            <StoreReadOnlyRow
              label="Display name"
              Icon={UserIcon}
              value={model.draft.display_name}
              emptyLabel="Not set"
            />
            <StoreReadOnlyRow
              label="Profile description"
              Icon={FileTextIcon}
              value={model.draft.description}
              emptyLabel="Not set"
            />
            <StoreReadOnlyRow
              label="Email"
              Icon={MailIcon}
              value={model.draft.email}
              emptyLabel="Not set"
              description="Private; used for Gravatar when no avatar is selected."
            />
          </dl>{/if}
        <dl class="store-settings__rows">
          <StoreReadOnlyRow
            label="Identity"
            copyLabel="Principal ID"
            Icon={FingerprintIcon}
            value={model.profile.principal}
            copyable
          />
        </dl>
        {#if model.validation}<p
            role="alert"
            class="store-directory-settings__validation"
          >
            {model.validation}
          </p>{/if}
        <StoreSettingsFooter>
          {#if model.conflict}<Button
              type="button"
              variant="outline"
              disabled={model.busy || !!model.pending}
              onclick={() => management.review()}
              >Use my draft with the current version</Button
            >{/if}
          {#if !editing}<Button
              type="button"
              variant="outline"
              disabled={model.busy || !!model.pending || model.conflict}
              onclick={() => {
                editing = true;
              }}><PencilIcon aria-hidden="true" />Edit</Button
            >{/if}
          {#if editing || management.dirty}<Button
              type="button"
              variant="outline"
              disabled={model.busy || !!model.pending}
              onclick={() => {
                management.cancel();
                editing = false;
              }}>Cancel</Button
            >
            <Button
              type="submit"
              disabled={!management.dirty ||
                model.busy ||
                !!model.pending ||
                model.conflict}
              >{model.busy ? "Saving…" : "Save profile"}</Button
            >
          {/if}
        </StoreSettingsFooter>
        {#if model.pending}<StoreSettingsFooter>
            {#snippet description()}<p>
                Save outcome unknown. Your original request and draft are
                retained.
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

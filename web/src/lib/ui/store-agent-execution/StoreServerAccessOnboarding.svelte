<script lang="ts">
  import * as Field from "@lapismd/design-core/shadcn/field";
  import * as Select from "@lapismd/design-core/shadcn/select";
  import { Input } from "@lapismd/design-core/shadcn/input";
  import { Button } from "@lapismd/design-core/shadcn/button";
  import { ConfirmDialog } from "@lapismd/design-core/forms/dialog";
  import KeyIcon from "@lucide/svelte/icons/key-round";
  import FolderIcon from "@lucide/svelte/icons/folder-git-2";
  import UsersIcon from "@lucide/svelte/icons/users";
  import StoreSettingsCard from "../store-settings/StoreSettingsCard.svelte";
  import StoreSettingsFooter from "../store-settings/StoreSettingsFooter.svelte";
  import StoreReadOnlyRow from "../store-settings/StoreReadOnlyRow.svelte";
  import StoreWorkField from "../store-work/StoreWorkField.svelte";
  import type { ServerAgentAdministration } from "../../controller/server-agent-administration.svelte.js";
  let {
    manager,
    repositories,
  }: {
    manager: ServerAgentAdministration;
    repositories: readonly {
      id: string;
      name: string;
      permission: string | null;
      profile?: string;
    }[];
  } = $props();
  const field = $props.id();
  const draft = $derived(manager.onboarding);
  let revoking = $state<string | null>(null);
  const disabled = $derived(manager.busy || !!manager.pending);
  const valid = $derived(
    /^[0-9a-f]{32}$/.test(draft.principal) && !!draft.credential.trim(),
  );
  const eligible = $derived(
    repositories.filter(
      (r) => r.permission === "admin" && (!r.profile || r.profile === "native"),
    ),
  );
  async function enroll(event: SubmitEvent) {
    event.preventDefault();
    await manager.enroll(draft.principal, draft.credential);
    if (!manager.pending && !manager.error) draft.credential = "";
  }
  async function grant(event: SubmitEvent) {
    event.preventDefault();
    await manager.grant(draft.repository, draft.principal, draft.credential);
    if (!manager.pending && !manager.error) draft.credential = "";
  }
</script>

<StoreSettingsCard
  id={`${field}-requirements`}
  title="Access requirements"
  Icon={KeyIcon}
  description="Each permission is independent. The same principal must own the installation token, agent allowlist entry and repository grants."
>
  <dl>
    <StoreReadOnlyRow
      label="Installation identity"
      value="Enroll an existing token with a manage-identities operator credential."
    />
    <StoreReadOnlyRow
      label="Agent permission"
      value="Select the principal in the agent's sharing configuration, then reload saved settings."
    />
    <StoreReadOnlyRow
      label="Repository access"
      value="Grant Write on the existing native WorkStore conversation home and every attached code repository."
    />
    <StoreReadOnlyRow
      label="Working files"
      value="The repositories must already exist on the server. Local dirty files are not uploaded by this dialog."
    />
  </dl>
</StoreSettingsCard>
{#if manager.access?.identities}
  <form onsubmit={enroll}>
    <StoreSettingsCard
      id={`${field}-enrollment`}
      title="Enroll installation identity"
      Icon={UsersIcon}
      description="Enroll an existing user's token. Credentials stay in this application session."
    >
      <Field.Group class="store-settings__editable-rows">
        <StoreWorkField
          label="Principal ID"
          id={`${field}-enroll-principal`}
          {disabled}
          class="store-settings__option-row"
          ><Input
            id={`${field}-enroll-principal`}
            bind:value={draft.principal}
            pattern={"[0-9a-f]{32}"}
            required
            {disabled}
          /></StoreWorkField
        >
        <StoreWorkField
          label="User access token"
          id={`${field}-enroll-token`}
          {disabled}
          class="store-settings__option-row"
          ><Input
            id={`${field}-enroll-token`}
            type="password"
            bind:value={draft.credential}
            autocomplete="off"
            required
            {disabled}
          /></StoreWorkField
        >
      </Field.Group>
      <StoreSettingsFooter
        ><Button type="submit" disabled={disabled || !valid}
          >Enroll token</Button
        ></StoreSettingsFooter
      >
    </StoreSettingsCard>
  </form>
  {#each manager.identities as binding (binding.token)}
    <StoreSettingsCard
      id={`${field}-${binding.token}`}
      title={manager.profiles[binding.principal] || "Installation identity"}
      label={`Installation identity token ${binding.token}`}
      Icon={UsersIcon}
    >
      <dl>
        <StoreReadOnlyRow
          label="Principal"
          value={binding.principal}
          copyable
        /><StoreReadOnlyRow
          label="Token ID"
          value={binding.token}
          copyable
        /><StoreReadOnlyRow
          label="State"
          value={binding.revoked ? "Revoked" : "Enrolled"}
        />
      </dl>
      <StoreSettingsFooter
        ><Button
          type="button"
          variant="destructive"
          disabled={disabled || binding.revoked}
          onclick={() => (revoking = binding.token)}
          >Revoke identity token</Button
        ></StoreSettingsFooter
      >
    </StoreSettingsCard>
  {/each}
{/if}
<form onsubmit={grant}>
  <StoreSettingsCard
    id={`${field}-repository`}
    title="Grant repository Write access"
    Icon={FolderIcon}
    description="Only native repositories where your current token has Admin access are eligible. Enroll the user's token first; grants do not enable agent sharing."
  >
    <Field.Group class="store-settings__editable-rows">
      <StoreWorkField
        label="Repository"
        id={`${field}-repository-picker`}
        {disabled}
        class="store-settings__option-row"
      >
        <Select.Root
          type="single"
          bind:value={draft.repository}
          disabled={disabled || !eligible.length}
          ><Select.Trigger id={`${field}-repository-picker`}
            >{eligible.find((r) => r.id === draft.repository)?.name ||
              "Select repository"}</Select.Trigger
          ><Select.Content preventScroll={false}
            >{#each eligible as r}<Select.Item value={r.id} label={r.name}
                >{r.name}</Select.Item
              >{/each}</Select.Content
          ></Select.Root
        >
      </StoreWorkField>
      <StoreWorkField
        label="Principal ID"
        id={`${field}-grant-principal`}
        {disabled}
        class="store-settings__option-row"
        ><Input
          id={`${field}-grant-principal`}
          bind:value={draft.principal}
          pattern={"[0-9a-f]{32}"}
          required
          {disabled}
        /></StoreWorkField
      >
      <StoreWorkField
        label="User access token"
        id={`${field}-grant-token`}
        {disabled}
        class="store-settings__option-row"
        ><Input
          id={`${field}-grant-token`}
          type="password"
          bind:value={draft.credential}
          autocomplete="off"
          required
          {disabled}
        /></StoreWorkField
      >
    </Field.Group>
    <StoreSettingsFooter
      ><Button
        type="submit"
        disabled={disabled ||
          !valid ||
          !eligible.some((r) => r.id === draft.repository)}
        >Grant Write access</Button
      ></StoreSettingsFooter
    >
  </StoreSettingsCard>
</form>
<ConfirmDialog
  open={!!revoking}
  onOpenChange={(open) => {
    if (!open && !disabled) revoking = null;
  }}
  title="Revoke identity token?"
  description="This token will lose installation identity access and its remote agent sessions will no longer be authorized. Repository grants remain separate."
  confirmLabel="Revoke token"
  onConfirm={async () => {
    if (!revoking) return false;
    await manager.revoke(revoking);
    if (manager.pending || manager.error) return false;
    revoking = null;
  }}
/>

<style>
  dl {
    margin: 0;
  }
</style>

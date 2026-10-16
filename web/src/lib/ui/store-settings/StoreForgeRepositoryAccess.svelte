<script lang="ts">
  import { untrack } from "svelte";
  import { Button } from "@lapismd/design-core/shadcn/button";
  import { Input } from "@lapismd/design-core/shadcn/input";
  import * as Field from "@lapismd/design-core/shadcn/field";
  import StoreSettingsCard from "./StoreSettingsCard.svelte";
  import StoreSettingsFooter from "./StoreSettingsFooter.svelte";
  import StoreSettingsSelect from "./StoreSettingsSelect.svelte";
  import StoreWorkField from "../store-work/StoreWorkField.svelte";
  import type { ForgeController } from "../../controller/forge/controller.svelte.js";
  import type { IntegrationInventory } from "../../controller/integrations.svelte.js";
  let {
    forge,
    accounts,
  }: { forge: ForgeController; accounts: IntegrationInventory["accounts"] } =
    $props();
  const detection = $derived(forge.data?.detection);
  const connection = $derived(forge.data?.connection);
  const draft = $derived(forge.connection.draft);
  const selected = $derived(
    detection?.remotes.find((r) => r.name === draft.remote),
  );
  const id = $props.id();
  $effect(() => {
    detection;
    untrack(() => forge.connection.initialize());
  });
  async function bind() {
    if (!selected || !draft.account) return;
    const success = await forge.run({
      kind: "bind_account",
      account: draft.account,
      connection: {
        provider: "github",
        api_url: selected.api_url,
        source_repository: selected.repository,
        repository: draft.repository || selected.repository,
        remote: selected.name,
        target_remote: draft.targetRemote || selected.name,
        target: draft.target,
        native_stacks: connection?.native_stacks ?? false,
      },
    });
    if (success) await forge.refresh();
  }
</script>

{#if detection?.provider === "github"}
  <StoreSettingsCard
    id={`${id}-title`}
    title="Repository access"
    description={connection?.repository ??
      detection.remotes[0]?.repository ??
      "Choose a GitHub repository connection."}
  >
    <form
      onsubmit={(event) => {
        event.preventDefault();
        void bind();
      }}
    >
      <Field.Group>
        <StoreWorkField
          label="GitHub account"
          id={`${id}-account`}
          description={detection.readiness.replaceAll("_", " ")}
        >
          <StoreSettingsSelect
            id={`${id}-account`}
            bind:value={draft.account}
            options={[
              { value: "", label: "Choose an account" },
              ...accounts
                .filter((a) => a.available)
                .map((a) => ({
                  value: a.id,
                  label: `${a.login} · ${new URL(a.api_url).host} · ${a.method}`,
                })),
            ]}
          />
        </StoreWorkField>
        <StoreWorkField label="Push remote" id={`${id}-remote`}
          ><StoreSettingsSelect
            id={`${id}-remote`}
            bind:value={draft.remote}
            options={[
              { value: "", label: "Choose a remote" },
              ...detection.remotes.map((r) => ({
                value: r.name,
                label: `${r.name} · ${r.repository}`,
              })),
            ]}
            onValueChange={() => {
              if (!connection) draft.repository = selected?.repository ?? "";
            }}
          /></StoreWorkField
        >
        <StoreWorkField
          label="Target repository"
          id={`${id}-repo`}
          description="Choose upstream explicitly for a fork."
          ><Input
            id={`${id}-repo`}
            bind:value={draft.repository}
            placeholder={selected?.repository ?? "owner/repository"}
          /></StoreWorkField
        >
        <StoreWorkField label="Target remote" id={`${id}-target-remote`}
          ><StoreSettingsSelect
            id={`${id}-target-remote`}
            bind:value={draft.targetRemote}
            options={[
              { value: "", label: "Same as push remote" },
              ...detection.remotes.map((r) => ({
                value: r.name,
                label: r.name,
              })),
            ]}
          /></StoreWorkField
        >
        <StoreWorkField
          label="Target branch"
          id={`${id}-target`}
          description="Leave empty to use the repository default branch."
          ><Input
            id={`${id}-target`}
            bind:value={draft.target}
            placeholder="Repository default"
          /></StoreWorkField
        >
      </Field.Group>
      <StoreSettingsFooter
        ><Button
          disabled={forge.busy ||
            Boolean(forge.pending) ||
            !draft.account ||
            !selected}>Connect repository</Button
        ></StoreSettingsFooter
      >
    </form>
  </StoreSettingsCard>
{/if}

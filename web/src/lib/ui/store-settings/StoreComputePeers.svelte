<script lang="ts">
  import * as Field from "@lapismd/design-core/shadcn/field";
  import { Button } from "@lapismd/design-core/shadcn/button";
  import { Input } from "@lapismd/design-core/shadcn/input";
  import { Textarea } from "@lapismd/design-core/shadcn/textarea";
  import { Switch } from "@lapismd/design-core/shadcn/switch";
  import { CodeBlock } from "@lapismd/design-core/shadcn/code-block";
  import StoreSettingsDisclosure from "./StoreSettingsDisclosure.svelte";
  import StoreReadOnlyRow from "./StoreReadOnlyRow.svelte";
  import StoreWorkField from "../store-work/StoreWorkField.svelte";
  import type {
    AgentCompute,
    ComputeValue,
  } from "../../controller/agent-compute.svelte.js";
  import type { Model } from "../../api/client.js";
  let {
    management,
    status,
    blocked,
    draft = $bindable(),
    onConfigure,
  }: {
    management?: AgentCompute;
    status: ComputeValue;
    blocked: boolean;
    draft: Model<"ComputeSettings"> | null;
    onConfigure: () => Promise<boolean>;
  } = $props();
  const id = $props.id();
  let peerName = $state("");
  let invitation = $state("");
  let validation = $state("");
  async function pair() {
    validation = "";
    try {
      const value = JSON.parse(invitation);
      if (!peerName.trim() || typeof value.id !== "string") throw Error();
      await management?.command({
        action: "pair",
        name: peerName,
        invitation: value,
      });
      if (!management?.error) {
        peerName = "";
        invitation = "";
      }
    } catch {
      validation = "Enter a machine name and a valid invitation.";
    }
  }
</script>

{#if draft}
  <section class="store-settings__section" aria-labelledby={`${id}-sharing`}>
    <header class="store-settings__section-heading">
      <h2 id={`${id}-sharing`}>Shared compute</h2>
    </header>
    <div class="store-settings__rows">
      <dl class="store-settings__summary-rows">
        <StoreReadOnlyRow
          label="Machine identity"
          value={status.identity ?? "Available after compute starts"}
          copyable={!!status.identity}
        />
      </dl>
      <Field.Group class="store-settings__editable-rows">
        <StoreWorkField
          class="store-settings__option-row"
          label="Share this machine"
          id={`${id}-share`}
          description="Only explicitly paired machines can use inference. Repository tools stay on the requesting machine."
        >
          <Switch
            id={`${id}-share`}
            checked={draft.sharing}
            disabled={blocked}
            onCheckedChange={(value) => {
              draft!.sharing = value;
              void onConfigure();
            }}
          />
        </StoreWorkField>
        <StoreWorkField
          class="store-settings__option-row"
          label="Inference host"
          id={`${id}-host`}
        >
          <select
            id={`${id}-host`}
            bind:value={draft.target_peer}
            disabled={blocked}
          >
            <option value={null}>This machine</option>
            {#each Object.entries(draft.peers) as [identity, peer] (identity)}
              <option value={identity}>{peer.name}</option>
            {/each}
          </select>
        </StoreWorkField>
        {#if status.invitation}
          <StoreWorkField
            class="store-settings__option-row"
            label="Machine invitation"
            description="On each machine, add the other machine’s invitation and verify its identity."
          >
            <CodeBlock
              aria-label="Machine invitation"
              code={JSON.stringify(status.invitation, null, 2)}
              language="json"
              title="Machine invitation"
              size="sm"
              width="100%"
              isWrapped
              maxHeight="16rem"
            />
          </StoreWorkField>
        {/if}
      </Field.Group>
    </div>
    {#each Object.entries(draft.peers) as [identity, peer] (identity)}
      <div class="store-settings__action-row">
        <div>
          <strong>{peer.name}</strong>
          <p class="store-compute-peer-identity">{identity}</p>
        </div>
        <div class="store-settings__actions">
          <Button
            variant="outline"
            disabled={blocked}
            onclick={() => management?.command({ action: "revoke", identity })}
            >Unpair</Button
          >
        </div>
      </div>
    {/each}
    <div class="store-settings__rows">
      <Field.Group class="store-settings__editable-rows">
        <StoreWorkField
          class="store-settings__option-row"
          label="Machine name"
          id={`${id}-peer-name`}
        >
          <Input
            id={`${id}-peer-name`}
            bind:value={peerName}
            disabled={blocked}
          />
        </StoreWorkField>
        <StoreWorkField
          class="store-settings__option-row"
          label="Machine invitation"
          id={`${id}-invite`}
          description="Paste the invitation from the machine you want to pair."
        >
          <Textarea
            id={`${id}-invite`}
            bind:value={invitation}
            disabled={blocked}
            rows={3}
            aria-invalid={!!validation}
            aria-describedby={validation ? `${id}-validation` : undefined}
          />
        </StoreWorkField>
      </Field.Group>
    </div>
    <div class="store-settings__action-row">
      <div>
        <strong>Pair a trusted machine</strong>
        <p>Verify the machine identity before granting inference access.</p>
        {#if validation}<Field.Error id={`${id}-validation`}
            >{validation}</Field.Error
          >{/if}
      </div>
      <div class="store-settings__actions">
        <Button
          variant="outline"
          disabled={blocked || !peerName || !invitation}
          onclick={pair}>Pair machine</Button
        >
      </div>
    </div>
    <div class="store-settings__rows">
      <Field.Group class="store-settings__editable-rows">
        <StoreSettingsDisclosure>
          <Field.Group class="store-settings__editable-rows">
            <StoreWorkField
              class="store-settings__option-row"
              label="Encrypted relay URLs"
              id={`${id}-relay`}
              description="Optional HTTPS transport relays. Changes apply after Stop compute and the next start. Public discovery stays disabled."
            >
              <Textarea
                id={`${id}-relay`}
                value={draft.relay_urls.join("\n")}
                disabled={blocked}
                rows={3}
                oninput={(event) => {
                  draft!.relay_urls = event.currentTarget.value
                    .split(/\s+/)
                    .filter(Boolean);
                }}
              />
            </StoreWorkField>
          </Field.Group>
        </StoreSettingsDisclosure>
      </Field.Group>
    </div>
  </section>
{/if}

<style>
  .store-compute-peer-identity {
    overflow-wrap: anywhere;
  }
</style>

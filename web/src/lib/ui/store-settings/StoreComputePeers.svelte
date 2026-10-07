<script lang="ts">
  import * as Field from "@lapismd/design-core/shadcn/field";
  import { Button } from "@lapismd/design-core/shadcn/button";
  import { Input } from "@lapismd/design-core/shadcn/input";
  import { Textarea } from "@lapismd/design-core/shadcn/textarea";
  import { Switch } from "@lapismd/design-core/shadcn/switch";
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
  <h2>Shared compute</h2>
  <Field.Group class="community-settings__option-group">
    <StoreWorkField
      label="Share this machine"
      id={`${id}-sharing`}
      description="Only explicitly paired machines can use inference. Repository tools stay on the requesting machine."
    >
      <Switch
        id={`${id}-sharing`}
        checked={draft.sharing}
        disabled={blocked}
        onCheckedChange={(value) => {
          draft!.sharing = value;
          void onConfigure();
        }}
      />
    </StoreWorkField>
    <StoreWorkField label="Inference host" id={`${id}-host`}>
      <select
        id={`${id}-host`}
        bind:value={draft.target_peer}
        disabled={blocked}
        ><option value={null}>This machine</option
        >{#each Object.entries(draft.peers) as [identity, peer] (identity)}<option
            value={identity}>{peer.name}</option
          >{/each}</select
      >
    </StoreWorkField>
    {#if status.invitation}<Field.Field
        ><Field.Label>Machine invitation</Field.Label><Textarea
          readonly
          value={JSON.stringify(status.invitation)}
        /><Field.Description
          >On each machine, add the other machine’s invitation and verify its
          identity.</Field.Description
        ></Field.Field
      >{/if}
    <StoreWorkField label="Machine identity"
      ><span>{status.identity ?? "Available after compute starts"}</span
      ></StoreWorkField
    >
    {#each Object.entries(draft.peers) as [identity, peer] (identity)}<div
        class="store-settings__action-row"
      >
        <div>
          <strong>{peer.name}</strong>
          <p>{identity}</p>
        </div>
        <Button
          variant="outline"
          disabled={blocked}
          onclick={() => management?.command({ action: "revoke", identity })}
          >Unpair</Button
        >
      </div>{/each}
    <Field.Field
      ><Field.Label for={`${id}-peer-name`}>Machine name</Field.Label><Input
        id={`${id}-peer-name`}
        bind:value={peerName}
        disabled={blocked}
      /></Field.Field
    >
    <Field.Field
      ><Field.Label for={`${id}-invite`}>Machine invitation</Field.Label
      ><Textarea
        id={`${id}-invite`}
        bind:value={invitation}
        disabled={blocked}
      /></Field.Field
    >
    {#if validation}<Field.Error>{validation}</Field.Error>{/if}<Button
      variant="outline"
      disabled={blocked || !peerName || !invitation}
      onclick={pair}>Pair machine</Button
    >
    <details>
      <summary>Advanced</summary><Field.Field
        ><Field.Label for={`${id}-relay`}>Encrypted relay URLs</Field.Label
        ><Textarea
          id={`${id}-relay`}
          value={draft.relay_urls.join("\n")}
          disabled={blocked}
          oninput={(event) => {
            draft!.relay_urls = event.currentTarget.value
              .split(/\s+/)
              .filter(Boolean);
          }}
        /><Field.Description
          >Optional HTTPS transport relays. Changes apply after Stop compute and
          the next start. Public discovery stays disabled.</Field.Description
        ></Field.Field
      >
    </details>
    <div class="store-settings__action-row">
      <p>Local models and peer selection never fall back to cloud inference.</p>
      <Button disabled={blocked} onclick={onConfigure}
        >Save compute settings</Button
      >
    </div>
  </Field.Group>
{/if}

<script lang="ts">
  import * as Field from "@lapismd/design-core/shadcn/field";
  import { Input } from "@lapismd/design-core/shadcn/input";
  import { Switch } from "@lapismd/design-core/shadcn/switch";
  import { Button } from "@lapismd/design-core/shadcn/button";
  import {
    copy,
    syncFields,
    type SyncEntry,
  } from "../../controller/server-configuration.js";
  import type { ServerSettingsManagement } from "../../controller/server-settings.svelte.js";
  let {
    management,
    repository,
    repositoryName,
  }: {
    management: ServerSettingsManagement;
    repository: string;
    repositoryName: string;
  } = $props();
  const entries = $derived(management.state.drafts.sync.draft as SyncEntry[]);
  const entry = $derived(entries.find((e) => e.repository === repository));
  const blocked = $derived(
    management.state.saving || Boolean(management.state.drafts.sync.unknown),
  );
  function update(patch: Partial<SyncEntry>) {
    management.configure(
      "sync",
      entries.map((e) =>
        e.repository === repository ? { ...copy(e), ...patch } : copy(e),
      ),
    );
  }
  function add() {
    if (!management.state.value || entry) return;
    management.configure("sync", [
      ...copy(entries),
      {
        repository,
        upstream: "",
        insecure_loopback: false,
        token_environment: "JJ_STORE_REPLICATION_TOKEN_UPSTREAM",
        ca_certificate: null,
        limits: copy(management.state.value.sync_defaults),
      },
    ]);
  }
</script>

<dl class="store-settings__summary-rows">
  <div>
    <dt>Repository</dt>
    <dd>
      {repositoryName || "Repository identity is not loaded yet."}
    </dd>
  </div>
  <div>
    <dt>Credential reference</dt>
    <dd>
      Supply the named environment variable when starting the server. Its secret
      value stays outside Settings. WorkStore sync retains its existing origin
      freeze.
    </dd>
  </div>
</dl>
{#if entry}
  <Field.Group class="store-settings__editable-rows">
    {#each [{ key: "upstream", label: "Upstream origin", help: "HTTPS origin, including a port if needed." }, { key: "token_environment", label: "Token environment variable", help: "A unique JJ_STORE_REPLICATION_TOKEN_ name; never paste a token." }, { key: "ca_certificate", label: "CA certificate path", help: "Optional absolute path to a custom CA certificate." }] as field}
      <Field.Field orientation="responsive" class="store-settings__option-row">
        <Field.Content
          ><Field.Label for={`sync-${field.key}`}>{field.label}</Field.Label
          ><Field.Description>{field.help}</Field.Description></Field.Content
        >
        <Input
          class="store-settings__input"
          id={`sync-${field.key}`}
          disabled={blocked}
          value={entry[
            field.key as "upstream" | "token_environment" | "ca_certificate"
          ] ?? ""}
          oninput={(event) =>
            update({
              [field.key]:
                event.currentTarget.value ||
                (field.key === "ca_certificate" ? null : ""),
            })}
        />
      </Field.Field>
    {/each}
    <Field.Field orientation="horizontal" class="store-settings__option-row">
      <Field.Content
        ><Field.Label for="sync-insecure">Allow loopback HTTP</Field.Label
        ><Field.Description
          >Only literal loopback addresses may use HTTP.</Field.Description
        ></Field.Content
      >
      <Switch
        id="sync-insecure"
        checked={entry.insecure_loopback}
        disabled={blocked}
        onCheckedChange={(insecure_loopback) => update({ insecure_loopback })}
      />
    </Field.Field>
    {#each syncFields as field}
      <Field.Field orientation="responsive" class="store-settings__option-row">
        <Field.Content
          ><Field.Label for={`sync-${field.key}`}>{field.label}</Field.Label>
          <Field.Description
            >Running: {management.state.value?.effective.sync
              .find((e) => e.repository === repository)
              ?.limits[field.key].toLocaleString("en-GB") ??
              "No running upstream"}</Field.Description
          >
        </Field.Content>
        <Input
          class="store-settings__input"
          id={`sync-${field.key}`}
          type="number"
          min={field.min}
          max={field.max}
          step="1"
          required
          disabled={blocked}
          value={String(entry.limits[field.key])}
          oninput={(event) =>
            update({
              limits: {
                ...copy(entry.limits),
                [field.key]: Number(event.currentTarget.value),
              },
            })}
        />
      </Field.Field>
    {/each}
  </Field.Group>
{/if}
<div class="store-settings__action-row store-settings__action-row--joined">
  <div>
    <strong>{entry ? "Remove upstream" : "Configure an upstream"}</strong>
    <p>Saved changes apply at the next startup. Saving contacts no peer.</p>
  </div>
  <div class="store-settings__actions">
    {#if entry}<Button
        type="button"
        variant="outline"
        disabled={blocked}
        onclick={() =>
          management.configure(
            "sync",
            entries.filter((e) => e.repository !== repository).map(copy),
          )}>Remove upstream</Button
      >
    {:else}<Button
        type="button"
        variant="outline"
        disabled={blocked ||
          !/^[0-9a-f]{32}$/.test(repository) ||
          entries.length >= 8}
        onclick={add}>Add upstream</Button
      >{/if}
  </div>
</div>

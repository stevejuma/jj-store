<script lang="ts">
  import StoreNumericSettingNumberRow from "./StoreNumericSettingNumberRow.svelte";
  import { syncNumericFields } from "./sync-numeric-fields.js";
  import StoreSettingsCard from "./StoreSettingsCard.svelte";
  import ActivityIcon from "@lucide/svelte/icons/activity";
  import NetworkIcon from "@lucide/svelte/icons/network";
  import SlidersIcon from "@lucide/svelte/icons/sliders-horizontal";
  import PlusIcon from "@lucide/svelte/icons/plus";
  import UnlinkIcon from "@lucide/svelte/icons/unlink";
  import { settingsPresentation } from "./settings-presentation.js";
  import StoreSettingsFooter from "./StoreSettingsFooter.svelte";
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
    pending = false,
  }: {
    management: ServerSettingsManagement;
    repository: string;
    repositoryName: string;
    pending?: boolean;
  } = $props();
  const presentation = settingsPresentation();
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

{#snippet limits(configured: SyncEntry)}
  {#each syncFields as field}
    {#if presentation.value !== "default"}
      {@const numeric = syncNumericFields[field.key]}
      <StoreNumericSettingNumberRow
        id={`sync-${field.key}`}
        label={numeric.label}
        units={numeric.units}
        description={`Running: ${management.state.value?.effective.sync.find((e) => e.repository === repository)?.limits[field.key].toLocaleString("en-GB") ?? "No running upstream"}`}
        value={configured.limits[field.key]}
        min={field.min}
        max={field.max}
        defaultValue={management.state.value?.sync_defaults[field.key]}
        disabled={blocked}
        onValueChange={(value) =>
          update({
            limits: { ...copy(configured.limits), [field.key]: value },
          })}
      />
    {:else}
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
          value={String(configured.limits[field.key])}
          oninput={(event) =>
            update({
              limits: {
                ...copy(configured.limits),
                [field.key]: Number(event.currentTarget.value),
              },
            })}
        />
      </Field.Field>
    {/if}
  {/each}
{/snippet}

<StoreSettingsCard
  repositoryOnly
  id="sync-current-status"
  title="Current status"
  description="Saved settings and the currently running configuration."
  Icon={ActivityIcon}
>
  <dl class="store-settings__summary-rows">
    {#if presentation.value !== "default"}<div>
        <dt>Saved and running</dt>
        <dd>
          <span role="status"
            >{pending
              ? "Saved changes need a server restart."
              : "Saved settings match the running configuration."}</span
          >
        </dd>
      </div>{/if}
    <div>
      <dt>Repository</dt>
      <dd>
        {repositoryName || "Repository identity is not loaded yet."}
      </dd>
    </div>
    <div>
      <dt>Credential reference</dt>
      <dd>
        Supply the named environment variable when starting the server. Its
        secret value stays outside Settings. WorkStore sync retains its existing
        origin freeze.
      </dd>
    </div>
  </dl>
</StoreSettingsCard>
<StoreSettingsCard
  repositoryOnly
  id="sync-upstream-heading"
  title="Upstream configuration"
  description="Configure the peer and its credential references."
  Icon={NetworkIcon}
>
  {#if entry}
    <Field.Group class="store-settings__editable-rows">
      {#each [{ key: "upstream", label: "Upstream origin", help: "HTTPS origin, including a port if needed." }, { key: "token_environment", label: "Token environment variable", help: "A unique JJ_STORE_REPLICATION_TOKEN_ name; never paste a token." }, { key: "ca_certificate", label: "CA certificate path", help: "Optional absolute path to a custom CA certificate." }] as field}
        <Field.Field
          orientation="responsive"
          class="store-settings__option-row"
        >
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
      {#if presentation.value === "default"}{@render limits(entry)}{/if}
    </Field.Group>
  {/if}
  <StoreSettingsFooter joined>
    {#snippet description()}<div>
        <strong>{entry ? "Remove upstream" : "Configure an upstream"}</strong>
        <p>Saved changes apply at the next startup. Saving contacts no peer.</p>
      </div>{/snippet}

    {#if entry}<Button
        type="button"
        variant="outline"
        disabled={blocked}
        onclick={() =>
          management.configure(
            "sync",
            entries.filter((e) => e.repository !== repository).map(copy),
          )}
        >{#if presentation.value !== "default"}<UnlinkIcon
            data-icon="inline-start"
            aria-hidden="true"
          />{/if}Remove upstream</Button
      >
    {:else}<Button
        type="button"
        variant="outline"
        disabled={blocked ||
          !/^[0-9a-f]{32}$/.test(repository) ||
          entries.length >= 8}
        onclick={add}
        >{#if presentation.value !== "default"}<PlusIcon
            data-icon="inline-start"
            aria-hidden="true"
          />{/if}Add upstream</Button
      >{/if}
  </StoreSettingsFooter>
</StoreSettingsCard>
{#if entry && presentation.value !== "default"}<StoreSettingsCard
    repositoryOnly
    id="sync-limits"
    title="Sync limits"
    description="Bounded transfer and worker limits for this upstream."
    Icon={SlidersIcon}
  >
    <Field.Group class="store-settings__editable-rows">
      {@render limits(entry)}
    </Field.Group>
  </StoreSettingsCard>{/if}

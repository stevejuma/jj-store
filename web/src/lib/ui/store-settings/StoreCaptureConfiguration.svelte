<script lang="ts">
  import * as Field from "@lapismd/design-core/shadcn/field";
  import { Button } from "@lapismd/design-core/shadcn/button";
  import { Switch } from "@lapismd/design-core/shadcn/switch";
  import { Input } from "@lapismd/design-core/shadcn/input";
  import { Textarea } from "@lapismd/design-core/shadcn/textarea";
  import StoreSettingsNotice from "./StoreSettingsNotice.svelte";
  import StoreNotice from "../store-notifications/StoreNotice.svelte";
  import type { CaptureManagement } from "../../controller/capture-types.js";
  import type { CaptureDraft } from "../../controller/capture-configuration.js";
  let { management }: { management: CaptureManagement } = $props();
  const settings = $derived(management.state.settings);
  const busy = $derived(
    management.state.commandBusy ||
      settings.saving ||
      !!settings.unknown ||
      !!management.state.pending,
  );
  const harnesses = [
    {
      name: "Codex CLI",
      profile: "codex-cli-jsonl-v1",
      enabled: "cli",
      roots: "cliRoots",
    },
    {
      name: "Codex Desktop",
      profile: "codex-desktop-jsonl-v1",
      enabled: "desktop",
      roots: "desktopRoots",
    },
  ] as const;
  const limits: {
    key: keyof Pick<CaptureDraft, "readers" | "interval" | "sources" | "bytes">;
    label: string;
    max: number;
    help: string;
  }[] = [
    {
      key: "readers",
      label: "Concurrent readers",
      max: 4,
      help: "Maximum transcript readers active at once (1–4).",
    },
    {
      key: "interval",
      label: "Discovery interval (seconds)",
      max: 30,
      help: "Check watched folders every 1–30 seconds.",
    },
    {
      key: "sources",
      label: "Tracked source limit",
      max: 4096,
      help: "Maximum retained source identities (1–4,096). Lowering this does not remove existing sources.",
    },
    {
      key: "bytes",
      label: "Pending byte budget",
      max: 67108864,
      help: "Durable pending transcript capacity in bytes (up to 64 MiB). Lowering this does not delete queued content.",
    },
  ];
</script>

<section
  class="store-settings__section"
  aria-labelledby="capture-configuration-title"
>
  <header class="store-settings__section-heading">
    <div>
      <h2 id="capture-configuration-title">Capture configuration</h2>
      <p>
        Settings apply to this local installation after the server restarts.
      </p>
    </div>
  </header>
  {#if settings.value && settings.draft}
    <form
      onsubmit={(event) => {
        event.preventDefault();
        void management.saveConfiguration();
      }}
    >
      <Field.Group>
        <div class="store-settings__rows">
          <dl class="store-settings__summary-rows">
            <div>
              <dt>Saved capture</dt>
              <dd>{settings.value.desired.enabled ? "Enabled" : "Disabled"}</dd>
            </div>
            <div>
              <dt>Currently configured</dt>
              <dd>
                {settings.value.effective.enabled
                  ? "Enabled"
                  : "Disabled"}{settings.value.restart_required
                  ? " · Restart required"
                  : " · Matches saved settings"}
              </dd>
            </div>
          </dl>
          <Field.Group class="store-settings__editable-rows">
            <Field.Field
              orientation="horizontal"
              class="store-settings__option-row"
            >
              <Field.Content
                ><Field.Label for="capture-enabled"
                  >Enable automatic capture</Field.Label
                ><Field.Description id="capture-enabled-help"
                  >Capture active sessions only when their repository or
                  workspace is linked to this server.</Field.Description
                ></Field.Content
              >
              <Switch
                id="capture-enabled"
                aria-describedby="capture-enabled-help"
                checked={settings.draft.enabled}
                disabled={busy}
                onCheckedChange={(enabled) => management.configure({ enabled })}
              />
            </Field.Field>
            {#each harnesses as h}
              <Field.Field
                orientation="horizontal"
                class="store-settings__option-row"
              >
                <Field.Content
                  ><Field.Label for={`capture-${h.enabled}`}
                    >{h.name} capture</Field.Label
                  ><Field.Description id={`capture-${h.enabled}-help`}
                    >Qualified version {settings.value.defaults.find(
                      (d) => d.profile === h.profile,
                    )?.product_version}. Inactive history requires explicit
                    import.</Field.Description
                  ></Field.Content
                >
                <Switch
                  id={`capture-${h.enabled}`}
                  aria-describedby={`capture-${h.enabled}-help`}
                  checked={settings.draft[h.enabled]}
                  disabled={busy}
                  onCheckedChange={(enabled) =>
                    management.configure({ [h.enabled]: enabled })}
                />
              </Field.Field>
              <Field.Field
                orientation="responsive"
                class="store-settings__option-row"
              >
                <Field.Content
                  ><Field.Label for={`capture-${h.roots}`}
                    >{h.name} folders</Field.Label
                  ><Field.Description id={`capture-${h.roots}-help`}
                    >One absolute folder per line, up to 16. Leave empty to use
                    these server defaults:{#each settings.value.defaults.find((d) => d.profile === h.profile)?.roots ?? [] as root}<br
                      /><code>{root}</code>{/each}</Field.Description
                  ></Field.Content
                >
                <Textarea
                  class="store-settings__input"
                  id={`capture-${h.roots}`}
                  aria-describedby={`capture-${h.roots}-help`}
                  rows={2}
                  maxlength={65552}
                  spellcheck={false}
                  value={settings.draft[h.roots]}
                  disabled={busy || !settings.draft[h.enabled]}
                  oninput={(event) =>
                    management.configure({
                      [h.roots]: event.currentTarget.value,
                    })}
                />
              </Field.Field>
            {/each}
            {#each limits as f}
              <Field.Field
                orientation="responsive"
                class="store-settings__option-row"
              >
                <Field.Content
                  ><Field.Label for={`capture-${f.key}`}>{f.label}</Field.Label
                  ><Field.Description id={`capture-${f.key}-help`}
                    >{f.help}</Field.Description
                  ></Field.Content
                >
                <Input
                  class="store-settings__input"
                  id={`capture-${f.key}`}
                  aria-describedby={`capture-${f.key}-help`}
                  type="number"
                  min="1"
                  max={f.max}
                  step="1"
                  required
                  value={settings.draft[f.key]}
                  disabled={busy}
                  oninput={(event) =>
                    management.configure({
                      [f.key]: event.currentTarget.value,
                    })}
                />
              </Field.Field>
            {/each}
          </Field.Group>
        </div>
        <div class="store-settings__action-row">
          <div>
            <strong>Save configuration</strong>
            <p>
              Restart the local server to activate saved changes. In the desktop
              app, quit and reopen the app. Existing capture history and pause
              policy are retained.
            </p>
          </div>
          <div class="store-settings__actions">
            <Button
              type="submit"
              disabled={busy || !settings.dirty || settings.conflict}
              >Save configuration</Button
            >
            <Button
              type="button"
              variant="outline"
              disabled={settings.saving ||
                !settings.dirty ||
                (!!settings.unknown && !settings.unknownObserved)}
              onclick={() => management.cancelConfiguration()}
              >{settings.conflict || settings.unknown
                ? "Use saved configuration"
                : "Cancel changes"}</Button
            >
          </div>
        </div>
      </Field.Group>
    </form>
    {#if settings.validation}<p role="alert">{settings.validation}</p>{/if}
    {#if settings.conflict && !settings.unknown}<StoreSettingsNotice
        >Server configuration changed while you were editing. Your draft is
        retained. Refresh to inspect the latest saved configuration, then use
        saved configuration before editing again.</StoreSettingsNotice
      >{/if}
    {#if settings.unknown}<StoreSettingsNotice
        >Save outcome unknown. Refresh to compare the saved configuration with
        your original draft. A different configuration does not prove that the
        original save failed.</StoreSettingsNotice
      >{/if}
  {:else}<StoreSettingsNotice
      >{settings.error
        ? "Capture configuration is unavailable. Refresh settings to retry."
        : "Loading local capture configuration…"}</StoreSettingsNotice
    >{/if}
  <StoreNotice
    scope="installation-capture"
    noticeKey="capture-settings-error"
    title="Agent configuration needs attention"
    message={settings.error}
    pending={settings.saving}
  />
  <StoreNotice
    scope="installation-capture"
    noticeKey="capture-settings-saved"
    title="Agent configuration"
    message={settings.message}
    severity="info"
    lifetime="timed"
  />
</section>

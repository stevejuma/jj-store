<script lang="ts">
  import * as Field from "@lapismd/design-core/shadcn/field";
  import * as Collapsible from "@lapismd/design-core/shadcn/collapsible";
  import { Button } from "@lapismd/design-core/shadcn/button";
  import { Switch } from "@lapismd/design-core/shadcn/switch";
  import { Textarea } from "@lapismd/design-core/shadcn/textarea";
  import ChevronRightIcon from "@lucide/svelte/icons/chevron-right";
  import CommunityMemberAvatar from "../community-member-avatar/CommunityMemberAvatar.svelte";
  import { agentAvatarUrl } from "../../collaboration/agent-avatar.js";
  import type { CaptureManagement } from "../../controller/capture-types.js";
  let { management, busy }: { management: CaptureManagement; busy: boolean } =
    $props();
  const settings = $derived(management.state.settings);
  let open = $state(true);
  const codex = {
    id: "codex",
    name: "Codex",
    handle: "codex",
    initials: "CX",
    kind: "agent" as const,
    avatarUrl: agentAvatarUrl("codex"),
  };
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
</script>

<Collapsible.Root class="store-capture-harness" bind:open>
  <Collapsible.Trigger>
    {#snippet child({ props })}
      <Button
        {...props}
        type="button"
        variant="ghost"
        class="store-capture-harness__trigger"
        aria-label="Codex configuration"
      >
        <CommunityMemberAvatar member={codex} size="small" decorative />
        <span class="store-capture-harness__label"
          >Codex <span class="store-settings__row-description"
            >CLI and Desktop</span
          ></span
        >
        <ChevronRightIcon data-icon="inline-end" aria-hidden="true" />
      </Button>
    {/snippet}
  </Collapsible.Trigger>
  <Collapsible.Content class="store-capture-harness__content">
    {#if settings.value && settings.draft}
      <Field.Group class="store-settings__editable-rows">
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
                )?.product_version}. Inactive history requires explicit import.</Field.Description
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
      </Field.Group>
    {/if}
  </Collapsible.Content>
</Collapsible.Root>

<style>
  :global(.store-capture-harness) {
    min-width: 0;
    border: 1px solid var(--border);
    border-radius: var(--radius);
    overflow: hidden;
    background: var(--card, var(--background));
  }
  :global(.store-capture-harness__trigger) {
    width: 100%;
    height: auto;
    justify-content: flex-start;
    padding: 0.85rem 1rem;
    gap: 0.75rem;
  }
  .store-capture-harness__label {
    display: grid;
    gap: 0.15rem;
    text-align: start;
    flex: 1;
  }
  :global(.store-capture-harness__trigger[aria-expanded="true"] > svg) {
    transform: rotate(90deg);
  }
  :global(.store-capture-harness__content) {
    border-block-start: 1px solid var(--border);
    padding-inline-start: 1rem;
  }
</style>

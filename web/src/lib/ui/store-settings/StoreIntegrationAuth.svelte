<script lang="ts">
  import { Button } from "@lapismd/design-core/shadcn/button";
  import { Input } from "@lapismd/design-core/shadcn/input";
  import * as InputGroup from "@lapismd/design-core/shadcn/input-group";
  import * as Field from "@lapismd/design-core/shadcn/field";
  import { Separator } from "@lapismd/design-core/shadcn/separator";
  import Copy from "@lucide/svelte/icons/copy";
  import ArrowUpRight from "@lucide/svelte/icons/arrow-up-right";
  import X from "@lucide/svelte/icons/x";
  import Eye from "@lucide/svelte/icons/eye";
  import EyeOff from "@lucide/svelte/icons/eye-off";
  import StoreSettingsCard from "./StoreSettingsCard.svelte";
  import StoreSettingsFooter from "./StoreSettingsFooter.svelte";
  import StoreWorkField from "../store-work/StoreWorkField.svelte";
  import type { Integrations } from "../../controller/integrations.svelte.js";
  let { manager }: { manager: Integrations } = $props();
  let visible = $state(false);
  $effect(() => {
    manager.flow;
    visible = false;
  });
  const id = $props.id();
  async function copy() {
    try {
      await navigator.clipboard.writeText(
        manager.data?.authorization?.user_code ?? "",
      );
    } catch {
      manager.error(new Error("Could not copy the verification code."));
    }
  }
</script>

{#if manager.flow}
  <StoreSettingsCard
    id={`${id}-title`}
    title={manager.flow === "oauth"
      ? "Authorize GitHub Account"
      : manager.flow === "enterprise"
        ? "Add GitHub Enterprise Account"
        : "Add Personal Access Token"}
  >
    {#snippet badge()}<Button
        variant="ghost"
        size="icon-sm"
        aria-label="Cancel authorization"
        onclick={() => manager.cancel()}><X aria-hidden="true" /></Button
      >{/snippet}
    {#if manager.flow === "oauth"}
      {@const auth = manager.data?.authorization}
      {#if auth}
        <div class="integration-auth__device">
          <ol class="integration-auth__steps">
            <li>
              <p>Copy the following verification code:</p>
              <div class="integration-auth__code">
                <strong>{auth.user_code}</strong><Button
                  variant="outline"
                  size="sm"
                  onclick={copy}
                  >Copy to Clipboard<Copy data-icon="inline-end" /></Button
                >
              </div>
            </li>
            <li>
              <Separator />
              <p>
                Navigate to the GitHub activation page and paste the code you
                copied.
              </p>
              <Button
                onclick={() =>
                  window.open(
                    auth.verification_uri,
                    "_blank",
                    "noopener,noreferrer",
                  )}
                >Open GitHub activation page<ArrowUpRight
                  data-icon="inline-end"
                /></Button
              >
            </li>
          </ol>
          <p class="integration-auth__waiting" role="status" aria-live="polite">
            {auth.state === "pending"
              ? "Waiting for GitHub authorization…"
              : `Authorization ${auth.state}`}
          </p>
        </div>
      {:else}<p class="integration-auth__device" role="status">
          Requesting verification code…
        </p>{/if}
    {:else}
      <form
        onsubmit={(event) => {
          event.preventDefault();
          void manager.command({
            kind: "add_token",
            api_url:
              manager.flow === "pat" ? "https://api.github.com" : manager.api,
            token: manager.token,
          });
        }}
      >
        <Field.Group>
          {#if manager.flow === "enterprise"}<StoreWorkField
              label="API Base URL"
              id={`${id}-api`}
              description="For example, https://github.example.com/api/v3"
            >
              <Input
                id={`${id}-api`}
                type="url"
                required
                bind:value={manager.api}
                disabled={manager.busy}
                placeholder="https://github.example.com/api/v3"
              />
            </StoreWorkField>{/if}
          <StoreWorkField
            label="Personal Access Token"
            id={`${id}-token`}
            description="Saved in your OS Keychain / Credential Manager."
          >
            <InputGroup.Root>
              <InputGroup.Input
                id={`${id}-token`}
                type={visible ? "text" : "password"}
                autocomplete="off"
                required
                bind:value={manager.token}
                disabled={manager.busy}
              />
              <InputGroup.Addon align="inline-end"
                ><InputGroup.Button
                  aria-label={visible ? "Hide token" : "Show token"}
                  onclick={() => (visible = !visible)}
                  >{#if visible}<EyeOff />{:else}<Eye />{/if}</InputGroup.Button
                ></InputGroup.Addon
              >
            </InputGroup.Root>
          </StoreWorkField>
        </Field.Group>
        <StoreSettingsFooter>
          <Button
            variant="outline"
            type="button"
            disabled={manager.busy}
            onclick={() => manager.cancel()}>Cancel</Button
          >
          <Button
            type="submit"
            disabled={manager.busy ||
              !manager.token.trim() ||
              (manager.flow === "enterprise" && !manager.api)}
          >
            {manager.busy ? "Connecting…" : "Add account"}</Button
          >
        </StoreSettingsFooter>
      </form>
    {/if}
  </StoreSettingsCard>
{/if}

<style>
  .integration-auth__device {
    padding: 1rem;
    display: flex;
    flex-direction: column;
    gap: 0.75rem;
  }
  .integration-auth__device p {
    margin: 0;
  }
  .integration-auth__steps {
    margin: 0;
    padding: 0 0 0 1.75rem;
    list-style: none;
    border-inline-start: 1px dashed var(--border);
  }
  .integration-auth__steps li {
    position: relative;
    display: flex;
    flex-direction: column;
    align-items: flex-start;
    gap: 1rem;
    padding-block: 0.25rem 1rem;
  }
  .integration-auth__steps li::before {
    content: "";
    position: absolute;
    inset-inline-start: -2.05rem;
    top: 0.65rem;
    width: 0.6rem;
    height: 0.6rem;
    border-radius: 50%;
    background: var(--muted-foreground);
    box-shadow: 0 0 0 4px var(--background);
  }
  .integration-auth__steps li :global([data-ui-component="separator"]) {
    align-self: stretch;
    margin-block-end: 0.5rem;
  }
  .integration-auth__steps li:last-child::before {
    top: 2.2rem;
  }
  .integration-auth__code {
    display: flex;
    align-items: center;
    gap: 0.75rem;
    flex-wrap: wrap;
    padding: 0.4rem;
    padding-inline-start: 0.75rem;
    border: 1px solid var(--border);
    border-radius: var(--radius-sm);
  }
  .integration-auth__code strong {
    font-size: 1.35rem;
    letter-spacing: 0.08em;
  }
  .integration-auth__waiting {
    color: var(--muted-foreground);
    font-size: 0.8rem;
    padding-inline-start: 1.75rem;
  }
</style>

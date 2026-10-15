<script lang="ts">
  import { Button } from "@lapismd/design-core/shadcn/button";
  import { Input } from "@lapismd/design-core/shadcn/input";
  import * as InputGroup from "@lapismd/design-core/shadcn/input-group";
  import * as Field from "@lapismd/design-core/shadcn/field";
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
    {#if manager.flow === "oauth"}
      {@const auth = manager.data?.authorization}
      {#if auth}
        <div class="integration-auth__device" role="status" aria-live="polite">
          <p>Copy the verification code, then enter it on GitHub.</p>
          <div>
            <strong>{auth.user_code}</strong><Button
              variant="outline"
              size="sm"
              onclick={copy}>Copy to Clipboard</Button
            >
          </div>
          <a href={auth.verification_uri} target="_blank" rel="noreferrer"
            >Open GitHub activation page ↗</a
          >
          <p>
            {auth.state === "pending"
              ? "Waiting for GitHub authorization…"
              : `Authorization ${auth.state}`}
          </p>
        </div>
      {:else}<p class="integration-auth__device" role="status">
          Requesting verification code…
        </p>{/if}
      <StoreSettingsFooter
        ><Button variant="outline" onclick={() => manager.cancel()}
          >Cancel</Button
        ></StoreSettingsFooter
      >
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
  .integration-auth__device > div {
    display: flex;
    align-items: center;
    gap: 1rem;
    flex-wrap: wrap;
  }
  .integration-auth__device strong {
    font-size: 1.35rem;
    letter-spacing: 0.08em;
  }
  .integration-auth__device a {
    text-decoration: underline;
    text-underline-offset: 0.2em;
  }
</style>

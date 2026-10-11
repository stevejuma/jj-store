<script lang="ts">
  import StoreSettingsFooter from "./StoreSettingsFooter.svelte";
  import StoreSettingsCard from "./StoreSettingsCard.svelte";
  import SectionIcon from "@lucide/svelte/icons/shield-check";
  import { onMount } from "svelte";
  import * as Field from "@lapismd/design-core/shadcn/field";
  import { Button } from "@lapismd/design-core/shadcn/button";
  import StoreWorkField from "../store-work/StoreWorkField.svelte";
  import StoreNotice from "../store-notifications/StoreNotice.svelte";
  import type { AgentChatGpt } from "../../controller/agent-chatgpt.svelte.js";
  let { accounts }: { accounts: AgentChatGpt } = $props();
  const id = $props.id();
  onMount(() => {
    void accounts.refresh();
  });
  const authorizing = $derived(
    accounts.value?.attempts.filter((a) => a.state === "authorizing") ?? [],
  );
</script>

<StoreSettingsCard
  id={`${id}-accounts`}
  title="ChatGPT accounts"
  Icon={SectionIcon}
  description="Connect your ChatGPT plan and choose an account for each JJ Agent."
>
  <Field.Group class="store-settings__editable-rows">
    {#each accounts.value?.accounts ?? [] as account (account.id)}
      <StoreWorkField
        class="store-settings__option-row"
        label={account.email ?? account.label}
        description={account.label}
      >
        <span
          >{account.connected
            ? account.plan_enabled
              ? "Using your ChatGPT plan"
              : "ChatGPT plan permission is required"
            : "Signed out"}</span
        >
      </StoreWorkField>
      <StoreSettingsFooter>
        {#snippet description()}<p>
            Account {account.id.slice(0, 8)}. Shared plan limits apply.
          </p>{/snippet}

        <Button
          variant="outline"
          disabled={accounts.busy ||
            !!accounts.pending ||
            authorizing.some((a) => a.account === account.id)}
          onclick={() => accounts.connect(account.id)}>Reconnect</Button
        >
        {#if account.connected}<Button
            variant="outline"
            disabled={accounts.busy || !!accounts.pending}
            onclick={() => accounts.signOut(account.id)}>Sign out</Button
          >{/if}
      </StoreSettingsFooter>
    {/each}
    {#each authorizing as attempt (attempt.id)}
      <StoreSettingsFooter>
        {#snippet description()}<p>
            Complete sign-in in your browser.
          </p>{/snippet}
        <Button
          variant="outline"
          disabled={accounts.busy || !!accounts.pending}
          onclick={() => accounts.cancel(attempt.id)}>Cancel sign-in</Button
        >
      </StoreSettingsFooter>
    {/each}
    {#each accounts.value?.attempts.filter((a) => a.error) ?? [] as attempt (attempt.id)}
      <StoreWorkField label="Connection status"
        ><span
          >{attempt.error === "remote_revocation_unconfirmed"
            ? "Signed out locally. Remote disconnection was not confirmed; manage access in ChatGPT settings."
            : attempt.error === "authorization_interrupted_reconnect_required"
              ? "Sign-in was interrupted. Reconnect to continue."
              : "Sign-in did not complete. Reconnect to try again."}</span
        ></StoreWorkField
      >
    {/each}
    <StoreSettingsFooter>
      {#snippet description()}<p>
          {accounts.value?.supported === false
            ? "ChatGPT plan connections require local macOS."
            : "Usage counts toward your eligible ChatGPT subscription."}
        </p>{/snippet}

      <Button
        variant="outline"
        disabled={accounts.busy}
        onclick={() => accounts.refresh()}>Check accounts</Button
      >
      <a
        class="chatgpt-usage"
        data-ui-component="button"
        data-slot="button"
        data-variant="outline"
        data-size="default"
        href="https://chatgpt.com/settings/usage"
        target="_blank"
        rel="noopener noreferrer">Manage usage</a
      >
      <Button
        disabled={!accounts.value?.supported ||
          accounts.busy ||
          !!accounts.pending}
        onclick={() => accounts.connect()}>Continue with ChatGPT</Button
      >
    </StoreSettingsFooter>
    {#if accounts.pending}<StoreSettingsFooter>
        {#snippet description()}<p>
            The connection request needs checking.
          </p>{/snippet}
        <Button
          variant="outline"
          disabled={accounts.busy}
          onclick={() => accounts.retry()}>Check connection request</Button
        >
      </StoreSettingsFooter>{/if}
  </Field.Group>
  <StoreNotice
    noticeKey="agent-chatgpt-accounts"
    title="ChatGPT connection needs attention"
    message={accounts.error}
  />
</StoreSettingsCard>

<style>
  .chatgpt-usage {
    flex: none;
    max-width: 100%;
  }
</style>

<script lang="ts">
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

<section class="store-settings__section" aria-labelledby={`${id}-accounts`}>
  <header class="store-settings__section-heading">
    <div>
      <h2 id={`${id}-accounts`}>ChatGPT accounts</h2>
      <p>Connect your ChatGPT plan and choose an account for each JJ Agent.</p>
    </div>
  </header>
  <Field.Group class="community-settings__option-group">
    {#each accounts.value?.accounts ?? [] as account (account.id)}
      <StoreWorkField
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
      <div
        class="store-settings__action-row store-settings__action-row--full-width"
      >
        <p>Account {account.id.slice(0, 8)}. Shared plan limits apply.</p>
        <div class="store-settings__actions store-settings__actions--wrapping">
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
        </div>
      </div>
    {/each}
    {#each authorizing as attempt (attempt.id)}
      <div
        class="store-settings__action-row store-settings__action-row--full-width"
        role="status"
      >
        <p>Complete sign-in in your browser.</p>
        <Button
          variant="outline"
          disabled={accounts.busy || !!accounts.pending}
          onclick={() => accounts.cancel(attempt.id)}>Cancel sign-in</Button
        >
      </div>
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
    <div
      class="store-settings__action-row store-settings__action-row--full-width"
    >
      <p>
        {accounts.value?.supported === false
          ? "ChatGPT plan connections require local macOS."
          : "Usage counts toward your eligible ChatGPT subscription."}
      </p>
      <div class="store-settings__actions store-settings__actions--wrapping">
        <Button
          disabled={!accounts.value?.supported ||
            accounts.busy ||
            !!accounts.pending}
          onclick={() => accounts.connect()}>Continue with ChatGPT</Button
        >
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
      </div>
    </div>
    {#if accounts.pending}<div
        class="store-settings__action-row store-settings__action-row--full-width"
      >
        <p>The connection request needs checking.</p>
        <Button
          variant="outline"
          disabled={accounts.busy}
          onclick={() => accounts.retry()}>Check connection request</Button
        >
      </div>{/if}
  </Field.Group>
  <StoreNotice
    noticeKey="agent-chatgpt-accounts"
    title="ChatGPT connection needs attention"
    message={accounts.error}
  />
</section>

<style>
  .chatgpt-usage {
    flex: 0 0 10rem;
    max-width: 100%;
  }
</style>

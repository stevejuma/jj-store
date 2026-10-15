<script lang="ts">
  import { tick, untrack } from "svelte";
  import { Button } from "@lapismd/design-core/shadcn/button";
  import * as Menu from "@lapismd/design-core/shadcn/dropdown-menu";
  import CommunityMemberAvatar from "../community-member-avatar/CommunityMemberAvatar.svelte";
  import { Skeleton } from "@lapismd/design-core/shadcn/skeleton";
  import Plus from "@lucide/svelte/icons/plus";
  import Trash2 from "@lucide/svelte/icons/trash-2";
  import KeyRound from "@lucide/svelte/icons/key-round";
  import StoreSettingsPage from "./StoreSettingsPage.svelte";
  import StoreSettingsCard from "./StoreSettingsCard.svelte";
  import StoreSettingsFooter from "./StoreSettingsFooter.svelte";
  import StoreSettingsEmpty from "./StoreSettingsEmpty.svelte";
  import StoreIntegrationAuth from "./StoreIntegrationAuth.svelte";
  import StoreForgeRepositoryAccess from "./StoreForgeRepositoryAccess.svelte";
  import StoreForgeIcon from "../store-forge/StoreForgeIcon.svelte";
  import CommunityChip from "../community-chip/CommunityChip.svelte";
  import { ownNotifications } from "../../notifications/notification-context.svelte.js";
  import type { StoreController } from "../../controller/contracts.js";
  let { controller }: { controller: StoreController } = $props();
  const manager = $derived(controller.integrations);
  const notifications = ownNotifications();
  $effect(() => {
    const owner = manager;
    if (!owner) return;
    owner.notifications = notifications.context;
    const app = controller;
    owner.changed = () => {
      void app.stackWorkspace?.forge?.refresh();
    };
    untrack(() => void owner.refresh());
    const timer = setInterval(() => void owner.refresh(), 30000);
    return () => clearInterval(timer);
  });
  const id = $props.id();
  let accountMenuOpen = $state(false);
  async function choose(method: "oauth" | "pat" | "enterprise") {
    accountMenuOpen = false;
    await tick();
    if (!manager) return;
    if (method === "oauth") await manager.begin();
    else manager.flow = method;
  }
</script>

<StoreSettingsPage
  section="integrations"
  onRefresh={async () => {
    await manager?.refresh();
    await controller.stackWorkspace?.forge?.refresh();
  }}
  refreshLabel="Refresh integrations"
  refreshDisabled={manager?.busy}
>
  {#if !manager?.available()}
    <StoreSettingsEmpty
      Icon={KeyRound}
      title="Integrations unavailable"
      description="Manage forge accounts through the local jj-store service."
    />
  {:else}
    <StoreSettingsCard
      id={`${id}-github`}
      title="GitHub"
      description="Create pull requests, inspect checks, and merge your stacks."
    >
      {#if !manager.data}<div
          role="status"
          aria-label="Loading GitHub accounts"
          class="integration-account"
        >
          <Skeleton class="size-10" /><Skeleton class="h-5 w-48" />
        </div>
      {:else}
        {#each manager.data.accounts as account (account.id)}
          <div class="integration-account">
            <CommunityMemberAvatar
              member={{
                id: `forge:${account.id}`,
                name: account.name || account.login,
                handle: account.login,
                initials: account.login.slice(0, 2).toUpperCase(),
                avatarUrl: account.avatar_url ?? undefined,
              }}
              decorative
            />
            <div class="integration-account__identity">
              <div>
                <strong>{account.login}</strong><CommunityChip variant="muted"
                  >{account.method === "oauth"
                    ? "OAuth"
                    : account.method === "enterprise"
                      ? "Enterprise PAT"
                      : "PAT"}</CommunityChip
                >
              </div>
              {#if account.email}<span>{account.email}</span>{/if}<span
                >{new URL(account.api_url).host} · {account.available
                  ? "Connected"
                  : "Reconnect required"}</span
              >
            </div>
            <Button
              variant="outline"
              size="sm"
              disabled={manager.busy}
              aria-label={`Forget ${account.login} ${account.method} account`}
              onclick={() =>
                manager.command({ kind: "forget", account: account.id })}
              >Forget<Trash2 data-icon="inline-end" /></Button
            >
          </div>
        {/each}
      {/if}
      <StoreSettingsFooter>
        {#snippet description()}<span class="integration-caption"
            ><StoreForgeIcon provider="github" /> GitHub account access</span
          >{/snippet}
        <Menu.Root bind:open={accountMenuOpen}
          ><Menu.Trigger
            >{#snippet child({ props })}<Button
                {...props}
                aria-label={manager.data?.accounts.length
                  ? "Add another GitHub account"
                  : "Add GitHub account"}
                variant="outline"
                disabled={manager.busy || Boolean(manager.flow)}
              >
                {manager.data?.accounts.length
                  ? "Add another account"
                  : "Add account"}<Plus data-icon="inline-end" /></Button
              >{/snippet}</Menu.Trigger
          >
          <Menu.Content align="end" preventScroll={false}
            ><Menu.Group>
              <Menu.Item
                disabled={!manager.data?.oauth_available}
                onSelect={() => void choose("oauth")}
                >Authorize GitHub Account</Menu.Item
              >
              <Menu.Item onSelect={() => void choose("pat")}
                >Add Personal Access Token</Menu.Item
              >
              <Menu.Item onSelect={() => void choose("enterprise")}
                >Add GitHub Enterprise Account</Menu.Item
              >
            </Menu.Group></Menu.Content
          >
        </Menu.Root>
      </StoreSettingsFooter>
    </StoreSettingsCard>
    {#if manager.data && !manager.data.oauth_available}<p
        class="integration-guidance"
      >
        Device sign-in needs a jj-store GitHub OAuth app client ID configured by
        the local service. You can add a personal token now.
      </p>{/if}
    <StoreIntegrationAuth {manager} />
    {#if controller.stackWorkspace?.forge}<StoreForgeRepositoryAccess
        forge={controller.stackWorkspace.forge}
        accounts={manager.data?.accounts ?? []}
      />{/if}
    <p class="integration-guidance">
      <KeyRound aria-hidden="true" /> Credentials are persisted locally in your OS
      Keychain / Credential Manager.
    </p>
  {/if}
</StoreSettingsPage>

<style>
  .integration-account {
    display: flex;
    align-items: center;
    gap: 0.75rem;
    padding: 1rem;
    border-bottom: 1px solid var(--border);
    min-width: 0;
  }
  .integration-account__identity {
    display: flex;
    flex: 1;
    flex-direction: column;
    gap: 0.2rem;
    min-width: 0;
    overflow-wrap: anywhere;
  }
  .integration-account__identity > div {
    display: flex;
    align-items: center;
    gap: 0.5rem;
    flex-wrap: wrap;
  }
  .integration-account__identity > span {
    font-size: 0.8rem;
    color: var(--muted-foreground);
  }
  .integration-caption {
    display: flex;
    align-items: center;
    gap: 0.5rem;
  }
  .integration-guidance {
    display: flex;
    align-items: center;
    gap: 0.5rem;
    font-size: 0.85rem;
    color: var(--muted-foreground);
    margin: 1rem 0;
  }
  .integration-guidance :global(svg) {
    width: 1rem;
    height: 1rem;
    flex-shrink: 0;
  }
  @media (max-width: 40rem) {
    .integration-account {
      flex-wrap: wrap;
    }
    .integration-account__identity {
      min-width: 50%;
    }
  }
</style>

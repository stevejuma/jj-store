<script lang="ts">
  import StoreSettingsCard from "./StoreSettingsCard.svelte";
  import StoreSettingsFooter from "./StoreSettingsFooter.svelte";
  import { untrack } from "svelte";
  import * as Field from "@lapismd/design-core/shadcn/field";
  import * as Dialog from "@lapismd/design-core/shadcn/dialog";
  import * as DropdownMenu from "@lapismd/design-core/shadcn/dropdown-menu";
  import { Button } from "@lapismd/design-core/shadcn/button";
  import { Skeleton } from "@lapismd/design-core/shadcn/skeleton";
  import RefreshCwIcon from "@lucide/svelte/icons/refresh-cw";
  import DownloadIcon from "@lucide/svelte/icons/download";
  import ExternalLinkIcon from "@lucide/svelte/icons/external-link";
  import EllipsisVerticalIcon from "@lucide/svelte/icons/ellipsis-vertical";
  import StoreNotice from "../store-notifications/StoreNotice.svelte";
  import StoreAgentRuntimeSummary from "./StoreAgentRuntimeSummary.svelte";
  import StoreSettingsEmpty from "./StoreSettingsEmpty.svelte";
  import type {
    AgentRuntimes,
    Runtime,
  } from "../../controller/agent-runtimes.svelte.js";
  import "./StoreAgentRuntimes.css";
  let {
    management,
    active = true,
  }: { management?: AgentRuntimes; active?: boolean } = $props();
  let selected = $state<Runtime | null>(null);
  let confirm = $state(false);
  const actionLabel = $derived(
    selected?.action === "update" ? "Update" : "Install",
  );
  const value = $derived(management?.value);
  const blocked = $derived(
    !value?.install_supported ||
      Boolean(management?.pending || management?.saving || management?.error),
  );
  $effect(() => {
    if (!active || !management) return;
    const owner = management;
    untrack(() => owner.start());
    return () => owner.stop();
  });
  function request(runtime: Runtime) {
    selected = runtime;
    confirm = true;
  }
</script>

<svelte:window
  ononline={() => {
    if (active) void management?.refresh();
  }}
/>
<StoreSettingsCard
  id="agent-runtimes-title"
  title="Agent runtimes"
  description="Choose which agent tools jj-store can use on this device."
  Icon={DownloadIcon}
>
  {#if !management || !management.available()}
    <StoreSettingsEmpty
      Icon={DownloadIcon}
      title="Agent runtimes unavailable"
      description="Connect to a local service with administrator access to manage agent runtimes on this device."
    />
  {:else}
    <StoreNotice
      noticeKey="agent-runtime-read"
      title="Could not check agent runtimes"
      message={management.error}
    />
    <StoreNotice
      noticeKey="agent-runtime-command"
      title="Runtime installation failed"
      message={management.pending ? "" : management.commandError}
    />
    <StoreNotice
      noticeKey={`agent-runtime-completed-${management.outcome?.request_id ?? ""}`}
      title="Runtime installed"
      message={management.outcome?.state === "completed"
        ? "Codex tooling is ready for a new connection."
        : ""}
      severity="info"
      lifetime="timed"
    />
    {#if value}
      <div class="store-settings__rows">
        <Field.Group class="store-settings__editable-rows store-runtime__list">
          {#each value.runtimes as runtime (runtime.id)}
            {@const status = management.error ? "unknown" : runtime.state}
            <StoreSettingsFooter>
              {#snippet description()}<StoreAgentRuntimeSummary
                  {runtime}
                  {status}
                />{/snippet}
              {#if runtime.action && runtime.state !== "ready"}
                <Button
                  variant="outline"
                  size="sm"
                  disabled={blocked}
                  onclick={() => request(runtime)}
                  aria-label={`${runtime.action === "update" ? "Update" : "Install"} ${runtime.label} runtime`}
                >
                  <DownloadIcon data-icon="inline-start" />{runtime.action ===
                  "update"
                    ? "Update"
                    : "Install"}
                </Button>
              {:else if runtime.id === "cursor"}
                <a
                  data-ui-component="button"
                  data-slot="button"
                  data-variant="outline"
                  data-size="sm"
                  href={runtime.guide_url}
                  target="_blank"
                  rel="noopener noreferrer"
                >
                  <ExternalLinkIcon
                    data-icon="inline-start"
                  />{runtime.state === "ready"
                    ? "Update guide"
                    : "Install guide"}
                </a>
              {/if}
              <DropdownMenu.Root>
                <DropdownMenu.Trigger>
                  {#snippet child({ props })}<Button
                      {...props}
                      variant="ghost"
                      size="icon-sm"
                      aria-label={`${runtime.label} runtime options`}
                      ><EllipsisVerticalIcon /></Button
                    >{/snippet}
                </DropdownMenu.Trigger>
                <DropdownMenu.Content align="end"
                  ><DropdownMenu.Group>
                    <DropdownMenu.Item>
                      {#snippet child({ props })}<a
                          {...props}
                          href={runtime.guide_url}
                          target="_blank"
                          rel="noopener noreferrer"
                          ><ExternalLinkIcon />Setup guide</a
                        >{/snippet}
                    </DropdownMenu.Item>
                  </DropdownMenu.Group></DropdownMenu.Content
                >
              </DropdownMenu.Root>
            </StoreSettingsFooter>
          {/each}
        </Field.Group>
      </div>
      {#if value.install_unavailable}<p class="store-settings__row-description">
          {value.install_unavailable}
        </p>{/if}
      {#if !value.npm_available || !value.node_version || Number(value.node_version.split(".")[0]) < 22}
        <a
          data-ui-component="button"
          data-slot="button"
          data-variant="outline"
          data-size="sm"
          href="https://nodejs.org/en/download"
          target="_blank"
          rel="noopener noreferrer"
          ><ExternalLinkIcon data-icon="inline-start" />Install Node.js</a
        >
      {/if}
      {#if management.pending}
        <div role="status" class="store-runtime__pending">
          <p>
            {management.commandError ||
              "Installing Codex tooling. You can leave this page; the service retains the original request."}
          </p>
          <Button
            variant="outline"
            size="sm"
            disabled={management.refreshing}
            onclick={() => void management?.refresh()}
            >Check installation</Button
          >
        </div>
      {/if}
    {:else if management.error}
      <StoreSettingsEmpty
        Icon={DownloadIcon}
        title="Runtime status unavailable"
        description="Check again to retrieve the tooling installed on this device."
      />
    {:else}
      <div
        class="store-settings__rows"
        aria-label="Loading agent runtimes"
        aria-busy="true"
      >
        <Field.Group class="store-settings__editable-rows store-runtime__list">
          {#each [0, 1] as row (row)}<Field.Field
              orientation="horizontal"
              class="store-settings__option-row store-runtime__row"
            >
              <Field.Content>
                <div class="store-runtime__summary">
                  <Skeleton class="h-8 w-8 rounded-lg" />
                  <div class="store-runtime__copy">
                    <Skeleton class="h-4 w-40" />
                    <Skeleton class="h-3 w-64 max-w-full" />
                  </div>
                </div>
              </Field.Content>
              <Skeleton class="h-8 w-20" />
            </Field.Field>{/each}
        </Field.Group>
      </div>
    {/if}
    <StoreSettingsFooter>
      {#if management?.available()}<Button
          variant="outline"
          size="sm"
          onclick={() => void management?.refresh()}
          disabled={management.refreshing}
        >
          <RefreshCwIcon data-icon="inline-start" />Check again
        </Button>
      {/if}
    </StoreSettingsFooter>
    <Dialog.Root bind:open={confirm}>
      <Dialog.Content>
        <Dialog.Header
          ><Dialog.Title>{actionLabel} Codex runtime?</Dialog.Title>
          <Dialog.Description
            >Download Codex ACP {selected?.target_version} and Codex CLI {selected?.target_cli_version}
            into jj-store's private device tooling directory. Existing global tools
            and chats are preserved. This does not authenticate or connect an agent.</Dialog.Description
          >
        </Dialog.Header>
        <Dialog.Footer
          ><Button variant="outline" onclick={() => (confirm = false)}
            >Cancel</Button
          >
          <Button
            disabled={blocked}
            onclick={() => {
              confirm = false;
              if (selected) void management?.install(selected);
            }}
            ><DownloadIcon data-icon="inline-start" />{actionLabel} runtime</Button
          >
        </Dialog.Footer>
      </Dialog.Content>
    </Dialog.Root>
  {/if}
</StoreSettingsCard>

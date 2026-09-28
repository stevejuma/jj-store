<script lang="ts">
  import * as Alert from "@lapismd/design-core/shadcn/alert";
  import { Button } from "@lapismd/design-core/shadcn/button";
  import * as Card from "@lapismd/design-core/shadcn/card";
  import * as Field from "@lapismd/design-core/shadcn/field";
  import { Input } from "@lapismd/design-core/shadcn/input";
  import { Spinner } from "@lapismd/design-core/shadcn/spinner";
  import { untrack } from "svelte";
  import type {
    StoreConnection,
    StoreControllerStatus,
  } from "../../controller/contracts.js";

  let {
    status,
    accessMode = "authenticated",
    repositories = [],
    storageReady = false,
    initialRepository = "",
    onConnect,
  }: {
    status: StoreControllerStatus;
    accessMode?: "bootstrapping" | "authenticated" | "local_trusted";
    repositories?: readonly { id: string; name: string }[];
    storageReady?: boolean;
    initialRepository?: string;
    onConnect: (connection: StoreConnection) => Promise<void>;
  } = $props();
  let repository = $state(untrack(() => initialRepository));
  let token = $state("");
  const pending = $derived(
    accessMode === "bootstrapping" || status.connection === "connecting",
  );

  async function connect() {
    const credential = token;
    token = "";
    await onConnect({ repository: repository.trim(), token: credential });
  }

  async function openLocal(repository: string) {
    await onConnect({ repository, token: null });
  }

  function submit(event: SubmitEvent) {
    event.preventDefault();
    void connect();
  }
</script>

<main class="store-connection">
  <Card.Root class="store-connection__card">
    <Card.Header>
      <h1 class="store-connection__title">
        {accessMode === "local_trusted"
          ? "Open a local repository."
          : "Explore native history."}
      </h1>
      <Card.Description>
        {#if accessMode === "bootstrapping"}Checking this jj-store installation…
        {:else if accessMode === "local_trusted"}Repositories registered by this
          local installation. The catalog is discovery metadata; repository
          authority is verified when opened.
        {:else}Connect to a jj-store repository with its repository-scoped
          token. The credential stays in this browser session only.
        {/if}
      </Card.Description>
    </Card.Header>
    <Card.Content>
      {#if accessMode === "bootstrapping"}
        <div
          class="store-connection__loading"
          role="status"
          aria-label="Loading installation status"
        >
          <Spinner aria-hidden="true" />
          <span>Loading installation status</span>
        </div>
      {:else if accessMode === "local_trusted"}
        {#if !storageReady}
          <Alert.Root variant="destructive">
            <Alert.Title>Storage unavailable</Alert.Title>
            <Alert.Description
              >The local authority has not reported ready.</Alert.Description
            >
          </Alert.Root>
        {:else if repositories.length === 0}
          <Alert.Root>
            <Alert.Title>No registered repositories</Alert.Title>
            <Alert.Description
              >Use <code>jj-store repository register</code> after creating or verifying
              a repository.</Alert.Description
            >
          </Alert.Root>
        {:else}
          <div
            class="store-connection__repositories"
            aria-label="Local repositories"
          >
            {#each repositories as candidate (candidate.id)}
              <Button
                variant="outline"
                disabled={pending}
                onclick={() => void openLocal(candidate.id)}
              >
                <span>{candidate.name}</span>
                <small>{candidate.id}</small>
              </Button>
            {/each}
          </div>
        {/if}
      {:else}
        <form onsubmit={submit}>
          <Field.Group>
            <Field.Field>
              <Field.Label for="repository-id">Repository ID</Field.Label>
              <Input
                id="repository-id"
                bind:value={repository}
                autocomplete="off"
                spellcheck="false"
                pattern="[0-9a-f]{32}"
                required
              />
              <Field.Description
                >Full 32-character repository ID</Field.Description
              >
            </Field.Field>
            <Field.Field>
              <Field.Label for="repository-token">Repository token</Field.Label>
              <Input
                id="repository-token"
                type="password"
                bind:value={token}
                autocomplete="off"
                required
              />
            </Field.Field>
            <Button
              type="button"
              disabled={pending || !repository || !token}
              onclick={() => void connect()}
            >
              {#if pending}<Spinner data-icon="inline-start" />{/if}
              {pending ? "Connecting…" : "Open repository"}
            </Button>
          </Field.Group>
        </form>
      {/if}
      {#if status.error}<Alert.Root variant="destructive">
          <Alert.Title>Connection failed</Alert.Title>
          <Alert.Description>{status.error}</Alert.Description>
        </Alert.Root>{/if}
    </Card.Content>
  </Card.Root>
</main>

<style>
  .store-connection {
    display: grid;
    min-height: 100vh;
    place-items: center;
    padding: 1.5rem;
    background: var(--sidebar);
  }
  :global(.store-connection__card) {
    width: min(100%, 30rem);
    background: var(--background);
  }
  .store-connection__title {
    margin: 0;
    font-size: 1.5rem;
    font-weight: 600;
    line-height: 1;
  }
  form,
  :global(.store-connection [data-slot="alert"]) {
    margin-block-start: 1rem;
  }
  .store-connection__loading {
    display: flex;
    align-items: center;
    gap: 0.5rem;
    min-height: 5rem;
    color: var(--muted-foreground);
  }
  .store-connection__repositories {
    display: grid;
    gap: 0.5rem;
  }
  :global(.store-connection__repositories button) {
    justify-content: space-between;
    min-height: 3rem;
  }
  :global(.store-connection__repositories small) {
    color: var(--muted-foreground);
    font-family: var(--font-mono);
  }
</style>

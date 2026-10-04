<script lang="ts">
  import StoreHelp from "../store-help/StoreHelp.svelte";
  import StoreNotice from "../store-notifications/StoreNotice.svelte";
  import StoreRepositoryPicker from "./StoreRepositoryPicker.svelte";
  import * as Alert from "@lapismd/design-core/shadcn/alert";
  import { Button } from "@lapismd/design-core/shadcn/button";
  import * as Card from "@lapismd/design-core/shadcn/card";
  import * as Field from "@lapismd/design-core/shadcn/field";
  import { Input } from "@lapismd/design-core/shadcn/input";
  import { Spinner } from "@lapismd/design-core/shadcn/spinner";
  import type {
    StoreConnection,
    StoreControllerStatus,
  } from "../../controller/contracts.js";

  let {
    status,
    accessMode = "authenticated",
    repositories = [],
    storageReady = false,
    onConnect,
  }: {
    status: StoreControllerStatus;
    accessMode?: "bootstrapping" | "authenticated" | "local_trusted";
    repositories?: readonly { id: string; name: string }[];
    storageReady?: boolean;
    onConnect: (connection: StoreConnection) => Promise<void>;
  } = $props();
  let token = $state("");
  const pending = $derived(
    accessMode === "bootstrapping" || status.connection === "connecting",
  );

  async function connect() {
    const credential = token;
    token = "";
    await onConnect({ repository: "", token: credential });
  }

  async function openRepository(repository: string) {
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
          : repositories.length > 0
            ? "Choose a repository."
            : "Explore native history."}
      </h1>
      <Card.Description>
        {#if accessMode === "bootstrapping"}Checking this jj-store installation…
        {:else if accessMode === "local_trusted"}Repositories registered by this
          local installation. The catalog is discovery metadata; repository
          authority is verified when opened.
        {:else if repositories.length > 0}Repositories currently authorized by
          the token held in this browser session.
        {:else}Enter one manual token to discover every repository it currently
          authorizes. The credential stays in this browser session only.
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
      {:else if !storageReady}
        <Alert.Root variant="destructive">
          <Alert.Title>Storage unavailable</Alert.Title>
          <Alert.Description
            >The repository authority has not reported ready.</Alert.Description
          >
        </Alert.Root>
      {:else if repositories.length > 0}
        <StoreRepositoryPicker
          {repositories}
          label="Repository"
          disabled={pending}
          onChange={(repository) => void openRepository(repository)}
        />
      {:else if accessMode === "local_trusted"}
        <Alert.Root>
          <Alert.Title>No registered repositories</Alert.Title>
          <Alert.Description
            >Create or explicitly register a repository in this installation.</Alert.Description
          >
        </Alert.Root>
      {:else}
        <form onsubmit={submit}>
          <Field.Group>
            <Field.Field>
              <Field.Label for="repository-token">Access token</Field.Label>
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
              disabled={pending || !token}
              onclick={() => void connect()}
            >
              {#if pending}<Spinner data-icon="inline-start" />{/if}
              {pending ? "Connecting…" : "Continue"}
            </Button>
          </Field.Group>
        </form>
      {/if}
      <StoreNotice
        noticeKey="connection"
        title="Connection failed"
        message={status.error}
        {pending}
      />
    </Card.Content>
    <StoreHelp />
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
</style>

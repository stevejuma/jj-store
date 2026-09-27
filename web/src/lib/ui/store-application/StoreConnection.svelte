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
    initialRepository = "",
    onConnect,
  }: {
    status: StoreControllerStatus;
    initialRepository?: string;
    onConnect: (connection: StoreConnection) => Promise<void>;
  } = $props();
  let repository = $state(untrack(() => initialRepository));
  let token = $state("");
  const pending = $derived(status.connection === "connecting");

  async function connect() {
    const credential = token;
    token = "";
    await onConnect({ repository: repository.trim(), token: credential });
  }

  function submit(event: SubmitEvent) {
    event.preventDefault();
    void connect();
  }
</script>

<main class="store-connection">
  <Card.Root class="store-connection__card">
    <Card.Header>
      <h1 class="store-connection__title">Explore native history.</h1>
      <Card.Description
        >Connect to a jj-store repository with its repository-scoped token. The
        credential stays in this browser session only.</Card.Description
      >
    </Card.Header>
    <Card.Content>
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
</style>

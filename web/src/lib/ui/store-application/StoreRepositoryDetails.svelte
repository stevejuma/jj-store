<script lang="ts">
  import type { StoreController } from "../../controller/contracts.js";
  let { controller, title }: { controller: StoreController; title: string } =
    $props();
</script>

<dl>
  <dt>Repository</dt>
  <dd>{controller.state.route.repository}</dd>
  <dt>View</dt>
  <dd>{title}</dd>
  <dt>Source</dt>
  <dd>
    {controller.state.status.connection === "connected"
      ? "Authenticated native repository"
      : "In-memory presentation"}
  </dd>
</dl>

<style>
  dl {
    padding: 0.75rem;
    font-size: var(--community-text-body);
  }
  dd {
    margin: 0 0 1rem;
    color: var(--muted-foreground);
  }
</style>

<script lang="ts">
  import StoreReadOnlyRow from "./StoreReadOnlyRow.svelte";
  import type { ServerSettingsManagement } from "../../controller/server-settings.svelte.js";
  let {
    management,
    section,
  }: { management?: ServerSettingsManagement; section: "service" | "storage" } =
    $props();
</script>

<section
  class="store-settings__section"
  aria-labelledby={`deployment-${section}`}
>
  <header>
    <h2 id={`deployment-${section}`}>
      {section === "service"
        ? "Launch and installation configuration"
        : "Authority storage configuration"}
    </h2>
    <p>
      These values are managed by setup and launch. Operational limits, Search,
      Agents, protection and Sync have editors in their respective sections.
    </p>
  </header>
  {#if management?.available() && management.state.value}
    <dl class="store-settings__rows">
      {#each management.state.value.inventory.filter((v) => v.section === section) as item (item.name)}
        <StoreReadOnlyRow
          label={item.name.replaceAll("_", " ").replaceAll(".", " / ")}
          value={item.saved}
          description={`${item.reason}${item.saved !== item.effective ? ` Running: ${item.effective}` : ""}`}
        />
      {/each}
    </dl>
  {:else}<p>
      Configuration inventory requires a local service with administrator
      access.
    </p>{/if}
</section>

<script lang="ts">
  import * as Field from "@lapismd/design-core/shadcn/field";
  import { Button } from "@lapismd/design-core/shadcn/button";
  import { Input } from "@lapismd/design-core/shadcn/input";
  import StoreWorkField from "../store-work/StoreWorkField.svelte";
  import type {
    AgentCompute,
    ComputeValue,
  } from "../../controller/agent-compute.svelte.js";
  let {
    management,
    status,
    blocked = false,
  }: {
    management?: AgentCompute;
    status: ComputeValue;
    blocked?: boolean;
  } = $props();
  let importPath = $state("");
  const id = $props.id();
</script>

<Field.Group class="community-settings__option-group">
  <StoreWorkField
    label="Import GGUF"
    id={`${id}-import`}
    description="Absolute path on the server machine. The app copies weights into its managed storage."
    ><Input
      id={`${id}-import`}
      bind:value={importPath}
      disabled={blocked || status.state === "serving"}
      placeholder="/absolute/path/model.gguf"
    /></StoreWorkField
  >
  <div class="store-settings__action-row">
    <p>Imported models remain available after restart.</p>
    <Button
      variant="outline"
      disabled={blocked || !importPath || status.state === "serving"}
      onclick={() =>
        management?.command({ action: "import", path: importPath })}
      >Import model</Button
    >
  </div>
  {#each status.models.filter((model) => model.installed) as model (model.id)}<div
      class="store-settings__action-row"
    >
      <p>{model.name}</p>
      <Button
        variant="outline"
        disabled={blocked || status.state === "serving"}
        onclick={() =>
          management?.command({ action: "remove", model: model.id })}
        >Remove weights</Button
      >
    </div>{/each}
</Field.Group>

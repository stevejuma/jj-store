<script lang="ts">
  import FixtureControls from "./FixtureControls.svelte";
  import { workflowHostFixtureController } from "./workflow-host-settings.js";
  import { RuntimeReload } from "../../web/src/lib/controller/runtime-reload.svelte.js";
  import StoreRuntimeReload from "../../web/src/lib/ui/store-settings/StoreRuntimeReload.svelte";
  import { untrack, onMount } from "svelte";
  import StoreWorkflowHostSettings from "../../web/src/lib/ui/store-settings/StoreWorkflowHostSettings.svelte";
  import StoreSettingsPage from "../../web/src/lib/ui/store-settings/StoreSettingsPage.svelte";
  import StoreDeploymentConfiguration from "../../web/src/lib/ui/store-settings/StoreDeploymentConfiguration.svelte";
  import StoreConfigurationEditor from "../../web/src/lib/ui/store-settings/StoreConfigurationEditor.svelte";
  import { createServerSettings } from "../../web/src/lib/controller/server-settings.svelte.js";
  import {
    copy,
    type Section,
    type SaveServerSettings,
  } from "../../web/src/lib/controller/server-configuration.js";
  import { serverSettingsValue } from "./server-settings.js";
  let {
    inventorySection,
    section = "service",
    supported = true,
    unknown = false,
    embedded = false,
    loading = false,
    lookupFailure,
  }: {
    inventorySection?: "service" | "storage";
    section?: Section;
    supported?: boolean;
    unknown?: boolean;
    embedded?: boolean;
    loading?: boolean;
    lookupFailure?: "profiles";
  } = $props();
  const controller = untrack(() =>
    workflowHostFixtureController(lookupFailure === "profiles"),
  );
  let value = serverSettingsValue();
  let finishRead: (() => void) | undefined;
  const management = createServerSettings({
    available: () => supported,
    client: {
      close() {},
      request: (async (_path: string, body?: SaveServerSettings) => {
        if (body) {
          if (unknown) throw new Error("lost response");
          value = {
            ...copy(value),
            desired: {
              ...copy(value.desired),
              [body.update.section]: copy(body.update.configuration),
            },
            revision: "cd".repeat(32),
            restart_required: true,
          };
        }
        if (loading && !body)
          await new Promise<void>((resolve) => (finishRead = resolve));
        return copy(value);
      }) as never,
    },
  });
  const reload = new RuntimeReload(
    () => section === "workflow" && supported,
    () => null,
    async (_path, init) => {
      const body = init?.body ? JSON.parse(String(init.body)) : null;
      if (body) {
        value.effective.workflow = copy(value.desired.workflow);
        return Response.json({
          request_id: body.request_id,
          generation: 1,
          state: "applied",
          results: [{ domain: "workflows", state: "applied", reason: null }],
        });
      }
      return Response.json({
        saved_revision: value.revision,
        supported: ["workflows"],
        pending: ["workflows"],
        generation: 0,
        original: null,
        outcome: null,
      });
    },
    async () => {
      await management.observe();
    },
  );
  onMount(() => {
    void management.observe();
    return () => {
      management.destroy();
      reload.destroy();
    };
  });
</script>

{#if loading}
  <FixtureControls floating={false}
    ><button onclick={() => finishRead?.()}>Finish loading</button
    ></FixtureControls
  >
  <FixtureControls floating={false}
    ><button onclick={() => void management.observe()}
      >Start background refresh</button
    ></FixtureControls
  >
{/if}

{#if section === "workflow"}<StoreRuntimeReload manager={reload} menu />{/if}
<StoreSettingsPage
  {embedded}
  section={section === "service" || section === "workflow"
    ? "service"
    : section === "sync"
      ? "sync"
      : "repository"}
  onRefresh={management.observe}
>
  {#if inventorySection}<StoreDeploymentConfiguration
      {management}
      section={inventorySection}
    />{:else if section === "workflow"}<StoreWorkflowHostSettings
      {management}
      {reload}
      {controller}
    />{:else}<StoreConfigurationEditor
      {management}
      {section}
      repository={"01".repeat(16)}
      repositoryName="Local project"
    />{/if}
</StoreSettingsPage>

<script lang="ts">
  import { onMount } from "svelte";
  import StoreSettingsPage from "../../web/src/lib/ui/store-settings/StoreSettingsPage.svelte";
  import StoreConfigurationEditor from "../../web/src/lib/ui/store-settings/StoreConfigurationEditor.svelte";
  import { createServerSettings } from "../../web/src/lib/controller/server-settings.svelte.js";
  import {
    copy,
    type Section,
    type SaveServerSettings,
  } from "../../web/src/lib/controller/server-configuration.js";
  import { serverSettingsValue } from "./server-settings.js";
  let {
    section = "service",
    supported = true,
    unknown = false,
  }: { section?: Section; supported?: boolean; unknown?: boolean } = $props();
  let value = serverSettingsValue();
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
        return copy(value);
      }) as never,
    },
  });
  onMount(() => {
    void management.observe();
    return () => management.destroy();
  });
</script>

<StoreSettingsPage
  section={section === "service"
    ? "service"
    : section === "sync"
      ? "sync"
      : "repository"}
  onRefresh={management.observe}
>
  <StoreConfigurationEditor
    {management}
    {section}
    repository={"01".repeat(16)}
    repositoryName="Local project"
  />
</StoreSettingsPage>

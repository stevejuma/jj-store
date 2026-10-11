<script lang="ts">
  import { untrack } from "svelte";
  import { AppShell, AppShellController } from "@lapismd/design-core/shell";
  import StoreGlobalSettingsLoading from "../../web/src/lib/ui/store-settings/StoreGlobalSettingsLoading.svelte";
  import type { GlobalSettingsSection } from "../../web/src/lib/controller/global-settings-navigation.js";
  let {
    section = "agents",
    tab = "runtimes",
    narrow = false,
  }: {
    section?: GlobalSettingsSection;
    tab?: string;
    narrow?: boolean;
  } = $props();
  let selected = $state(untrack(() => tab));
  const shell = new AppShellController();
</script>

<div
  data-testid="loading-frame"
  style:width={narrow ? "320px" : "900px"}
  style:max-width="100%"
  style:height="700px"
>
  <AppShell.Root
    controller={shell}
    desktopMinMainWidth={0}
    style="--ui-shell-height: 100%; --ui-shell-main-block-inset: 0px"
  >
    <AppShell.Main
      ><StoreGlobalSettingsLoading
        {section}
        tab={selected}
        onSelect={(next) => (selected = next)}
      /></AppShell.Main
    >
  </AppShell.Root>
</div>

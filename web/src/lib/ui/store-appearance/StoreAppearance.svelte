<script lang="ts">
  import * as Tabs from "@lapismd/design-core/shadcn/tabs";
  import StoreSettingsPage from "../store-settings/StoreSettingsPage.svelte";
  import { onDestroy } from "svelte";
  import { useAppearance } from "../../appearance/store.svelte.js";
  import AppearanceThemes from "./AppearanceThemes.svelte";
  import AppearancePreferences from "./AppearancePreferences.svelte";
  import "@fontsource-variable/inter";
  import "./appearance.css";
  import "./appearance-preview.css";
  import "./theme-preview.css";
  const appearance = useAppearance();
  let active = $state("theme");
  onDestroy(() => appearance?.resetPreview());
</script>

<StoreSettingsPage
  section="appearance"
  bind:value={active}
  tabs={[
    { value: "theme", label: "Theme" },
    { value: "preferences", label: "Preferences" },
  ]}
>
  <section class="store-appearance" aria-label="Appearance settings">
    <Tabs.Content value="theme"><AppearanceThemes /></Tabs.Content>
    <Tabs.Content value="preferences"><AppearancePreferences /></Tabs.Content>
  </section>
</StoreSettingsPage>

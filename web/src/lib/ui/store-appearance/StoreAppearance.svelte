<script lang="ts">
  import { onDestroy } from "svelte";
  import { useAppearance } from "../../appearance/store.svelte.js";
  import AppearanceThemes from "./AppearanceThemes.svelte";
  import AppearancePreferences from "./AppearancePreferences.svelte";
  import "@fontsource-variable/inter";
  import "./appearance.css";
  import "./appearance-preview.css";
  import "./theme-preview.css";
  const appearance = useAppearance();
  onDestroy(() => appearance?.resetPreview());
</script>

<section class="store-appearance" aria-label="Appearance settings">
  <header>
    <h1>Appearance</h1>
    <p>Choose how jj-store looks and feels.</p>
  </header>
  <div class="appearance-groups">
    <AppearanceThemes /><AppearancePreferences />
  </div>
</section>

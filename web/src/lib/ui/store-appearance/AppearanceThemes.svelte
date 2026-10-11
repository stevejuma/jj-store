<script lang="ts">
  import StoreSettingsCard from "../store-settings/StoreSettingsCard.svelte";
  import SectionIcon from "@lucide/svelte/icons/palette";
  import SunMoon from "@lucide/svelte/icons/sun-moon";
  import Sun from "@lucide/svelte/icons/sun";
  import Moon from "@lucide/svelte/icons/moon";
  import ChevronDown from "@lucide/svelte/icons/chevron-down";
  import RotateCcw from "@lucide/svelte/icons/rotate-ccw";
  import { Switch } from "@lapismd/design-core/shadcn/switch";
  import { Slider } from "@lapismd/design-core/shadcn/slider";
  import { Button } from "@lapismd/design-core/shadcn/button";
  import { useAppearance } from "../../appearance/store.svelte.js";
  import {
    themeNames,
    pairedThemes,
    lightThemes,
    themePairs,
    themeLabel,
    accents,
    type ThemeName,
  } from "../../appearance/theme-catalog.js";
  import type { ColorMode } from "../../appearance/preferences.js";
  import AppearanceChoice from "./AppearanceChoice.svelte";
  import AppearanceRow from "./AppearanceRow.svelte";
  import ThemePreview from "./ThemePreview.svelte";
  const store = useAppearance()!;
  let expanded = $state(false);
  const paired = $derived(store.saved.mode === "system");
  const buzz = $derived(store.theme === "buzz" || store.theme === "buzz-dark");
  const original = $derived(
    store.theme === "original" || store.theme === "original-dark",
  );
  const selected = $derived(
    paired && !lightThemes.has(store.saved.theme)
      ? (themePairs.get(store.saved.theme) ?? store.saved.theme)
      : store.saved.theme,
  );
  const themes = $derived(
    paired
      ? pairedThemes
      : store.saved.mode === "light"
        ? [
            ...pairedThemes,
            ...themeNames.filter(
              (n) => lightThemes.has(n) && !themePairs.has(n),
            ),
          ]
        : [
            ...pairedThemes.map((n) => themePairs.get(n)!),
            ...themeNames.filter(
              (n) => !lightThemes.has(n) && !themePairs.has(n),
            ),
          ],
  );
</script>

<StoreSettingsCard id="appearance-theme" title="Theme" Icon={SectionIcon}>
  <div class="appearance-card appearance-card--settings">
    <AppearanceRow
      title="Color mode"
      description="Follow your system or choose a light or dark appearance."
    >
      <AppearanceChoice
        label="Color mode"
        value={store.saved.mode}
        options={[
          { value: "system", label: "System", icon: SunMoon },
          { value: "light", label: "Light", icon: Sun },
          { value: "dark", label: "Dark", icon: Moon },
        ]}
        onChange={(v) => store.mode(v as ColorMode)}
      />
    </AppearanceRow>
    <AppearanceRow
      title="Theme style"
      description="Choose the colors used throughout jj-store."
    >
      <button
        class="appearance-theme-trigger"
        type="button"
        aria-label={`Theme style, ${themeLabel(selected)}`}
        aria-expanded={expanded}
        aria-controls="appearance-theme-options"
        onclick={() => (expanded = !expanded)}
        ><ThemePreview theme={selected} {paired} /><ChevronDown
          aria-hidden="true"
        /></button
      >
    </AppearanceRow>
    {#if expanded}<div
        id="appearance-theme-options"
        class="appearance-theme-gallery"
        aria-label="Theme styles"
      >
        {#each themes as theme (theme)}<button
            type="button"
            class="appearance-theme-tile"
            aria-label={`Use ${themeLabel(theme)} theme`}
            aria-pressed={selected === theme}
            onclick={() => store.update({ theme })}
            ><ThemePreview {theme} {paired} /><span>{themeLabel(theme)}</span
            ></button
          >{/each}
      </div>{/if}
    {#if !buzz && !original}<AppearanceRow
        title="Accent color"
        description="Choose the highlight color used throughout jj-store."
      >
        <div class="appearance-accents" aria-label="Accent colors">
          {#each accents as color}<button
              type="button"
              data-accent={color.name.toLowerCase()}
              aria-label={`Use ${color.name} accent`}
              title={color.name}
              aria-pressed={store.saved.accent === color.value}
              onclick={() => store.update({ accent: color.value })}
              ><span></span></button
            >{/each}
        </div>
      </AppearanceRow>{/if}
    <AppearanceRow
      title="Glass background"
      description="Blur the app background behind navigation while keeping content solid."
      ><Switch
        aria-label="Glass background"
        checked={store.saved.glass}
        onCheckedChange={(glass) => store.update({ glass })}
      /></AppearanceRow
    >
    {#if store.saved.glass}<AppearanceRow
        title="Glass opacity"
        description="Lower values reveal more of the background blur."
      >
        <div class="appearance-opacity">
          <Slider
            type="single"
            min={30}
            max={90}
            step={1}
            value={store.saved.opacity}
            aria-label="Glass opacity"
            onValueChange={(opacity) => store.update({ opacity })}
          /><output>{store.saved.opacity}%</output><Button
            variant="ghost"
            size="icon-sm"
            aria-label="Reset glass opacity"
            title="Reset glass opacity"
            onclick={() => store.update({ opacity: 65 })}
            ><RotateCcw aria-hidden="true" /></Button
          >
        </div>
      </AppearanceRow>{/if}
    {#if buzz}<AppearanceRow
        title="Prominent active tab"
        description="Give the selected navigation item a higher-contrast background."
        ><Switch
          aria-label="Prominent active tab"
          checked={store.saved.prominent}
          onCheckedChange={(prominent) => store.update({ prominent })}
        /></AppearanceRow
      >{/if}
  </div>
</StoreSettingsCard>

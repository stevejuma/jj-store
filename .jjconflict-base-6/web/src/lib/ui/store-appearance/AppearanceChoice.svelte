<script lang="ts">
  import type { Component } from "svelte";
  let {
    label,
    value,
    options,
    onChange,
    onPreview,
  }: {
    label: string;
    value: string;
    options: readonly { value: string; label: string; icon?: Component }[];
    onChange: (value: string) => void;
    onPreview?: (value: string | null) => void;
  } = $props();
  function keys(event: KeyboardEvent) {
    if (!["ArrowLeft", "ArrowRight", "Home", "End"].includes(event.key)) return;
    event.preventDefault();
    const buttons = Array.from(
      event.currentTarget instanceof HTMLElement
        ? event.currentTarget.querySelectorAll<HTMLButtonElement>("button")
        : [],
    );
    let index = buttons.indexOf(document.activeElement as HTMLButtonElement);
    index =
      event.key === "Home"
        ? 0
        : event.key === "End"
          ? buttons.length - 1
          : (index + (event.key === "ArrowRight" ? 1 : -1) + buttons.length) %
            buttons.length;
    buttons[index]?.focus();
    buttons[index]?.click();
  }
</script>

<!-- A single-tab-stop radiogroup with explicit arrow-key selection. -->
<div
  class="appearance-choice"
  role="radiogroup"
  tabindex="-1"
  aria-label={label}
  onkeydown={keys}
>
  {#each options as option (option.value)}
    <button
      type="button"
      role="radio"
      aria-checked={value === option.value}
      tabindex={value === option.value ? 0 : -1}
      onpointerenter={() => onPreview?.(option.value)}
      onpointerleave={() => onPreview?.(null)}
      onfocus={() => onPreview?.(option.value)}
      onblur={() => onPreview?.(null)}
      onclick={() => onChange(option.value)}
    >
      {#if option.icon}<option.icon aria-hidden="true" />{/if}{option.label}
    </button>
  {/each}
</div>

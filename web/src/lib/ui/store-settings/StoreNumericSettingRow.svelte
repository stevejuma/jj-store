<script lang="ts">
  import * as Field from "@lapismd/design-core/shadcn/field";
  import * as InputGroup from "@lapismd/design-core/shadcn/input-group";
  import * as Select from "@lapismd/design-core/shadcn/select";
  import { Slider } from "@lapismd/design-core/shadcn/slider";
  import RotateCcw from "@lucide/svelte/icons/rotate-ccw";
  import CircleAlert from "@lucide/svelte/icons/circle-alert";
  import {
    canonicalNumericValue,
    displayNumericValue,
    numericSettingError,
    type NumericUnit,
  } from "./numeric-setting.js";
  import "./numeric-setting.css";

  let {
    id,
    label,
    description,
    value,
    units,
    preferredUnit,
    min,
    max,
    defaultValue,
    disabled = false,
    readonly = false,
    error = "",
    slider = false,
    density = "regular",
    onValueChange,
  }: {
    id: string;
    label: string;
    description?: string;
    value: string;
    units: readonly NumericUnit[];
    preferredUnit?: string;
    min: number;
    max: number;
    defaultValue?: number;
    disabled?: boolean;
    readonly?: boolean;
    error?: string;
    slider?: boolean;
    density?: "regular" | "compact";
    onValueChange: (canonical: string) => void;
  } = $props();
  let selected = $state("");
  let edit = $state<{ canonical: string; text: string; unit: string }>();
  let input = $state<HTMLInputElement | null>(null);
  const unit = $derived(
    units.find((u) => u.id === (selected || preferredUnit)) ?? units[0],
  );
  const display = $derived(displayNumericValue(value, unit));
  const text = $derived(
    edit?.canonical === value && edit.unit === unit.id
      ? edit.text
      : display.text,
  );
  const validation = $derived(
    error || numericSettingError(value, min, max, units[0].label),
  );
  const changed = $derived(
    defaultValue !== undefined &&
      (value.trim() === "" || Number(value) !== defaultValue),
  );
  const showExact = $derived(!validation && unit.id !== units[0].id);
  const blocked = $derived(disabled || readonly);
  const helpId = $derived(`${id}-help`);
  const stateId = $derived(`${id}-state`);
  const errorId = $derived(`${id}-error`);
  $effect(() => {
    input?.setCustomValidity(validation);
  });
  function update(text: string) {
    if (blocked) return;
    const canonical = canonicalNumericValue(text, unit);
    edit = { canonical, text, unit: unit.id };
    onValueChange(canonical);
  }
</script>

<Field.Field
  orientation="responsive"
  class="store-settings__option-row store-numeric-setting"
  data-density={density}
  data-invalid={validation ? true : undefined}
  data-disabled={disabled ? true : undefined}
>
  <Field.Content>
    <Field.Label for={id}>{label}</Field.Label>
    {#if description}<Field.Description id={helpId}
        >{description}</Field.Description
      >{/if}
    {#if defaultValue !== undefined}<p class="store-numeric-setting__default">
        Default: {defaultValue.toLocaleString("en-GB")}
        {units[0].label}
      </p>{/if}
  </Field.Content>
  <div class="store-numeric-setting__controls">
    <div class="store-numeric-setting__entry">
      <InputGroup.Root class="store-numeric-setting__input" aria-label={label}>
        <InputGroup.Input
          bind:ref={input}
          {id}
          type="number"
          required
          step="any"
          min={Number(displayNumericValue(String(min), unit).text)}
          max={Number(displayNumericValue(String(max), unit).text)}
          {disabled}
          {readonly}
          value={text}
          aria-invalid={validation ? true : undefined}
          aria-describedby={`${description ? helpId : ""} ${showExact ? stateId : ""} ${validation ? errorId : ""}`.trim()}
          oninput={(event) => update(event.currentTarget.value)}
        />
        <InputGroup.Addon
          align="inline-end"
          class="store-numeric-setting__unit"
        >
          {#if units.length > 1}
            <Select.Root
              type="single"
              value={unit.id}
              disabled={blocked}
              onValueChange={(next) => {
                if (next) {
                  selected = next;
                  edit = undefined;
                }
              }}
            >
              <Select.Trigger
                aria-label={`${label} unit`}
                class="store-numeric-setting__unit-trigger"
                >{unit.label}</Select.Trigger
              >
              <Select.Content
                ><Select.Group>
                  {#each units as option (option.id)}<Select.Item
                      value={option.id}
                      label={option.label}>{option.label}</Select.Item
                    >{/each}
                </Select.Group></Select.Content
              >
            </Select.Root>
          {:else}<InputGroup.Text>{unit.label}</InputGroup.Text>{/if}
          {#if defaultValue !== undefined && changed}<InputGroup.Button
              type="button"
              variant="ghost"
              class="store-numeric-setting__reset"
              size="icon-sm"
              disabled={blocked}
              aria-label={`Reset ${label} to default`}
              title={`Reset to ${defaultValue.toLocaleString("en-GB")} ${units[0].label}`}
              onclick={() => {
                edit = undefined;
                input?.focus();
                onValueChange(String(defaultValue));
              }}><RotateCcw aria-hidden="true" /></InputGroup.Button
            >{/if}
        </InputGroup.Addon>
      </InputGroup.Root>
    </div>
    {#if slider}
      <Slider
        type="single"
        {min}
        {max}
        step={1}
        value={validation ? min : Number(value)}
        disabled={blocked || Boolean(validation)}
        aria-label={`${label} slider`}
        onValueChange={(next) => {
          if (!blocked) {
            edit = undefined;
            onValueChange(String(next));
          }
        }}
      />
      <div class="store-numeric-setting__range">
        <span>{min.toLocaleString("en-GB")}</span><span
          >{max.toLocaleString("en-GB")} {units[0].label}</span
        >
      </div>
    {/if}
    {#if showExact}<p id={stateId} class="store-numeric-setting__state">
        {display.approximate
          ? "Approximate display; exact value: "
          : "Exact value: "}{Number(value).toLocaleString("en-GB")}
        {units[0].label}
      </p>{/if}
    {#if validation}<p
        id={errorId}
        class="store-numeric-setting__error"
        role="alert"
      >
        <CircleAlert aria-hidden="true" />{validation}
      </p>{/if}
  </div>
</Field.Field>

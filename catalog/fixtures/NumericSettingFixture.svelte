<script lang="ts">
  import * as Field from "@lapismd/design-core/shadcn/field";
  import { Button } from "@lapismd/design-core/shadcn/button";
  import StoreNumericSettingRow from "../../web/src/lib/ui/store-settings/StoreNumericSettingRow.svelte";
  import {
    bytesUnits,
    millisecondsUnits,
  } from "../../web/src/lib/ui/store-settings/numeric-setting.js";
  let {
    value = $bindable("1000"),
    bytes = false,
    disabled = false,
    readonly = false,
    slider = false,
    sliderStep = 1,
    min = 0,
    density = "regular",
    defaultValue = 1000,
    narrow = false,
  }: {
    value?: string;
    bytes?: boolean;
    disabled?: boolean;
    readonly?: boolean;
    slider?: boolean;
    sliderStep?: number;
    min?: number;
    density?: "regular" | "compact";
    defaultValue?: number;
    narrow?: boolean;
  } = $props();
  let changes = $state(0);
  let saved = $state("");
</script>

<form
  style:max-width={narrow ? "320px" : "760px"}
  onsubmit={(event) => {
    event.preventDefault();
    saved = value;
  }}
>
  <Field.Group>
    <StoreNumericSettingRow
      id="numeric-value"
      label={bytes ? "Maximum text file size" : "Polling interval"}
      description="Configure the worker limit."
      {value}
      units={bytes ? bytesUnits : millisecondsUnits}
      {min}
      max={bytes ? 1048576 : 60000}
      {defaultValue}
      {disabled}
      {readonly}
      {slider}
      {sliderStep}
      {density}
      onValueChange={(next) => {
        value = next;
        changes += 1;
      }}
    />
  </Field.Group>
  <Button type="submit">Save</Button>
  <p data-testid="canonical">{value}</p>
  <p data-testid="changes">{changes}</p>
  <p data-testid="saved">{saved}</p>
</form>

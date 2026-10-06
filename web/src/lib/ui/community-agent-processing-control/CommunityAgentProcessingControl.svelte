<!-- Community card processing control; jj-store owns explicit lifecycle commands. -->
<script lang="ts">
  import { Button } from "@lapismd/design-core/shadcn/button";
  import { Spinner } from "@lapismd/design-core/shadcn/spinner";
  let {
    label,
    accessibleLabel,
    disabled = false,
    pending = false,
    onChange,
  }: {
    label: string;
    accessibleLabel: string;
    disabled?: boolean;
    pending?: boolean;
    onChange: () => void;
  } = $props();
</script>

<div
  class="agent-processing"
  data-presentation="card"
  aria-label="Agent processing"
>
  <Button
    class="agent-processing__card-button"
    disabled={disabled || pending}
    aria-label={accessibleLabel}
    aria-busy={pending}
    onclick={onChange}
    >{#if pending}<Spinner aria-hidden="true" />{:else}{label}{/if}</Button
  >
</div>

<style>
  .agent-processing {
    display: grid;
    gap: 0.375rem;
    min-width: 0;
  }
  :global(.agent-processing__card-button) {
    border: 1px solid var(--border);
    border-radius: 999px;
    padding-inline: 1rem;
    min-width: 4rem;
    box-shadow: var(--shadow-sm, 0 1px 2px rgb(0 0 0 / 0.1));
  }
</style>

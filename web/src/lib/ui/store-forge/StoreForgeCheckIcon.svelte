<script lang="ts">
  import CircleCheck from "@lucide/svelte/icons/circle-check";
  import CircleX from "@lucide/svelte/icons/circle-x";
  import LoaderCircle from "@lucide/svelte/icons/loader-circle";
  import CircleMinus from "@lucide/svelte/icons/circle-minus";
  import Clock from "@lucide/svelte/icons/clock";
  import CircleHelp from "@lucide/svelte/icons/circle-help";
  import Hand from "@lucide/svelte/icons/hand";
  let { state }: { state: string } = $props();
  const Icon = $derived(
    state === "passed"
      ? CircleCheck
      : state === "failed"
        ? CircleX
        : state === "running"
          ? LoaderCircle
          : state === "queued"
            ? Clock
            : state === "manual"
              ? Hand
              : ["skipped", "cancelled"].includes(state)
                ? CircleMinus
                : CircleHelp,
  );
</script>

<span class="forge-check-icon" data-state={state}
  ><Icon size={16} aria-hidden="true" /></span
>

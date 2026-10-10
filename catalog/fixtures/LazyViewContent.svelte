<script lang="ts">
  let { label, open }: { label: string; open: boolean } = $props();
  let draft = $state("");
</script>

<section aria-label={`${label} content`} hidden={!open}>
  <label>{label} draft <input bind:value={draft} /></label>
</section>

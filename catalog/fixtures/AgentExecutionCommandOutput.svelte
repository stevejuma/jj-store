<script lang="ts">
  let {
    commands,
    lastCommand,
  }: {
    commands: Record<string, unknown>[];
    lastCommand: Record<string, unknown> | null | undefined;
  } = $props();
</script>

{#if lastCommand}
  <output aria-label="Session commands">{JSON.stringify(commands)}</output>
  <output aria-label="Last session command"
    >{JSON.stringify(lastCommand)}</output
  >
{/if}

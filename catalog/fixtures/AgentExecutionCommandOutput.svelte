<script lang="ts">
  let {
    commands,
    lastCommand,
    lastOpened,
  }: {
    commands: Record<string, unknown>[];
    lastCommand: Record<string, unknown> | null | undefined;
    lastOpened?: { repository: string; commit: string; operation: string };
  } = $props();
</script>

{#if lastCommand}
  <output aria-label="Session commands">{JSON.stringify(commands)}</output>
  <output aria-label="Last session command"
    >{JSON.stringify(lastCommand)}</output
  >
{/if}
{#if lastOpened}<output aria-label="Last opened revision"
    >{JSON.stringify(lastOpened)}</output
  >{/if}

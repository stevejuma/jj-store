<script lang="ts">
  import * as Alert from "@lapismd/design-core/shadcn/alert";
  import type { StoreMutationPrepared } from "../../controller/store-mutation-types.js";
  let { result }: { result: StoreMutationPrepared } = $props();
  const bookmarkNames = $derived(Object.keys(result.bookmarks));
</script>

<Alert.Root>
  <Alert.Title>Preview prepared</Alert.Title>
  <Alert.Description>
    Review the provisional native effects before publishing this request.
  </Alert.Description>
</Alert.Root>
<dl class="store-mutation-preview">
  <dt>Operation</dt>
  <dd>{result.operation.slice(0, 12)}…</dd>
  <dt>Created commits</dt>
  <dd>{result.created.length}</dd>
  <dt>Rewritten commits</dt>
  <dd>{result.rewrites.length}</dd>
  <dt>Staged objects</dt>
  <dd>{result.staged_objects}</dd>
  <dt>Staged bytes</dt>
  <dd>{result.staged_bytes}</dd>
  {#if bookmarkNames.length}<dt>Bookmarks</dt>
    <dd>{bookmarkNames.join(", ")}</dd>{/if}
  {#if result.abandoned?.length}<dt>Abandoned commits</dt>
    <dd>{result.abandoned.length}</dd>{/if}
  {#if result.conflict_resolution}<dt>Conflict paths</dt>
    <dd>{result.conflict_resolution.selected_paths.join(", ")}</dd>
    <dt>Remaining conflicts</dt>
    <dd>{Object.values(result.conflict_resolution.remaining).flat().length}</dd>
  {/if}
</dl>

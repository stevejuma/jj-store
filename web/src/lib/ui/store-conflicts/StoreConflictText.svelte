<script lang="ts">
  import type { ComponentProps } from "svelte";
  import { MergeEditor } from "@lapismd/design-core/diff";
  import { repositoryEditorLanguage } from "../community-projects/repository-editor-language.js";
  let { path, ...props }: ComponentProps<typeof MergeEditor> = $props();
</script>

<MergeEditor
  {path}
  language={repositoryEditorLanguage(path ?? "").id}
  baseLabel="Resolved"
  editableSides={["base"]}
  {...props}
/>

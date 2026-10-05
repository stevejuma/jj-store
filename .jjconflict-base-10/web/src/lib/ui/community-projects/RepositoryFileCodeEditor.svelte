<script lang="ts">
  import { CodeEditor } from "@lapismd/design-core/forms";

  import { repositoryEditorLanguage } from "./repository-editor-language.js";

  let {
    path,
    value = $bindable(""),
    onChange,
  }: {
    path: string;
    value?: string;
    onChange?: (value: string) => void;
  } = $props();

  const language = $derived(repositoryEditorLanguage(path));
</script>

<div class="store-file-editor__contents" data-source-language={language.name}>
  <CodeEditor
    bind:value
    language={language.id}
    extensions={language.extension}
    ariaLabel={`Editing ${path} file contents`}
    minHeight="28rem"
    syntaxDiagnostics={false}
    {onChange}
  />
</div>

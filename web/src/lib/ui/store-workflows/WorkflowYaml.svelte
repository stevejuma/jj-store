<script lang="ts">
  import { CodeEditor } from "@lapismd/design-core/forms";
  import { repositoryEditorLanguage } from "../community-projects/repository-editor-language.js";
  import { EditorState } from "@codemirror/state";
  import { EditorView } from "@codemirror/view";
  let {
    value = $bindable(""),
    disabled = false,
  }: { value?: string; disabled?: boolean } = $props();
  const language = repositoryEditorLanguage("workflow.yml");
</script>

<CodeEditor
  bind:value
  language={language.id}
  extensions={[
    language.extension,
    EditorState.readOnly.of(disabled),
    EditorView.editable.of(!disabled),
  ]}
  ariaLabel="Workflow YAML"
  minHeight="28rem"
  syntaxDiagnostics={false}
/>

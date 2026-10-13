<script lang="ts">
  import { CodeEditor } from "@lapismd/design-core/forms";
  import { EditorState } from "@codemirror/state";
  import { EditorView } from "@codemirror/view";
  let {
    value = $bindable(""),
    disabled = false,
    onChange,
  }: {
    value?: string;
    disabled?: boolean;
    onChange(value: string): void;
  } = $props();
</script>

<CodeEditor
  bind:value
  language="json"
  ariaLabel="Host configuration JSON"
  minHeight="18rem"
  syntaxDiagnostics={false}
  extensions={[
    EditorState.readOnly.of(disabled),
    EditorView.editable.of(!disabled),
  ]}
  {onChange}
/>

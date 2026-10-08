import { EditorView } from "@codemirror/view";

/** Shared reading/draft caret; hosts select its semantic theme color. */
export const chatCaret = EditorView.theme({
  "&.cm-editor .cm-cursor": {
    borderInlineStartWidth: "2px",
    borderInlineStartColor: "var(--chat-caret-color, var(--muted-foreground))",
    transform: "scaleY(1.2)",
    transformOrigin: "center",
  },
});

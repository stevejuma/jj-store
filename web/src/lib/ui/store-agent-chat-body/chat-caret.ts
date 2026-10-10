import { EditorView } from "@codemirror/view";

/** Shared reading/draft caret; hosts select its semantic theme color. */
export const chatCaret = EditorView.theme({
  "&.cm-editor .cm-cursor": {
    borderInlineStartWidth: "2px",
    borderInlineStartColor: "var(--chat-caret-color, var(--muted-foreground))",
    // Zero-margin document surfaces clip the default half-stroke overhang.
    marginInlineStart: "0",
    transform: "scaleY(1.2)",
    transformOrigin: "center",
  },
});

/** Document drafts need clearance inside their intentionally frameless host. */
export const chatDraftCaret = [
  chatCaret,
  EditorView.theme({
    "&.cm-editor > .cm-scroller > .cm-content": {
      paddingBlock: "0.25em",
      paddingInline: "0.125em",
    },
  }),
];

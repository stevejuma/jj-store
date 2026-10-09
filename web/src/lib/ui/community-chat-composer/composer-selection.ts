import type { EditorView } from "@codemirror/view";

/** Retain the draft selection while its presentation remounts the editor. */
export function createComposerSelection(onView: (view: EditorView) => void) {
  let selection = { anchor: 0, head: 0 };
  return {
    onView(view: EditorView) {
      onView(view);
      const anchor = Math.min(selection.anchor, view.state.doc.length);
      const head = Math.min(selection.head, view.state.doc.length);
      if (anchor || head) view.dispatch({ selection: { anchor, head } });
    },
    onSelection({ view }: { view: EditorView }) {
      const { anchor, head } = view.state.selection.main;
      selection = { anchor, head };
    },
  };
}

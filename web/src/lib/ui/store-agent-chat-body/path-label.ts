import { WidgetType, type EditorView } from "@codemirror/view";

/** Presentation only: the captured document and its comment offsets stay exact. */
export class PathLabel extends WidgetType {
  constructor(readonly label: string) {
    super();
  }
  eq(other: PathLabel) {
    return this.label === other.label;
  }
  toDOM(view: EditorView) {
    const span = view.dom.ownerDocument.createElement("span");
    span.textContent = this.label;
    return span;
  }
}

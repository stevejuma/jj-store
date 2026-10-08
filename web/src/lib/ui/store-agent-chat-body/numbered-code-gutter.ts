import { Decoration, EditorView, GutterMarker, gutter } from "@codemirror/view";
import { capturedLinePrefixes } from "./captured-code.js";

class CapturedLineNumber extends GutterMarker {
  constructor(readonly number: string) {
    super();
  }
  eq(other: CapturedLineNumber) {
    return this.number === other.number;
  }
  toDOM() {
    const label = document.createElement("span");
    label.textContent = this.number;
    label.dataset.capturedLine = this.number;
    return label;
  }
}

/** Hide only the capture prefix; the immutable document and anchors never move. */
export function numberedCodeGutter(text: string) {
  const prefixes = capturedLinePrefixes(text);
  const numbers = new Map(prefixes.map(({ from, number }) => [from, number]));
  const widest = prefixes.reduce(
    (value, row) => (row.number.length > value.length ? row.number : value),
    "",
  );
  const hidden = Decoration.set(
    prefixes.map(({ from, to }) => Decoration.replace({}).range(from, to)),
  );
  return [
    EditorView.decorations.of(hidden),
    EditorView.atomicRanges.of(() => hidden),
    gutter({
      class: "chat-capture-gutter",
      initialSpacer: () => new CapturedLineNumber(widest),
      lineMarker(_view, line) {
        const number = numbers.get(line.from);
        return number === undefined ? null : new CapturedLineNumber(number);
      },
    }),
  ];
}

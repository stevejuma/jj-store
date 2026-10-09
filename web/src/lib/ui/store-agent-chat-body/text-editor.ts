import { EditorState, Prec, StateEffect, StateField } from "@codemirror/state";
import { Decoration, EditorView, keymap } from "@codemirror/view";
import {
  capturedCodeLanguage,
  fileCodeLanguage,
  numberedCodeHighlights,
} from "./captured-code.js";
import { inlineReferences } from "./inline-references.js";
import { readingDecorations } from "./markdown-decorations.js";
import { commentDecorations } from "./comment-widgets.js";
import { middleDecorations } from "./middle-fold.js";
import { patchLines } from "./patch-lines.js";
import { chatCaret } from "./chat-caret.js";
import { numberedCodeGutter } from "./numbered-code-gutter.js";
import type { ChatBodyBlock } from "./model.js";
import type { ChatBodyComments } from "./comments.svelte.js";

export const refreshComments = StateEffect.define<null>();
export function readingExtensions(
  block: ChatBodyBlock,
  comments: ChatBodyComments,
  contexts: Map<unknown, unknown>,
  fileView = false,
) {
  const references =
    block.kind === "markdown"
      ? inlineReferences(block.text, comments, contexts)
      : [];
  const reading =
    block.kind === "markdown"
      ? readingDecorations(block.text, references)
      : undefined;
  const referenceDecorations = Decoration.set(
    references.map((r) => r.decoration),
    true,
  );
  const open = (view: EditorView, initial = "") => {
    comments.open(block, view.state.selection.main, initial);
    return true;
  };
  const annotations = StateField.define({
    create: () => commentDecorations(block, comments, contexts),
    update: (value, tr) =>
      tr.effects.some((e) => e.is(refreshComments))
        ? commentDecorations(block, comments, contexts)
        : value,
    provide: (field) => EditorView.decorations.from(field),
  });
  let composing = false;
  const folds = StateField.define({
    create: () => middleDecorations(block, comments),
    update: (value, tr) =>
      tr.effects.some((e) => e.is(refreshComments))
        ? middleDecorations(block, comments)
        : value,
    provide: (field) => [
      EditorView.decorations.from(field),
      EditorView.atomicRanges.of((view) => view.state.field(field)),
    ],
  });
  return [
    chatCaret,
    // Input is routed into comments; the retained document rejects every edit.
    Prec.highest(EditorView.editable.of(true)),
    EditorState.transactionFilter.of((tr) => (tr.docChanged ? [] : tr)),
    EditorView.contentAttributes.of({
      "aria-readonly": "true",
      "data-chat-reader": block.id,
    }),
    block.kind === "markdown"
      ? [
          EditorView.decorations.of(reading!.decorations),
          EditorView.decorations.of(referenceDecorations),
          EditorView.atomicRanges.of(() => referenceDecorations),
          EditorView.atomicRanges.of(() => reading!.hidden),
        ]
      : block.lineNumbered
        ? EditorView.decorations.of(
            Decoration.set(
              numberedCodeHighlights(block.text, block.path).map(
                ({ from, to, classes }) =>
                  Decoration.mark({ class: classes }).range(from, to),
              ),
              true,
            ),
          )
        : fileView
          ? fileCodeLanguage(block.path)
          : capturedCodeLanguage(block.path),
    block.kind === "patch" ? patchDecorations(block.text) : [],
    block.lineNumbered ? numberedCodeGutter(block.text) : [],
    annotations,
    folds,
    Prec.highest(
      keymap.of([
        { key: "Ctrl-Alt-m", run: (view) => open(view) },
        { key: "Meta-Alt-m", run: (view) => open(view) },
      ]),
    ),
    EditorView.domEventHandlers({
      keydown(event, view) {
        const element = event.target instanceof Element ? event.target : null;
        if (
          element?.closest("[data-chat-comment-area], [data-chat-fold-control]")
        )
          return false;
        const link = element?.closest<HTMLElement>("[data-chat-link]");
        if (link && event.key === "Enter") {
          window.open(link.dataset.chatLink, "_blank", "noopener,noreferrer");
          return true;
        }
        if (
          !composing &&
          !event.isComposing &&
          !event.ctrlKey &&
          !event.metaKey &&
          event.key.length === 1 &&
          event.key !== "Dead" &&
          !link
        ) {
          event.preventDefault();
          return open(view, event.key);
        }
        return false;
      },
      paste(event, view) {
        if ((event.target as Element)?.closest?.("[data-chat-comment-area]"))
          return false;
        const text = event.clipboardData?.getData("text/plain");
        event.preventDefault();
        if (text) open(view, text);
        return true;
      },
      compositionstart() {
        composing = true;
        return false;
      },
      compositionend(event, view) {
        composing = false;
        if (event.data) open(view, event.data);
        return true;
      },
      click(event) {
        const link = (event.target as Element)?.closest?.<HTMLElement>(
          "[data-chat-link]",
        );
        if (link) {
          window.open(link.dataset.chatLink, "_blank", "noopener,noreferrer");
          return true;
        }
        return false;
      },
      drop(event) {
        event.preventDefault();
        return true;
      },
    }),
    EditorView.inputHandler.of((view, _from, _to, text) => {
      if (text && !composing) open(view, text);
      return true;
    }),
  ];
}
function patchDecorations(text: string) {
  const ranges = patchLines(text).map(({ from, kind }) =>
    Decoration.line({
      attributes: {
        class: `chat-patch-${kind}`,
        "data-patch-kind": kind,
      },
    }).range(from),
  );
  return EditorView.decorations.of(Decoration.set(ranges, true));
}

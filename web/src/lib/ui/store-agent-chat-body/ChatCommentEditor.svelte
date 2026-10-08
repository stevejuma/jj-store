<script lang="ts">
  import { tick } from "svelte";
  import { MiraCodeEditor, type MiraCodeEditorHandle } from "@lapismd/mira";
  import { Prec } from "@codemirror/state";
  import { keymap, EditorView } from "@codemirror/view";
  import { chatCaret } from "./chat-caret.js";
  import type { ChatBodyComments } from "./comments.svelte.js";
  import type { ChatCommentThread, ChatComment } from "./model.js";
  let {
    state,
    thread,
    comment,
    location,
  }: {
    state: ChatBodyComments;
    thread: ChatCommentThread;
    comment: ChatComment;
    location: "inline" | "queue";
  } = $props();
  let editor: MiraCodeEditorHandle;
  const extensions = [
    chatCaret,
    EditorView.updateListener.of((update) => {
      if (update.view.hasFocus && (update.selectionSet || update.docChanged)) {
        const { anchor, head } = update.state.selection.main;
        state.selections.set(`${comment.id}:${location}`, { anchor, head });
      }
    }),
    Prec.highest(
      keymap.of([
        {
          key: "Ctrl-Enter",
          run: (view) => {
            if (view.composing) return false;
            void state.send();
            return true;
          },
        },
        {
          key: "Mod-Enter",
          run: (view) => {
            if (view.composing) return false;
            void state.send();
            return true;
          },
        },
        {
          key: "Escape",
          run: () => {
            state.close();
            return true;
          },
        },
      ]),
    ),
    EditorView.domEventHandlers({
      focus: () => state.activate(thread, comment, location),
    }),
  ];
  $effect(() => {
    if (
      state.active?.commentId === comment.id &&
      state.active.location === location
    )
      void tick().then(() => {
        const view = editor?.getView();
        if (!view || view.hasFocus) return;
        const selection = state.selections.get(`${comment.id}:${location}`);
        view.dispatch({
          selection: selection
            ? {
                anchor: Math.min(selection.anchor, view.state.doc.length),
                head: Math.min(selection.head, view.state.doc.length),
              }
            : { anchor: view.state.doc.length },
        });
        view.focus();
      });
  });
</script>

<div
  class="chat-comment-editor"
  data-comment-editor={comment.id}
  data-location={location}
  data-editing={state.active?.commentId === comment.id &&
    state.active.location === location}
>
  <MiraCodeEditor
    bind:this={editor}
    value={comment.text}
    onChange={(value) => state.edit(comment.id, value)}
    readonly={comment.status !== "draft"}
    {extensions}
    ariaLabel={`Comment on ${thread.label}`}
    variant="document"
    surface="frameless"
    height="content"
    minHeight="1.5rem"
    lineNumbers={false}
    spellcheck
  />
</div>

<style>
  .chat-comment-editor {
    min-width: 0;
    --chat-caret-color: var(--primary);
    --mira-code-editor-padding: 0;
    --mira-code-editor-background: transparent;
    --mira-code-editor-font-family: var(--font-sans, system-ui);
    --mira-code-editor-font-size: 1rem;
    --file-margins-x: 0px;
    --file-margins-y: 0px;
    --file-line-width: 100%;
  }
</style>

<script lang="ts">
  import { getAllContexts, onMount, tick, untrack } from "svelte";
  import { MiraCodeEditor, type MiraCodeEditorHandle } from "@lapismd/mira";
  import { Button } from "@lapismd/design-core/shadcn/button";
  import MessageCircleIcon from "@lucide/svelte/icons/message-circle";
  import type { EditorView } from "@codemirror/view";
  import { readingExtensions, refreshComments } from "./text-editor.js";
  import type { ChatBodyBlock } from "./model.js";
  import type { ChatBodyComments } from "./comments.svelte.js";
  import { validAnchor } from "./anchors.js";
  import "./text-reader.css";
  let {
    block,
    state: comments,
    showHeader = true,
  }: {
    block: ChatBodyBlock;
    state: ChatBodyComments;
    showHeader?: boolean;
  } = $props();
  let editor: MiraCodeEditorHandle;
  let view = $state<EditorView>();
  const contexts = getAllContexts();
  const extensions = untrack(() =>
    readingExtensions(block, comments, contexts),
  );
  onMount(() => {
    let disposed = false;
    void tick().then(() => {
      if (disposed) return;
      view = editor.getView() ?? undefined;
      comments.readers.set(block.id, (anchor) => {
        if (validAnchor(block, anchor))
          view?.dispatch({
            selection: { anchor: anchor.from, head: anchor.to },
          });
        editor.focus();
      });
    });
    return () => {
      disposed = true;
      comments.readers.delete(block.id);
      view = undefined;
    };
  });
  $effect(() => {
    comments.active;
    comments.threads;
    view?.dispatch({ effects: refreshComments.of(null) });
  });
  function comment() {
    comments.open(block, view?.state.selection.main ?? { from: 0, to: 0 });
  }
</script>

<section
  class="chat-text-block"
  role="group"
  data-kind={block.kind}
  data-block-id={block.id}
  aria-label={`${block.label} content`}
>
  {#if showHeader && block.kind !== "markdown"}<header>
      {block.label}
    </header>{/if}
  <div class="chat-text-reader">
    <MiraCodeEditor
      bind:this={editor}
      value={block.text}
      {extensions}
      readonly
      lineNumbers={false}
      ariaLabel={block.label}
      variant={block.kind === "markdown" ? "document" : "code"}
      surface="frameless"
      height="content"
      minHeight="0"
    />
  </div>
  <div class="chat-text-actions">
    <Button
      variant="ghost"
      size="sm"
      aria-label={`Comment on ${block.label}`}
      onclick={comment}
    >
      <MessageCircleIcon size={14} /> Comment
    </Button>
  </div>
</section>

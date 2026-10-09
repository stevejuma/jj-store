<script lang="ts">
  import { getAllContexts, onMount, tick, untrack } from "svelte";
  import { MiraCodeEditor, type MiraCodeEditorHandle } from "@lapismd/mira";
  import { Compartment } from "@codemirror/state";
  import ChatHoverActions from "./ChatHoverActions.svelte";
  import type { EditorView } from "@codemirror/view";
  import { readingExtensions, refreshComments } from "./text-editor.js";
  import type { ChatBodyBlock } from "./model.js";
  import type { ChatBodyComments } from "./comments.svelte.js";
  import { validAnchor } from "./anchors.js";
  import { middleFoldKey } from "./middle-fold.js";
  import { replyIndicator } from "./reply-indicator.js";
  import "./text-reader.css";
  let {
    block,
    state: comments,
    showHeader = true,
    reactionTarget,
    reply = false,
    fileView = false,
  }: {
    block: ChatBodyBlock;
    state: ChatBodyComments;
    showHeader?: boolean;
    reactionTarget?: string;
    reply?: boolean;
    fileView?: boolean;
  } = $props();
  let editor: MiraCodeEditorHandle;
  let view = $state<EditorView>();
  const contexts = getAllContexts();
  const replyPrefix = new Compartment();
  const reader = new Compartment();
  let configured = "";
  const extensions = untrack(() => [
    reader.of(readingExtensions(block, comments, contexts, fileView)),
    replyPrefix.of(reply ? replyIndicator : []),
  ]);
  onMount(() => {
    let disposed = false;
    void tick().then(() => {
      if (disposed) return;
      view = editor.getView() ?? undefined;
      comments.readers.set(block.id, (anchor, scroll = false) => {
        if (validAnchor(block, anchor))
          view?.dispatch({
            selection: { anchor: anchor.from, head: anchor.to },
            scrollIntoView: scroll,
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
    const signature = JSON.stringify([
      block.id,
      block.revision,
      block.text,
      block.path,
      block.kind,
      block.lineNumbered,
      block.wholeSection,
      block.commentable,
      fileView,
    ]);
    if (view && signature !== configured) {
      configured = signature;
      const previous = view.state.doc.toString(),
        next = block.text;
      let from = 0,
        end = previous.length,
        nextEnd = next.length;
      while (from < end && from < nextEnd && previous[from] === next[from])
        from++;
      while (
        end > from &&
        nextEnd > from &&
        previous[end - 1] === next[nextEnd - 1]
      ) {
        end--;
        nextEnd--;
      }
      // Only host-owned immutable capture updates bypass the edit-rejection filter.
      // A minimal replacement preserves cursor positions and existing widgets.
      view.dispatch({
        changes:
          previous === next
            ? undefined
            : { from, to: end, insert: next.slice(from, nextEnd) },
        effects: reader.reconfigure(
          readingExtensions(
            untrack(() => block),
            comments,
            contexts,
            fileView,
          ),
        ),
        filter: false,
      });
    }
  });
  $effect(() => {
    view?.dispatch({
      effects: replyPrefix.reconfigure(reply ? replyIndicator : []),
    });
  });
  $effect(() => {
    comments.active;
    comments.visibleThreads;
    comments.highlightedThread;
    comments.disclosures[middleFoldKey(block)];
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
  data-file-view={fileView || undefined}
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
      lineNumbers={fileView}
      ariaLabel={block.label}
      variant={fileView ? "code" : "document"}
      surface="frameless"
      height="content"
      minHeight="0"
    />
  </div>
  <ChatHoverActions
    {block}
    state={comments}
    {reactionTarget}
    onComment={comment}
  />
</section>

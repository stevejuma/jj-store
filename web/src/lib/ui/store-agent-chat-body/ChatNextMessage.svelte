<script lang="ts">
  import type { Snippet } from "svelte";
  import { MiraCodeEditor, type MiraCodeEditorHandle } from "@lapismd/mira";
  import { keymap } from "@codemirror/view";
  import { Prec } from "@codemirror/state";
  import CommunityMemberAvatar from "../community-member-avatar/CommunityMemberAvatar.svelte";
  import { chatDraftCaret } from "./chat-caret.js";
  import type { CommunityMemberModel } from "../community-workspace/model.js";
  let {
    value = $bindable(""),
    member,
    onSubmit,
    disabled = false,
    hideAvatar = false,
    input,
  }: {
    value?: string;
    member: CommunityMemberModel;
    onSubmit: () => void;
    disabled?: boolean;
    hideAvatar?: boolean;
    input?: Snippet;
  } = $props();
  let editor = $state<MiraCodeEditorHandle>();
  const extensions = [
    chatDraftCaret,
    Prec.highest(
      keymap.of([
        ...["Ctrl-Enter", "Mod-Enter"].map((key) => ({
          key,
          run: (view: import("@codemirror/view").EditorView) => {
            if (view.composing || disabled) return false;
            onSubmit();
            return true;
          },
        })),
      ]),
    ),
  ];
  export function focus() {
    editor?.focus();
    editor?.getView()?.dom.scrollIntoView({ block: "nearest" });
  }
</script>

<div
  class="chat-next-message"
  role="group"
  aria-label="Next message draft"
  data-chat-next-message
>
  <div class="chat-next-avatar">
    {#if !hideAvatar}<CommunityMemberAvatar
        {member}
        size="xsmall"
        decorative
      />{/if}
  </div>
  <div class="chat-next-editor">
    {#if input}{@render input()}{:else}<MiraCodeEditor
        bind:this={editor}
        bind:value
        readonly={disabled}
        {extensions}
        ariaLabel="Next message"
        variant="document"
        surface="frameless"
        height="content"
        minHeight="max(16rem, 45vh)"
        lineNumbers={false}
        spellcheck
      />{/if}
  </div>
</div>

<style>
  .chat-next-message {
    display: grid;
    grid-template-columns: 1.5rem minmax(0, 1fr);
    gap: 0.5rem;
  }
  .chat-next-avatar {
    padding-top: max(
      0px,
      calc(
        0.25rem +
          (
            var(--conversation-prose-size, 0.875rem) *
              var(--conversation-prose-leading, 1.65) - 1.5rem
          ) /
          2
      )
    );
  }
  .chat-next-editor {
    min-width: 0;
    --chat-caret-color: var(--primary);
    --mira-font-sans: var(
      --conversation-prose-font,
      var(--font-sans, system-ui)
    );
    --mira-font-size: var(--conversation-prose-size, 0.875rem);
    --mira-line-height: var(--conversation-prose-leading, 1.65);
    --file-margins-x: 0px;
    --file-margins-y: 0px;
    --file-line-width: 100%;
    --mira-code-editor-padding: 0;
    --mira-code-editor-background: transparent;
    --mira-code-editor-font-family: var(--font-sans, system-ui);
    --mira-code-editor-font-size: var(--conversation-prose-size, 0.875rem);
  }
</style>

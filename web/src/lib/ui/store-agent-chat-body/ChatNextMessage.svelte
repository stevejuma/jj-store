<script lang="ts">
  import { MiraCodeEditor, type MiraCodeEditorHandle } from "@lapismd/mira";
  import { keymap } from "@codemirror/view";
  import { Prec } from "@codemirror/state";
  import CommunityMemberAvatar from "../community-member-avatar/CommunityMemberAvatar.svelte";
  import { chatCaret } from "./chat-caret.js";
  import type { CommunityMemberModel } from "../community-workspace/model.js";
  let {
    value = $bindable(""),
    member,
    onSubmit,
  }: {
    value?: string;
    member: CommunityMemberModel;
    onSubmit: () => void;
  } = $props();
  let editor: MiraCodeEditorHandle;
  const extensions = [
    chatCaret,
    Prec.highest(
      keymap.of([
        {
          key: "Mod-Enter",
          run: () => {
            onSubmit();
            return true;
          },
        },
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
    <CommunityMemberAvatar {member} size="xsmall" decorative />
  </div>
  <div class="chat-next-editor">
    <MiraCodeEditor
      bind:this={editor}
      bind:value
      {extensions}
      ariaLabel="Next message"
      variant="document"
      surface="frameless"
      height="content"
      minHeight="max(16rem, 45vh)"
      lineNumbers={false}
      spellcheck
    />
  </div>
</div>

<style>
  .chat-next-message {
    display: grid;
    grid-template-columns: 1.5rem minmax(0, 1fr);
    gap: 0.5rem;
  }
  .chat-next-avatar {
    padding-top: 0.125rem;
  }
  .chat-next-editor {
    min-width: 0;
    --chat-caret-color: var(--primary);
    --mira-font-size: 1rem;
    --mira-line-height: 1.75;
    --file-margins-x: 0px;
    --file-margins-y: 0px;
    --file-line-width: 100%;
    --mira-code-editor-padding: 0;
    --mira-code-editor-background: transparent;
    --mira-code-editor-font-family: var(--font-sans, system-ui);
    --mira-code-editor-font-size: 1rem;
  }
</style>

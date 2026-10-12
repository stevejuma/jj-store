import { untrack } from "svelte";
import type { EditorView } from "@codemirror/view";
import { refreshCommunityComposerTokenDecorations } from "./composer-extension.js";

export function watchComposerTokenDecorations(
  state: () => {
    members: unknown;
    rooms: unknown;
    communityEmojis: unknown;
    blocked: boolean;
    view: EditorView | undefined;
  },
) {
  $effect(() => {
    const { view } = state();
    untrack(() => {
      if (view) refreshCommunityComposerTokenDecorations(view);
    });
  });
}

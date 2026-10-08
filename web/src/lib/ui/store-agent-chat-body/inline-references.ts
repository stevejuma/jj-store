import { mount, unmount, tick } from "svelte";
import { Decoration, WidgetType, type EditorView } from "@codemirror/view";
import { communityComposerEntityMatches } from "../community-chat-composer/composer-entity-links.js";
import { contentMentionMatches } from "../../community/content-mentions.js";
import {
  isFencedCodePosition,
  isInlineCodePosition,
} from "../community-chat-composer/model.js";
import ChatInlineReference from "./ChatInlineReference.svelte";
import type { ChatBodyComments } from "./comments.svelte.js";
const cleanups = new WeakMap<HTMLElement, () => void>();
class ReferenceWidget extends WidgetType {
  constructor(
    readonly state: ChatBodyComments,
    readonly contexts: Map<unknown, unknown>,
    readonly label: string,
    readonly href?: string,
    readonly memberId?: string,
  ) {
    super();
  }
  eq(other: ReferenceWidget) {
    return (
      this.state === other.state &&
      this.label === other.label &&
      this.href === other.href &&
      this.memberId === other.memberId
    );
  }
  toDOM(view: EditorView) {
    const host = document.createElement("span");
    host.contentEditable = "false";
    const instance = mount(ChatInlineReference, {
      target: host,
      context: this.contexts,
      props: {
        state: this.state,
        label: this.label,
        href: this.href,
        memberId: this.memberId,
      },
    });
    cleanups.set(host, () => {
      void unmount(instance);
    });
    void tick().then(() => {
      if (host.isConnected) view.requestMeasure();
    });
    return host;
  }
  destroy(host: HTMLElement) {
    cleanups.get(host)?.();
    cleanups.delete(host);
  }
  ignoreEvent() {
    return true;
  }
}
/** Replace only presentation: the editor document and selection offsets stay captured. */
export function inlineReferences(
  text: string,
  state: ChatBodyComments,
  contexts: Map<unknown, unknown>,
) {
  const ranges = [
    ...communityComposerEntityMatches(text).map((m) => ({
      from: m.from,
      to: m.to,
      label: m.link.label ?? m.href,
      href: m.href,
      memberId: undefined,
    })),
    ...contentMentionMatches(text).map((m) => ({
      from: m.start,
      to: m.end,
      label: m.value,
      memberId: m.pubkey,
      href: undefined,
    })),
  ].sort((a, b) => a.from - b.from);
  let end = -1;
  return ranges
    .filter((r) => {
      if (
        r.from < end ||
        isFencedCodePosition(text, r.from) ||
        isInlineCodePosition(text, r.from)
      )
        return false;
      end = r.to;
      return true;
    })
    .map((r) => ({
      ...r,
      decoration: Decoration.replace({
        widget: new ReferenceWidget(
          state,
          contexts,
          r.label,
          r.href,
          r.memberId,
        ),
      }).range(r.from, r.to),
    }));
}

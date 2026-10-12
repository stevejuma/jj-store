import { Decoration, WidgetType } from "@codemirror/view";
import { communityEmojiMatches } from "../community-chat-composer/composer-widgets.js";
import {
  isFencedCodePosition,
  isInlineCodePosition,
} from "../community-chat-composer/model.js";
import type { CommunityEmojiReference } from "../../community/workspace-model.js";
class EmojiWidget extends WidgetType {
  constructor(
    readonly name: string,
    readonly url: string,
  ) {
    super();
  }
  eq(other: EmojiWidget) {
    return other.name === this.name && other.url === this.url;
  }
  toDOM() {
    const image = document.createElement("img");
    image.src = this.url;
    image.alt = `:${this.name}:`;
    image.title = image.alt;
    image.style.cssText =
      "width:1.25em;height:1.25em;object-fit:contain;vertical-align:text-bottom";
    return image;
  }
}
export function emojiDecorations(
  text: string,
  values: readonly CommunityEmojiReference[] = [],
) {
  return communityEmojiMatches(text, values)
    .filter(
      (m) =>
        !isFencedCodePosition(text, m.from) &&
        !isInlineCodePosition(text, m.from),
    )
    .map((m) =>
      Decoration.replace({
        widget: new EmojiWidget(m.emoji.shortcode, m.emoji.url),
      }).range(m.from, m.to),
    );
}

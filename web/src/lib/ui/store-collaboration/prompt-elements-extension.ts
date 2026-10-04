import { defineMiraExtension } from "@lapismd/mira/extensions";
import StorePromptElement from "./StorePromptElement.svelte";
import {
  PROMPT_ELEMENT_MARKER,
  remarkPromptElements,
} from "./remark-prompt-elements.js";
type Node = {
  tagName?: string;
  properties?: Record<string, unknown>;
  children?: Node[];
};

// The marker uses safe div/title through Mira sanitization. Only afterwards
// does the application select its compiled component; source attributes stay text.
function rehypePromptElements() {
  return (tree: Node) => {
    function visit(node: Node) {
      const title = node.properties?.title;
      if (
        node.tagName === "div" &&
        typeof title === "string" &&
        title.startsWith(PROMPT_ELEMENT_MARKER)
      ) {
        node.tagName = "storepromptelement";
        node.properties = { label: title.slice(PROMPT_ELEMENT_MARKER.length) };
      }
      node.children?.forEach(visit);
    }
    visit(tree);
  };
}

export const promptElementsExtension = defineMiraExtension({
  name: "agent-prompt-elements",
  remarkPlugins: [remarkPromptElements],
  rehypePlugins: [rehypePromptElements],
  components: { storepromptelement: StorePromptElement },
});

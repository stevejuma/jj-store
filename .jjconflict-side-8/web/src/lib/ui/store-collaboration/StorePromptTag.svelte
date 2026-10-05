<script lang="ts">
  import { promptTagTokens } from "./prompt-tag-tokens.js";
  let { label }: { label: string } = $props();
  const tokens = $derived(promptTagTokens(label));
</script>

<span class="store-prompt-tag"
  >{#each tokens as token}<span data-xml-token={token.kind}>{token.text}</span
    >{/each}</span
>

<style>
  .store-prompt-tag {
    font-family: var(--font-mono);
    font-size: inherit;
    white-space: pre-wrap;
    overflow-wrap: anywhere;
  }
  [data-xml-token="name"] {
    color: color-mix(
      in srgb,
      var(--ui-code-syntax-tag, var(--foreground)) 80%,
      var(--foreground)
    );
  }
  [data-xml-token="attribute"] {
    /* Named identifiers stay distinct even in palettes with neutral attributes. */
    color: color-mix(
      in srgb,
      var(--ui-code-syntax-constant, var(--foreground)) 85%,
      var(--foreground)
    );
  }
  [data-xml-token="string"] {
    color: var(--ui-code-syntax-string, var(--foreground));
  }
  [data-xml-token="punctuation"] {
    color: var(--ui-code-syntax-punctuation, var(--foreground));
  }
</style>

<script lang="ts">
  type MarkdownBlock =
    | { kind: "heading"; level: 1 | 2 | 3; text: string }
    | { kind: "paragraph"; text: string }
    | { kind: "list"; items: string[] }
    | { kind: "code"; language: string; text: string };

  let {
    content,
    label = "Markdown preview",
  }: { content: string; label?: string } = $props();

  const blocks = $derived(parseMarkdown(content));

  function parseMarkdown(markdown: string): MarkdownBlock[] {
    const lines = markdown
      .replace(/\r\n/g, "\n")
      .replace(/\r/g, "\n")
      .split("\n");
    const parsed: MarkdownBlock[] = [];
    let paragraph: string[] = [];
    let listItems: string[] = [];
    let codeLines: string[] | null = null;
    let codeLanguage = "";

    function flushParagraph() {
      if (paragraph.length === 0) return;
      parsed.push({ kind: "paragraph", text: paragraph.join(" ").trim() });
      paragraph = [];
    }

    function flushList() {
      if (listItems.length === 0) return;
      parsed.push({ kind: "list", items: listItems });
      listItems = [];
    }

    for (const line of lines) {
      const fence = line.match(/^```\s*([^`]*)\s*$/u);
      if (fence) {
        if (codeLines) {
          parsed.push({
            kind: "code",
            language: codeLanguage,
            text: codeLines.join("\n"),
          });
          codeLines = null;
          codeLanguage = "";
        } else {
          flushParagraph();
          flushList();
          codeLines = [];
          codeLanguage = fence[1]?.trim() ?? "";
        }
        continue;
      }

      if (codeLines) {
        codeLines.push(line);
        continue;
      }

      const heading = line.match(/^(#{1,3})\s+(.+)$/u);
      if (heading) {
        flushParagraph();
        flushList();
        parsed.push({
          kind: "heading",
          level: heading[1]!.length as 1 | 2 | 3,
          text: heading[2]!.trim(),
        });
        continue;
      }

      const list = line.match(/^\s*[-*+]\s+(.+)$/u);
      if (list) {
        flushParagraph();
        listItems.push(list[1]!.trim());
        continue;
      }

      if (line.trim() === "") {
        flushParagraph();
        flushList();
        continue;
      }

      flushList();
      paragraph.push(line.trim());
    }

    if (codeLines) {
      parsed.push({
        kind: "code",
        language: codeLanguage,
        text: codeLines.join("\n"),
      });
    }
    flushParagraph();
    flushList();
    return parsed;
  }
</script>

<section
  class="community-projects__markdown-preview"
  data-ui-component="community-markdown-preview"
  aria-label={label}
>
  {#each blocks as block, index (`${block.kind}-${index}`)}
    {#if block.kind === "heading"}
      {#if block.level === 1}
        <h1>{block.text}</h1>
      {:else if block.level === 2}
        <h2>{block.text}</h2>
      {:else}
        <h3>{block.text}</h3>
      {/if}
    {:else if block.kind === "list"}
      <ul>
        {#each block.items as item, itemIndex (`${item}-${itemIndex}`)}
          <li>{item}</li>
        {/each}
      </ul>
    {:else if block.kind === "code"}
      <pre data-language={block.language || undefined}><code>{block.text}</code
        ></pre>
    {:else}
      <p>{block.text}</p>
    {/if}
  {:else}
    <p class="community-projects__status">This file is empty.</p>
  {/each}
</section>

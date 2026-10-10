<script module lang="ts">
  import { defineMeta } from "@storybook/addon-svelte-csf";
  import { expect, userEvent, waitFor, within } from "storybook/test";
  import Prototype from "../../fixtures/ChatBodyFixture.svelte";
  import Agent from "../../fixtures/WorkspaceFilesFixture.svelte";
  import Thread from "../../fixtures/StoreThreadsFixture.svelte";
  const { Story } = defineMeta({
    title: "JJ Store/Document caret parity",
    tags: ["visual-pending"],
    parameters: {
      layout: "fullscreen",
      jjStoreShell: true,
      a11y: { test: "error" },
    },
  });
  async function caret(editor: HTMLElement, draft = false) {
    await waitFor(() => {
      const cursor = editor.closest(".cm-editor")!.querySelector(".cm-cursor");
      expect(cursor).not.toBeNull();
      expect(getComputedStyle(cursor!).borderInlineStartWidth).toBe("2px");
      expect(getComputedStyle(cursor!).transform).toBe(
        "matrix(1, 0, 0, 1.2, 0, 0)",
      );
      const bounds = cursor!.getBoundingClientRect();
      for (let host = cursor!.parentElement; host; host = host.parentElement) {
        const style = getComputedStyle(host);
        const clip = host.getBoundingClientRect();
        if (/hidden|clip|auto|scroll/.test(style.overflowX)) {
          expect(
            bounds.left,
            `${host.className} clips the caret start`,
          ).toBeGreaterThanOrEqual(clip.left + (draft ? 2 : 0) - 0.25);
          expect(
            bounds.right,
            `${host.className} clips the caret end`,
          ).toBeLessThanOrEqual(clip.right + 0.25);
        }
        if (/hidden|clip|auto|scroll/.test(style.overflowY)) {
          expect(
            bounds.top,
            `${host.className} clips the caret top`,
          ).toBeGreaterThanOrEqual(clip.top + (draft ? 4 : 0) - 0.25);
          expect(
            bounds.bottom,
            `${host.className} clips the caret bottom`,
          ).toBeLessThanOrEqual(clip.bottom + 0.25);
        }
      }
    });
    return getComputedStyle(
      editor.closest(".cm-editor")!.querySelector(".cm-cursor")!,
    ).borderInlineStartColor;
  }
  async function parity({ canvasElement }: { canvasElement: HTMLElement }) {
    const c = within(canvasElement);
    // Wait for the conversation's editors, rather than an unrelated fixture input.
    const draft = await c.findByRole("group", { name: "Next message draft" });
    const input = await within(draft).findByRole("textbox", {
      name: /^(Next message|Markdown editor)$/,
    });
    const reader = await waitFor(() => {
      const content = canvasElement.querySelector<HTMLElement>(
        '.chat-text-block[data-kind="markdown"] .cm-content',
      );
      expect(content).not.toBeNull();
      return content!;
    });
    reader!.scrollIntoView({ block: "center" });
    await userEvent.click(reader!);
    await userEvent.keyboard("{Home}");
    const readingColor = await caret(reader!);
    input.scrollIntoView({ block: "center" });
    await userEvent.click(input);
    const draftColor = await caret(input, true);
    const avatar = input
      .closest(".chat-next-message")!
      .querySelector(".community-member-avatar")!
      .getBoundingClientRect();
    const cursor = input
      .closest(".cm-editor")!
      .querySelector(".cm-cursor")!
      .getBoundingClientRect();
    // Glyph metrics differ between the plain and rich editors; align centers.
    expect(
      Math.abs((cursor.top + cursor.bottom - avatar.top - avatar.bottom) / 2),
    ).toBeLessThan(1);
    expect(cursor.height).toBeGreaterThan(avatar.height * 0.85);
    expect(draftColor).not.toBe(readingColor);
    expect(getComputedStyle(input).fontSize).toBe("16px");
    expect(getComputedStyle(input).lineHeight).toBe("28px");
    await userEvent.keyboard("Caret sample");
    expect(input).toHaveTextContent("Caret sample");
    await userEvent.keyboard("{Home}");
    await caret(input, true);
    await userEvent.keyboard("{End}{Enter}Second line");
    expect(input).toHaveTextContent("Second line");
    await userEvent.keyboard("{Home}");
    await caret(input, true);
    await userEvent.keyboard("{End}");
    await caret(input, true);
  }
</script>

<Story
  asChild
  name="Prototype uses distinct reading and draft carets"
  play={parity}
>
  {#snippet children()}<Prototype />{/snippet}
</Story>
<Story asChild name="Agent document matches the prototype carets" play={parity}>
  {#snippet children()}<Agent />{/snippet}
</Story>
<Story
  asChild
  name="Thread document matches the prototype carets"
  play={parity}
>
  {#snippet children()}<Thread selected />{/snippet}
</Story>

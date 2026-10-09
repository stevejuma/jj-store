<script module lang="ts">
  import { EditorView } from "@codemirror/view";
  import { defineMeta } from "@storybook/addon-svelte-csf";
  import {
    expect,
    userEvent,
    waitFor,
    within,
    fireEvent,
  } from "storybook/test";
  import Fixture from "../../fixtures/ConversationContinuityFixture.svelte";
  const { Story } = defineMeta({
    title: "JJ Store/Conversation continuity",
    component: Fixture,
    tags: ["visual-pending"],
    parameters: { layout: "fullscreen" },
  });
</script>

<Story
  name="Document prompt is inline and preserves selection through composer view switches"
  play={async ({ canvasElement }) => {
    const c = within(canvasElement);
    const input = () => c.getByRole("textbox", { name: "Markdown editor" });
    const next = await c.findByRole("group", { name: "Next message draft" });
    await expect(next).toContainElement(input());
    await expect(
      next.querySelector(".cm-content")!.getBoundingClientRect().height,
    ).toBeGreaterThanOrEqual(256);
    await expect(
      c
        .getByRole("toolbar", { name: "Message formatting and actions" })
        .closest(".chat-body-footer"),
    ).not.toBeNull();
    await expect(
      canvasElement.querySelector('[data-ui-component="ai-chat-composer"]'),
    ).toBeNull();
    await expect(
      next.querySelector('[data-ui-component="scroll-area"]'),
    ).toBeNull();
    await userEvent.click(input());
    await userEvent.type(input(), "first{Enter}second 🦀{Enter}last");
    const editor = EditorView.findFromDOM(input())!;
    await expect(editor.state.doc.toString()).toBe("first\nsecond 🦀\nlast");
    await expect(c.getByRole("button", { name: "Send message" })).toBeEnabled();
    await userEvent.keyboard("{Home}{Shift>}{ArrowRight}{ArrowRight}{/Shift}");
    const selected = {
      anchor: editor.state.selection.main.anchor,
      head: editor.state.selection.main.head,
    };
    await fireEvent.compositionStart(input(), { data: "" });
    await userEvent.click(c.getByRole("button", { name: "Switch renderer" }));
    await expect(next).toBeInTheDocument();
    await fireEvent.compositionEnd(input(), { data: "" });
    await waitFor(() =>
      expect(c.queryByRole("group", { name: "Next message draft" })).toBeNull(),
    );
    await expect(
      canvasElement.querySelector('[data-ui-component="ai-chat-composer"]'),
    ).not.toBeNull();
    await expect(
      EditorView.findFromDOM(input())!.state.selection.main.anchor,
    ).toBe(selected.anchor);
    await expect(
      EditorView.findFromDOM(input())!.state.selection.main.head,
    ).toBe(selected.head);
    await userEvent.click(c.getByRole("button", { name: "Switch renderer" }));
    await c.findByRole("group", { name: "Next message draft" });
    await expect(EditorView.findFromDOM(input())!.state.doc.toString()).toBe(
      "first\nsecond 🦀\nlast",
    );
    await expect(
      EditorView.findFromDOM(input())!.state.selection.main.anchor,
    ).toBe(selected.anchor);
    await expect(
      EditorView.findFromDOM(input())!.state.selection.main.head,
    ).toBe(selected.head);
    await userEvent.click(c.getByRole("button", { name: "Remount body" }));
    await c.findByRole("group", { name: "Next message draft" });
    await expect(input()).toHaveTextContent("second 🦀");
    await userEvent.click(input());
    await userEvent.keyboard("{Control>}{Enter}{/Control}");
    await waitFor(() => expect(input().textContent).toBe(""));
  }}
/>

<Story
  name="Streaming updates preserve readers and exact completion"
  play={async ({ canvasElement }) => {
    const c = within(canvasElement);
    const editor = await c.findByRole("textbox", { name: "Stream" });
    await expect(editor).toHaveTextContent("First");
    await userEvent.click(c.getByRole("button", { name: "Stream chunk" }));
    await waitFor(() => expect(editor).toHaveTextContent("First next 🦀"));
    await expect(c.getByRole("textbox", { name: "Stream" })).toBe(editor);
    await expect(
      c.getByRole("button", { name: "Comment on Stream" }),
    ).toBeDisabled();
    await userEvent.click(c.getByRole("button", { name: "Commit capture" }));
    await waitFor(() =>
      expect(
        c.getByRole("button", { name: "Comment on Stream" }),
      ).not.toBeDisabled(),
    );
    await expect(c.getByRole("textbox", { name: "Stream" })).toBe(editor);
  }}
/>
<Story
  name="Shared drafts survive composition view switches and remounts"
  play={async ({ canvasElement }) => {
    const c = within(canvasElement);
    const reader = await c.findByRole("textbox", { name: "Source" });
    await userEvent.click(reader);
    await userEvent.keyboard("{Home}{Shift>}{End}{/Shift}x");
    const inline = () =>
      canvasElement.querySelector<HTMLElement>(
        '[data-location="inline"] [role="textbox"]',
      )!;
    await waitFor(() => expect(inline()).toHaveFocus());
    const editor = inline();
    await userEvent.keyboard(" pending 🦀");
    await expect(inline()).toBe(editor);
    const queued = canvasElement.querySelector<HTMLElement>(
      '[data-location="queue"] [role="textbox"]',
    )!;
    await expect(queued).toHaveTextContent("x pending 🦀");
    await fireEvent.compositionStart(editor, { data: "" });
    await userEvent.click(c.getByRole("button", { name: "Switch renderer" }));
    await expect(
      canvasElement.querySelector("[data-conversation-view]"),
    ).toHaveAttribute("data-conversation-view", "document");
    await fireEvent.compositionEnd(editor, { data: "" });
    await waitFor(() =>
      expect(
        canvasElement.querySelector("[data-conversation-view]"),
      ).toHaveAttribute("data-conversation-view", "classic"),
    );
    await expect(inline()).toHaveTextContent("x pending 🦀");
    await userEvent.click(c.getByRole("button", { name: "Remount body" }));
    await waitFor(() => expect(inline()).toHaveTextContent("x pending 🦀"));
    await userEvent.click(c.getByRole("button", { name: "Switch renderer" }));
    await waitFor(() => expect(inline()).toHaveTextContent("x pending 🦀"));
    const message = c.getByRole("textbox", { name: "Markdown editor" });
    await userEvent.click(message);
    await userEvent.type(message, "Mixed{Enter}prompt");
    await expect(message).toHaveTextContent("Mixedprompt");
    await expect(
      canvasElement.querySelector('[data-location="queue"]'),
    ).not.toBeNull();
    await userEvent.keyboard("{Control>}{Enter}{/Control}");
    await waitFor(() =>
      expect(canvasElement.querySelector("[data-comment-editor]")).toBeNull(),
    );
    await waitFor(() => expect(message.textContent).toBe(""));
  }}
/>

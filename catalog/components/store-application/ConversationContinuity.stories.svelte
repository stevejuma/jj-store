<script module lang="ts">
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

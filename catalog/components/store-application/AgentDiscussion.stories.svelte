<script module lang="ts">
  import { defineMeta } from "@storybook/addon-svelte-csf";
  import { expect, userEvent, within, waitFor } from "storybook/test";
  import Fixture from "../../fixtures/AgentInlineCommentsFixture.svelte";
  import {
    openInlineConversation,
    startInlineComment,
    commentEditor,
    inlineEditors,
    promptCommands,
    sendInlineComments,
  } from "../../fixtures/agent-inline-comment-play.js";
  const { Story } = defineMeta({
    title: "JJ Store/Agent message discussion",
    component: Fixture,
    tags: ["visual-pending"],
    parameters: { layout: "fullscreen" },
  });
</script>

<Story
  name="Drafts at different message anchors remain independent and submit together"
  play={async ({ canvasElement }) => {
    const c = await openInlineConversation(canvasElement);
    const assistant = await startInlineComment(canvasElement);
    await userEvent.type(assistant, "Comment on the answer");
    await userEvent.click(c.getByRole("textbox", { name: "Your message" }));
    await userEvent.click(
      c.getByRole("button", { name: "Comment on Your message" }),
    );
    const user = await commentEditor(canvasElement, "Your message");
    await userEvent.type(user, "Comment on the prompt");
    await startInlineComment(canvasElement);
    await expect(inlineEditors(canvasElement)[0]).toHaveTextContent(
      "Comment on the answer",
    );
    await expect(
      inlineEditors(canvasElement, "Your message")[0],
    ).toHaveTextContent("Comment on the prompt");
    expect(
      canvasElement.querySelectorAll(
        '[data-location="queue"] [role="textbox"]',
      ),
    ).toHaveLength(2);
    expect(promptCommands(canvasElement)).toHaveLength(0);
    await sendInlineComments(canvasElement);
    const prompt = promptCommands(canvasElement)[0];
    if (prompt.action.action !== "prompt") throw new Error("Expected prompt");
    expect(prompt.action.text).toBe("");
    expect(prompt.action.comments!.comments).toHaveLength(2);
    expect(prompt.action.comments!.comments[0]).toMatchObject({
      body: "Comment on the answer",
      source: { target: { content: { id: "22".repeat(32) } } },
    });
    expect(prompt.action.comments!.comments[1]).toMatchObject({
      body: "Comment on the prompt",
      source: { target: { content: { id: "11".repeat(32) } } },
    });
    expect(promptCommands(canvasElement)).toHaveLength(1);
  }}
/>
<Story
  name="Immutable message reactions use the shared toolbar and reaction bar"
  play={async ({ canvasElement }) => {
    const c = await openInlineConversation(canvasElement),
      page = within(document.body);
    const source = c.getAllByRole("textbox", { name: "Assistant message" })[0];
    await userEvent.click(source);
    await userEvent.click(
      c.getAllByRole("button", {
        name: "Add reaction to Assistant message",
      })[0],
    );
    await userEvent.click(await page.findByRole("button", { name: ":lapis:" }));
    const reaction = await c.findByRole("button", {
      name: /1 reaction with Lapis/,
    });
    await userEvent.click(reaction);
    await waitFor(() =>
      expect(
        c.queryByRole("button", { name: /1 reaction with Lapis/ }),
      ).toBeNull(),
    );
    expect(promptCommands(canvasElement)).toHaveLength(0);
  }}
/>
<Story
  name="Inline range comments synchronize the queue survive remount and send exact sources"
  play={async ({ canvasElement }) => {
    const c = await openInlineConversation(canvasElement);
    const source = c.getAllByRole("textbox", { name: "Assistant message" })[0];
    await userEvent.click(source);
    await userEvent.keyboard("{Home}{Shift>}{End}{/Shift}x");
    const editor = await commentEditor(canvasElement);
    await userEvent.keyboard(" Use the shared form component.");
    const text = "x Use the shared form component.";
    await expect(editor).toHaveTextContent(text);
    const queued = canvasElement.querySelector<HTMLElement>(
      '[data-location="queue"] [role="textbox"]',
    )!;
    await expect(queued).toHaveTextContent(text);
    expect(promptCommands(canvasElement)).toHaveLength(0);
    await userEvent.click(c.getByRole("button", { name: "Switch renderer" }));
    await waitFor(() =>
      expect(
        canvasElement.querySelector("[data-conversation-view]"),
      ).toHaveAttribute("data-conversation-view", "classic"),
    );
    await expect(inlineEditors(canvasElement)[0]).toHaveTextContent(text);
    await userEvent.click(
      c.getByRole("button", { name: "Leave fixture conversation" }),
    );
    await openInlineConversation(canvasElement);
    await expect(inlineEditors(canvasElement)[0]).toHaveTextContent(text);
    await userEvent.click(c.getByRole("button", { name: "Switch renderer" }));
    await waitFor(() =>
      expect(
        canvasElement.querySelector("[data-conversation-view]"),
      ).toHaveAttribute("data-conversation-view", "document"),
    );
    await sendInlineComments(canvasElement);
    const prompt = promptCommands(canvasElement)[0];
    if (prompt.action.action !== "prompt") throw new Error("Expected prompt");
    expect(prompt.action.comments?.comments[0]).toMatchObject({
      body: text,
      source: {
        quote: "Ready to help.",
        selection: { encoding: "utf8", start_byte: "0", end_byte: "14" },
        target: { content: { id: "22".repeat(32) } },
      },
    });
    expect(promptCommands(canvasElement)).toHaveLength(1);
  }}
/>
<Story
  name="Unsupported repositories explain unavailable comments without dispatch"
  args={{ writeUnavailable: true }}
  play={async ({ canvasElement }) => {
    const c = await openInlineConversation(canvasElement);
    const source = c.getAllByRole("textbox", { name: "Assistant message" })[0];
    await userEvent.click(source);
    const action = c.getAllByRole("button", {
      name: "Comment on Assistant message",
    })[0];
    await waitFor(() => expect(action).toBeDisabled());
    await expect(action).toHaveAttribute(
      "title",
      "Comments are unavailable until collaboration replication is qualified.",
    );
    await expect(source).toHaveTextContent("Ready to help.");
    expect(inlineEditors(canvasElement)).toHaveLength(0);
    expect(promptCommands(canvasElement)).toHaveLength(0);
  }}
/>
<Story
  name="Unpublished assistant content stays readable until exact commenting is available"
  args={{ unpublished: true }}
  play={async ({ canvasElement }) => {
    const c = await openInlineConversation(canvasElement);
    const source = c.getAllByRole("textbox", { name: "Assistant message" })[0];
    await userEvent.click(source);
    const action = c.getAllByRole("button", {
      name: "Comment on Assistant message",
    })[0];
    await expect(action).toBeDisabled();
    await expect(action).toHaveAttribute(
      "title",
      "Waiting for an exact published source",
    );
    await expect(source).toHaveTextContent("Ready to help.");
    expect(inlineEditors(canvasElement)).toHaveLength(0);
    expect(promptCommands(canvasElement)).toHaveLength(0);
  }}
/>
<Story
  name="Unknown comment batches retain the exact request for receipt recovery"
  args={{ unknown: true }}
  play={async ({ canvasElement }) => {
    const c = await openInlineConversation(canvasElement);
    const editor = await startInlineComment(canvasElement);
    await userEvent.type(editor, "Publish exactly once");
    await userEvent.click(c.getByRole("button", { name: "Send message" }));
    const check = await c.findByRole("button", {
      name: "Check retained request",
    });
    await expect(editor).toHaveTextContent("Publish exactly once");
    await expect(editor).toHaveAttribute("contenteditable", "false");
    await expect(
      c.getByRole("button", { name: "Send message" }),
    ).toBeDisabled();
    const original = promptCommands(canvasElement)[0];
    await userEvent.click(check);
    await expect(
      await c.findByRole("button", { name: "Check retained request" }),
    ).toBeEnabled();
    expect(promptCommands(canvasElement)).toHaveLength(1);
    await userEvent.click(
      c.getByRole("button", { name: "Confirm fixture request" }),
    );
    await userEvent.click(
      c.getByRole("button", { name: "Check retained request" }),
    );
    await waitFor(() =>
      expect(
        c.queryByRole("button", { name: "Check retained request" }),
      ).toBeNull(),
    );
    await userEvent.click(
      c.getByRole("button", { name: "Refresh captured feedback" }),
    );
    await c.findByRole("button", { name: "Edit comment" });
    expect(promptCommands(canvasElement)).toEqual([original]);
    const checks = JSON.parse(
      canvasElement.querySelector('[data-testid="comment-checks"]')!
        .textContent!,
    );
    expect(checks.length).toBeGreaterThanOrEqual(3);
    expect(checks.every((id: string) => id === original.request_id)).toBe(true);
    await expect(
      canvasElement.querySelector('[data-location="queue"]'),
    ).toBeNull();
  }}
/>

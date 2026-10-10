<script module lang="ts">
  import { defineMeta } from "@storybook/addon-svelte-csf";
  import { expect, userEvent, within, waitFor } from "storybook/test";
  import Fixture from "../../fixtures/AgentInlineCommentsFixture.svelte";
  import {
    openInlineConversation,
    startInlineComment,
    commentEditor,
    inlineEditors,
    commentCommands,
    promptCommands,
    sendInlineComments,
  } from "../../fixtures/agent-inline-comment-play.js";
  const { Story } = defineMeta({
    title: "JJ Store/Independent agent replies",
    component: Fixture,
    tags: ["visual-pending"],
    parameters: { layout: "fullscreen" },
  });
</script>

<Story
  name="Offline inline drafts survive failed feedback and Send prepares the session"
  args={{ status: "Offline" }}
  play={async ({ canvasElement }) => {
    const c = await openInlineConversation(canvasElement);
    const editor = await startInlineComment(canvasElement);
    await userEvent.type(editor, "Comment while the agent is stopped");
    await userEvent.click(
      c.getByRole("button", { name: "Fail feedback refresh" }),
    );
    await waitFor(() =>
      expect(
        canvasElement.querySelector('[data-testid="comment-feedback-error"]'),
      ).toHaveTextContent("Feedback refresh failed"),
    );
    await expect(editor).toHaveAttribute("contenteditable", "true");
    await userEvent.click(editor);
    await userEvent.keyboard("{End} and retain it");
    const text = "Comment while the agent is stopped and retain it";
    await expect(editor).toHaveTextContent(text);
    await expect(
      canvasElement.querySelector('[data-location="queue"] [role="textbox"]'),
    ).toHaveTextContent(text);
    await expect(c.getByRole("button", { name: "Send message" })).toBeEnabled();
    expect(commentCommands(canvasElement)).toHaveLength(0);
    expect(promptCommands(canvasElement)).toHaveLength(0);
    await userEvent.click(
      c.getByRole("button", { name: "Refresh captured feedback" }),
    );
    await waitFor(() =>
      expect(
        canvasElement.querySelector('[data-testid="comment-feedback-error"]'),
      ).toHaveTextContent(/^$/),
    );
    await sendInlineComments(canvasElement);
    expect(
      commentCommands(canvasElement).map((command) => command.action.action),
    ).toEqual(["resume", "prompt"]);
    const prompt = promptCommands(canvasElement)[0];
    if (prompt.action.action !== "prompt") throw new Error("Expected prompt");
    expect(prompt.action.comments?.comments[0].body).toBe(text);
    expect(promptCommands(canvasElement)).toHaveLength(1);
  }}
/>
<Story
  name="Multiple comments and exact revision edits share an anchor while the agent works"
  args={{ status: "Working" }}
  play={async ({ canvasElement }) => {
    const c = await openInlineConversation(canvasElement);
    const first = await startInlineComment(canvasElement);
    await userEvent.type(first, "First inline comment");
    await userEvent.click(
      c.getByRole("button", { name: "Add another comment" }),
    );
    const second = await commentEditor(canvasElement);
    await userEvent.type(second, "Second inline comment");
    expect(inlineEditors(canvasElement)).toHaveLength(2);
    expect(promptCommands(canvasElement)).toHaveLength(0);
    await expect(
      c.getByRole("button", { name: "Queue message" }),
    ).toBeEnabled();
    expect(c.queryByRole("button", { name: "Stop agent" })).toBeNull();
    expect(c.queryByRole("button", { name: "Send message" })).toBeNull();
    await userEvent.click(
      c.getByRole("button", { name: "Complete fixture turn" }),
    );
    await waitFor(() =>
      expect(c.getByRole("button", { name: "Send message" })).toBeEnabled(),
    );
    await expect(inlineEditors(canvasElement)[0]).toHaveTextContent(
      "First inline comment",
    );
    await expect(inlineEditors(canvasElement)[1]).toHaveTextContent(
      "Second inline comment",
    );
    await sendInlineComments(canvasElement);
    const initial = promptCommands(canvasElement)[0];
    if (initial.action.action !== "prompt") throw new Error("Expected prompt");
    const drafts = initial.action.comments!.comments;
    expect(drafts.map((d) => d.body)).toEqual([
      "First inline comment",
      "Second inline comment",
    ]);
    expect(drafts[0].thread_id).toBe(drafts[1].thread_id);
    expect(drafts[0].message_id).not.toBe(drafts[1].message_id);
    expect(drafts[0].reply_to).toBeNull();
    await userEvent.click(
      c.getAllByRole("button", { name: "Edit comment" })[0],
    );
    const edited = await commentEditor(canvasElement);
    await expect(edited).toHaveTextContent("First inline comment");
    await userEvent.click(edited);
    await userEvent.keyboard("{Control>}a{/Control}Edited inline comment");
    await sendInlineComments(canvasElement);
    const updated = promptCommands(canvasElement)[1];
    if (updated.action.action !== "prompt") throw new Error("Expected prompt");
    expect(updated.action.comments!.comments[0]).toMatchObject({
      body: "Edited inline comment",
      message_id: drafts[0].message_id,
      expected_revision: { kind: "human_message_revision" },
      source: drafts[0].source,
    });
    expect(
      updated.action.comments!.comments[0].expected_revision,
    ).not.toBeNull();
    await waitFor(() => {
      const area = canvasElement.querySelector('[data-placement="inline"]')!;
      expect(area).toHaveTextContent("Edited inline comment");
      expect(area).toHaveTextContent("Second inline comment");
      expect(area).not.toHaveTextContent("First inline comment");
    });
    expect(promptCommands(canvasElement)).toHaveLength(2);
    await expect(c.getByRole("button", { name: "Send message" })).toBeVisible();
  }}
/>
<Story
  name="Sent context and captured tool output use the current inline comment queue"
  args={{ tools: true }}
  play={async ({ canvasElement }) => {
    const c = await openInlineConversation(canvasElement);
    await expect(
      await c.findByRole("group", { name: /^Session started$/ }),
    ).toBeVisible();
    await expect(
      await c.findByRole("group", { name: /^Turn completed$/ }),
    ).toBeVisible();
    expect(
      [...canvasElement.querySelectorAll("time[datetime]")].some(
        (node) => node.getAttribute("datetime") === "2026-10-06T21:59:59.000Z",
      ),
    ).toBe(true);
    await userEvent.click(
      await c.findByRole("button", { name: "Sent context" }),
    );
    await userEvent.click(
      await c.findByRole("button", { name: "<agent_context>" }),
    );
    await userEvent.click(
      await c.findByRole("button", { name: "<repository_context>" }),
    );
    await expect(c.getByText("Pinned repository facts")).toBeVisible();
    await userEvent.click(c.getByRole("button", { name: "Show raw context" }));
    await expect(
      c.getByRole("region", { name: "Raw context" }),
    ).toHaveTextContent("Pinned repository facts");
    await userEvent.click(
      c.getByRole("button", { name: "Show rendered context" }),
    );
    await expect(c.getByText("Pinned repository facts")).toBeVisible();
    await userEvent.click(
      c.getByRole("button", { name: "Show details for Read captured.ts" }),
    );
    const source = await c.findByRole("textbox", { name: "src/captured.ts" });
    await expect(source).toHaveTextContent("export const captured = 42;");
    await userEvent.click(source);
    await userEvent.click(
      c.getByRole("button", { name: "Comment on src/captured.ts" }),
    );
    const editor = await commentEditor(canvasElement, "src/captured.ts");
    await userEvent.type(editor, "Use this captured tool file");
    await expect(
      canvasElement.querySelector('[data-location="queue"] [role="textbox"]'),
    ).toHaveTextContent("Use this captured tool file");
    await sendInlineComments(canvasElement);
    const prompt = promptCommands(canvasElement)[0];
    if (prompt.action.action !== "prompt") throw new Error("Expected prompt");
    expect(prompt.action.comments?.comments[0]).toMatchObject({
      body: "Use this captured tool file",
      source: {
        part: "/rawOutput/formatted_output",
        quote: "export const captured = 42;",
        target: {
          type: "tool_output",
          tool_id: "77".repeat(16),
          content: { id: "66".repeat(32) },
        },
      },
    });
    expect(promptCommands(canvasElement)).toHaveLength(1);
  }}
/>
<Story
  name="Consecutive tool disclosures retain raw mode and exact pending sources independently"
  args={{ tools: "grouped" }}
  play={async ({ canvasElement }) => {
    const c = await openInlineConversation(canvasElement);
    const read = await c.findByRole("button", {
      name: "Show details for Read captured.ts",
    });
    await expect(
      c.getByRole("button", { name: "Show details for Edited file" }),
    ).toHaveAttribute("aria-expanded", "false");
    read.focus();
    await userEvent.keyboard("{Enter}");
    await userEvent.click(
      c.getByRole("button", {
        name: "Show raw output in read sidebar for Read captured.ts",
      }),
    );
    await c.findByText(/formatted_output/);
    await userEvent.click(
      c.getByRole("button", { name: "Close read tool details" }),
    );
    await userEvent.click(
      c.getByRole("button", { name: "Show details for Edited file" }),
    );
    await expect(
      c.getByRole("button", { name: "Hide details for Edited file" }),
    ).toHaveAttribute("aria-expanded", "true");
    await userEvent.click(
      c.getByRole("button", { name: "Hide details for Edited file" }),
    );
    await userEvent.click(
      c.getByRole("button", { name: "Show details for Read captured.ts" }),
    );
    c.getByRole("button", {
      name: "Show rich output in read sidebar for Read captured.ts",
    }).focus();
    await waitFor(() =>
      expect(
        c.getByRole("button", {
          name: "Show rich output in read sidebar for Read captured.ts",
        }),
      ).toBeVisible(),
    );
    await userEvent.click(
      c.getByRole("button", {
        name: "Show rich output in read sidebar for Read captured.ts",
      }),
    );
    const source = await c.findByRole("textbox", { name: "src/captured.ts" });
    await userEvent.click(source);
    await userEvent.click(
      c.getByRole("button", { name: "Comment on src/captured.ts" }),
    );
    const editor = await commentEditor(canvasElement, "src/captured.ts");
    await userEvent.type(editor, "Comment on the captured read");
    await userEvent.click(
      c.getByRole("button", { name: "Close read tool details" }),
    );
    await expect(
      canvasElement.querySelector('[data-location="queue"] [role="textbox"]'),
    ).toHaveTextContent("Comment on the captured read");
    expect(promptCommands(canvasElement)).toHaveLength(0);
    await userEvent.click(
      c.getByRole("button", { name: "Show details for Read captured.ts" }),
    );
    await expect(
      inlineEditors(canvasElement, "src/captured.ts")[0],
    ).toHaveTextContent("Comment on the captured read");
    await sendInlineComments(canvasElement);
    const prompt = promptCommands(canvasElement)[0];
    if (prompt.action.action !== "prompt") throw new Error("Expected prompt");
    expect(prompt.action.comments!.comments).toHaveLength(1);
    expect(prompt.action.comments!.comments[0].source.target).toMatchObject({
      type: "tool_output",
      tool_id: "77".repeat(16),
    });
    await expect(
      c.getByRole("button", { name: "Show details for Edited file" }),
    ).toHaveAttribute("aria-expanded", "false");
  }}
/>

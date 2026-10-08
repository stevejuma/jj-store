<script module lang="ts">
  import { defineMeta } from "@storybook/addon-svelte-csf";
  import { expect, userEvent, within, waitFor } from "storybook/test";
  import Fixture from "../../fixtures/AgentExecutionFixture.svelte";
  const { Story } = defineMeta({
    title: "JJ Store/Independent agent replies",
    component: Fixture,
    tags: ["visual-pending"],
    parameters: { layout: "fullscreen" },
  });
  async function open(canvasElement: HTMLElement) {
    const c = within(canvasElement);
    await userEvent.click(
      c.getByRole("button", { name: "Open fixture conversation" }),
    );
    const message = (
      await c.findAllByRole("article", {
        name: "Message from Repository helper",
      })
    )[0];
    await userEvent.hover(message);
    await waitFor(() =>
      expect(message).toHaveAttribute("data-actions-visible"),
    );
    await userEvent.click(
      within(message).getByRole("button", { name: "Reply" }),
    );
    return {
      c,
      panel: within(
        await c.findByRole("region", { name: "Replies to Repository helper" }),
      ),
    };
  }
</script>

<Story
  name="Offline replies stay editable and save after feedback refresh fails"
  args={{
    agentReplies: "refresh-failed",
    chatFeedback: "completed",
    status: "Offline",
  }}
  play={async ({ canvasElement }) => {
    const { c, panel } = await open(canvasElement);
    await panel.findByText("Feedback refresh failed");
    const editor = panel.getByRole("textbox", { name: "Markdown editor" });
    await userEvent.type(editor, "Reply while the agent is stopped");
    await expect(
      panel.getByRole("button", { name: "Save reply" }),
    ).toBeEnabled();
    await userEvent.click(panel.getByRole("button", { name: "Save reply" }));
    await expect(editor).not.toHaveTextContent(
      "Reply while the agent is stopped",
    );
    await expect(
      c.queryByLabelText("Session commands"),
    ).not.toBeInTheDocument();
  }}
/>
<Story
  name="Ordinary nested replies and edits stay independent of a working agent"
  args={{ agentReplies: true, chatFeedback: "working", status: "Working" }}
  play={async ({ canvasElement }) => {
    const { c, panel } = await open(canvasElement);
    const editor = panel.getByRole("textbox", { name: "Markdown editor" });
    await userEvent.type(editor, "First discussion message");
    await userEvent.click(panel.getByRole("button", { name: "Save reply" }));
    const message = await panel.findByRole("article", {
      name: "Message from You",
    });
    await userEvent.hover(message);
    await waitFor(() =>
      expect(message).toHaveAttribute("data-actions-visible"),
    );
    await userEvent.click(
      within(message).getByRole("button", { name: "Reply" }),
    );
    await userEvent.type(editor, "Nested discussion message");
    await userEvent.click(panel.getByRole("button", { name: "Save reply" }));
    await panel.findByText("Nested discussion message");
    await expect(
      canvasElement.querySelector('[data-thread-depth="2"]'),
    ).not.toBeNull();
    await userEvent.hover(message);
    await userEvent.click(
      within(message).getByRole("button", { name: "More message actions" }),
    );
    await userEvent.click(
      await within(document.body).findByRole("menuitem", {
        name: "Edit message",
      }),
    );
    await expect(editor).toHaveTextContent("First discussion message");
    await userEvent.clear(editor);
    await userEvent.type(editor, "Edited discussion message");
    await userEvent.click(panel.getByRole("button", { name: "Save changes" }));
    await waitFor(() =>
      expect(
        panel
          .getAllByRole("article")
          .some((article) =>
            article.textContent?.includes("Edited discussion message"),
          ),
      ).toBe(true),
    );
    await expect(
      c.queryByLabelText("Session commands"),
    ).not.toBeInTheDocument();
  }}
/>

<Story
  name="Shared chat rows show timed lifecycle XML context and exact tool replies"
  args={{
    agentReplies: true,
    chatFeedback: "completed",
    chatTools: true,
    status: "Offline",
  }}
  play={async ({ canvasElement }) => {
    const c = within(canvasElement);
    await userEvent.click(
      c.getByRole("button", { name: "Open fixture conversation" }),
    );
    await expect(
      await c.findByRole("group", { name: /^Session started$/ }),
    ).toBeVisible();
    await expect(
      await c.findByRole("group", { name: /^Turn completed$/ }),
    ).toBeVisible();
    const start = c.getByRole("group", {
      name: /^Session started$/,
    });
    const completed = c.getByRole("group", {
      name: /^Turn completed$/,
    });
    await expect(start).toHaveAttribute("title", "Session started");
    await expect(completed).toHaveAttribute("title", "Turn completed");
    expect(start.querySelector("svg")).not.toBeNull();
    expect(completed.querySelector("svg")).not.toBeNull();
    expect(start.textContent).not.toContain("Session started");
    expect(completed.textContent).not.toContain("Turn completed");
    const clocks = [...canvasElement.querySelectorAll("time[datetime]")];
    expect(
      clocks.some(
        (node) => node.getAttribute("datetime") === "2026-10-06T21:59:59.000Z",
      ),
    ).toBe(true);
    await userEvent.click(
      await c.findByText("Prepared context", { exact: true }),
    );
    await userEvent.click(
      await c.findByRole("button", { name: "<agent_context>" }),
    );
    await userEvent.click(
      await c.findByRole("button", { name: "<repository_context>" }),
    );
    await expect(c.getByText("Pinned repository facts")).toBeVisible();
    const tool = (
      await c.findByRole("button", {
        name: "Show details for Read captured.ts",
      })
    ).closest("article")!;
    await expect(tool).toHaveAttribute(
      "data-ui-component",
      "agent-tool-message",
    );
    await expect(
      within(tool).queryByText("Repository helper", { exact: true }),
    ).toBeNull();
    await expect(within(tool).queryByRole("img")).toBeNull();
    await expect(tool).not.toHaveClass("community-message");
    expect(getComputedStyle(tool).borderInlineStartWidth).toBe("0px");
    expect(getComputedStyle(tool).backgroundColor).toBe("rgba(0, 0, 0, 0)");
    const trigger = tool.querySelector('[data-ai-chat-part="call-trigger"]')!;
    const icon = trigger.querySelector('[data-ui-part="call-status"]')!;
    expect(getComputedStyle(tool).paddingInlineStart).toBe("0px");
    expect(
      icon.getBoundingClientRect().left - trigger.getBoundingClientRect().left,
    ).toBe(8);
    await userEvent.hover(trigger);
    await userEvent.hover(tool);
    await waitFor(() => expect(tool).toHaveAttribute("data-actions-visible"));
    await expect(
      within(tool).getByRole("button", { name: "Add reaction" }),
    ).toBeEnabled();
    await userEvent.click(within(tool).getByRole("button", { name: "Reply" }));
    const panel = within(
      await c.findByRole("region", { name: "Replies to Repository helper" }),
    );
    await userEvent.type(
      panel.getByRole("textbox", { name: "Markdown editor" }),
      "Use the captured tool file",
    );
    await userEvent.click(panel.getByRole("button", { name: "Save reply" }));
    await expect(
      await panel.findByText("Use the captured tool file", { exact: true }),
    ).toBeVisible();
    await expect(
      within(tool).getByRole("button", { name: "1 reply" }),
    ).toBeVisible();
  }}
/>

<Story
  name="Consecutive tools expand together and retain exact call actions"
  args={{
    agentReplies: true,
    chatFeedback: "completed",
    chatTools: "grouped",
    status: "Offline",
  }}
  play={async ({ canvasElement }) => {
    const c = within(canvasElement);
    await userEvent.click(
      c.getByRole("button", { name: "Open fixture conversation" }),
    );
    const group = await c.findByRole("button", { name: "Expand 2 tool calls" });
    await expect(group).toHaveAttribute("aria-expanded", "false");
    const root = group.closest('[data-ui-component="ai-chat-tool-calls"]')!;
    const container = root.parentElement!;
    const style = getComputedStyle(container);
    const availableWidth =
      container.clientWidth -
      parseFloat(style.paddingInlineStart) -
      parseFloat(style.paddingInlineEnd);
    expect(root.getBoundingClientRect().width).toBeCloseTo(availableWidth, 0);
    await expect(
      c.queryByRole("button", { name: "Show details for Read captured.ts" }),
    ).not.toBeInTheDocument();
    group.focus();
    await userEvent.keyboard("{Enter}");
    await expect(
      c.getByRole("button", { name: "Collapse 2 tool calls" }),
    ).toHaveAttribute("aria-expanded", "true");
    const read = c.getByRole("button", {
      name: "Show details for Read captured.ts",
    });
    const edit = c.getByRole("button", {
      name: "Show details for Edit captured.ts",
    });
    await expect(read).toBeVisible();
    await expect(edit).toBeVisible();
    const tool = read.closest("article")!;
    await expect(tool).toHaveAttribute(
      "data-message-id",
      `${"55".repeat(16)}/tool/read`,
    );
    await expect(within(tool).queryByRole("img")).toBeNull();
    await expect(tool).not.toHaveClass("community-message");
    expect(getComputedStyle(tool).borderInlineStartWidth).toBe("0px");
    const icon = read.querySelector('[data-ui-part="call-status"]')!;
    expect(
      icon.getBoundingClientRect().left - read.getBoundingClientRect().left,
    ).toBe(8);
    await userEvent.click(read);
    await expect(c.getByRole("group", { name: "typescript" })).toBeVisible();
    await userEvent.click(c.getByRole("button", { name: "Show raw output" }));
    await expect(c.getByText(/formatted_output/)).toBeVisible();
    await userEvent.click(
      c.getByRole("button", { name: "Collapse 2 tool calls" }),
    );
    await userEvent.click(
      c.getByRole("button", { name: "Expand 2 tool calls" }),
    );
    await expect(
      c.getByRole("button", { name: "Show rich output" }),
    ).toBeVisible();
    const current = c
      .getByRole("button", { name: "Hide details for Read captured.ts" })
      .closest("article")!;
    await userEvent.hover(current);
    await waitFor(() =>
      expect(current).toHaveAttribute("data-actions-visible"),
    );
    await expect(
      within(current).getByRole("button", { name: "Add reaction" }),
    ).toBeEnabled();
    await userEvent.click(
      within(current).getByRole("button", { name: "Reply" }),
    );
    const panel = within(
      await c.findByRole("region", { name: "Replies to Repository helper" }),
    );
    await userEvent.type(
      panel.getByRole("textbox", { name: "Markdown editor" }),
      "Reply to the grouped read",
    );
    await userEvent.click(panel.getByRole("button", { name: "Save reply" }));
    await expect(
      await panel.findByText("Reply to the grouped read", { exact: true }),
    ).toBeVisible();
    await expect(
      within(current).getByRole("button", { name: "1 reply" }),
    ).toBeVisible();
    await expect(
      within(
        c
          .getByRole("button", { name: "Show details for Edit captured.ts" })
          .closest("article")!,
      ).queryByRole("button", {
        name: "1 reply",
      }),
    ).toBeNull();
  }}
/>

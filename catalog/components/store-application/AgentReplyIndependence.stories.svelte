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

<script module lang="ts">
  import { defineMeta } from "@storybook/addon-svelte-csf";
  import { expect, userEvent, within, waitFor } from "storybook/test";
  import Fixture from "../../fixtures/AgentExecutionFixture.svelte";
  const { Story } = defineMeta({
    title: "JJ Store/Agent message discussion",
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
  name="Exact reactions share the message toolbar and bar"
  args={{ agentReplies: true, chatFeedback: "completed" }}
  play={async ({ canvasElement }) => {
    const c = within(canvasElement),
      page = within(document.body);
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
      within(message).getByRole("button", { name: "Add reaction" }),
    );
    await userEvent.click(await page.findByRole("button", { name: ":lapis:" }));
    const reaction = await within(message).findByRole("button", {
      name: /1 reaction with Lapis/,
    });
    await userEvent.click(reaction);
    await waitFor(() =>
      expect(
        within(message).queryByRole("button", {
          name: /1 reaction with Lapis/,
        }),
      ).not.toBeInTheDocument(),
    );
    await expect(
      c.queryByLabelText("Session commands"),
    ).not.toBeInTheDocument();
  }}
/>
<Story
  name="Saved replies retain drafts and do not prompt"
  args={{ agentReplies: true, chatFeedback: "completed" }}
  play={async ({ canvasElement }) => {
    const { c, panel } = await open(canvasElement);
    const editor = panel.getByRole("textbox", { name: "Markdown editor" });
    await userEvent.type(editor, "Use the shared form component.");
    await userEvent.click(panel.getByRole("button", { name: "Save reply" }));
    await panel.findByText("Use the shared form component.");
    await expect(editor).not.toHaveTextContent(
      "Use the shared form component.",
    );
    await expect(
      c.queryByLabelText("Session commands"),
    ).not.toBeInTheDocument();
    await userEvent.type(editor, "Retained next reply");
    await userEvent.click(
      c.getByRole("button", { name: "Manage fixture agents" }),
    );
    const reopened = await open(canvasElement);
    await expect(
      reopened.panel.getByRole("textbox", { name: "Markdown editor" }),
    ).toHaveTextContent("Retained next reply");
    await reopened.panel.findByText("Use the shared form component.");
  }}
/>
<Story
  name="Unsupported repository retains reply draft without dispatch"
  args={{
    agentReplies: true,
    feedbackWriteUnavailable: true,
    chatFeedback: "completed",
  }}
  play={async ({ canvasElement }) => {
    const { panel, c } = await open(canvasElement);
    await userEvent.type(
      panel.getByRole("textbox", { name: "Markdown editor" }),
      "Retained feedback",
    );
    await panel.findByText(/until collaboration replication is qualified/);
    await expect(
      panel.getByRole("button", { name: "Save reply" }),
    ).toBeDisabled();
    await expect(
      c.queryByLabelText("Session commands"),
    ).not.toBeInTheDocument();
  }}
/>
<Story
  name="Reply waits for original publication"
  args={{ chatFeedback: "completed" }}
  play={async ({ canvasElement }) => {
    const { panel } = await open(canvasElement);
    await userEvent.type(
      panel.getByRole("textbox", { name: "Markdown editor" }),
      "Keep this reply draft",
    );
    await panel.findByText(/Waiting for this message to be published/);
    await expect(
      panel.getByRole("button", { name: "Save reply" }),
    ).toBeDisabled();
  }}
/>
<Story
  name="Unknown reply keeps exact request for status recovery"
  args={{ agentReplies: true, unknownReply: true, chatFeedback: "completed" }}
  play={async ({ canvasElement }) => {
    const { panel, c } = await open(canvasElement);
    const editor = panel.getByRole("textbox", { name: "Markdown editor" });
    await userEvent.type(editor, "Saved exactly once");
    await userEvent.click(panel.getByRole("button", { name: "Save reply" }));
    await panel.findByRole("button", { name: "Check reply status" });
    await expect(editor).toHaveTextContent("Saved exactly once");
    await userEvent.click(
      panel.getByRole("button", { name: "Check reply status" }),
    );
    await waitFor(() =>
      expect(
        panel.queryByRole("button", { name: "Check reply status" }),
      ).not.toBeInTheDocument(),
    );
    await panel.findByText("Saved exactly once");
    await expect(
      panel.getByRole("textbox", { name: "Markdown editor" }),
    ).not.toHaveTextContent("Saved exactly once");
    await expect(panel.queryByRole("alert")).not.toBeInTheDocument();
    await expect(
      c.queryByLabelText("Session commands"),
    ).not.toBeInTheDocument();
  }}
/>

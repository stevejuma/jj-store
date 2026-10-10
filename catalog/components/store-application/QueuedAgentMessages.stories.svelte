<script module lang="ts">
  import { defineMeta } from "@storybook/addon-svelte-csf";
  import { expect, userEvent, within, waitFor } from "storybook/test";
  import {
    preview,
    longPreview,
  } from "../../fixtures/queued-message-checks.js";
  import Fixture from "../../fixtures/QueuedAgentMessagesFixture.svelte";
  const { Story } = defineMeta({
    title: "JJ Store/Queued agent messages",
    component: Fixture,
    tags: ["visual-pending"],
    parameters: { layout: "fullscreen", a11y: { test: "error" } },
  });
  const input = (c: ReturnType<typeof within>) => {
    const document = c.queryByRole("group", { name: "Next message draft" });
    return document
      ? within(document).getByRole("textbox")
      : c.getByRole("textbox", { name: "Markdown editor" });
  };
  const commands = (c: ReturnType<typeof within>) =>
    JSON.parse(
      c.getByLabelText("Submitted queue commands").textContent ?? "[]",
    );
  async function queue(canvasElement: HTMLElement) {
    const c = within(canvasElement);
    await c.findByText("Ready to help.");
    const draft = input(c);
    expect(draft).toHaveAttribute("aria-disabled", "false");
    expect(c.getByRole("button", { name: "Stop agent" })).toBeEnabled();
    await userEvent.click(draft);
    await userEvent.keyboard("   ");
    expect(c.getByRole("button", { name: "Stop agent" })).toBeVisible();
    await userEvent.keyboard("{Control>}a{/Control}{Backspace}First follow-up");
    const send = c.getByRole("button", { name: "Queue message" });
    expect(send).toHaveAttribute("data-variant", "default");
    expect(send.querySelector("svg")).toHaveClass("lucide-arrow-up");
    await userEvent.click(send);
    const pending = await c.findByRole("region", { name: "Queued messages" });
    expect(pending).toHaveTextContent("First follow-up");
    expect(draft).toHaveTextContent("");
    await userEvent.click(draft);
    await userEvent.keyboard("Second follow-up");
    await userEvent.keyboard("{Control>}{Enter}{/Control}");
    await waitFor(() => expect(pending).toHaveTextContent("2 queued messages"));
    expect(
      c.getByRole("log", { name: "Conversation messages" }),
    ).not.toHaveTextContent("First follow-up");
    await userEvent.click(c.getByRole("button", { name: "Switch view" }));
    expect(
      c.getByRole("region", { name: "Queued messages" }),
    ).toHaveTextContent("Second follow-up");
    await userEvent.click(input(c));
    await userEvent.keyboard("Keep this next draft");
    await userEvent.click(c.getByRole("button", { name: "Toggle chat" }));
    await userEvent.click(c.getByRole("button", { name: "Toggle chat" }));
    expect(input(c)).toHaveTextContent("Keep this next draft");
    const cancels = c.getAllByRole("button", {
      name: "Cancel queued message for Repository helper",
    });
    await userEvent.click(cancels[1]);
    await waitFor(() =>
      expect(
        c.getByRole("region", { name: "Queued messages" }),
      ).not.toHaveTextContent("Second follow-up"),
    );
    expect(input(c)).toHaveTextContent("Keep this next draft");
    await userEvent.click(
      c.getByRole("button", { name: "Complete active turn" }),
    );
    await waitFor(() =>
      expect(c.queryByRole("region", { name: "Queued messages" })).toBeNull(),
    );
    expect(
      c.getByRole("log", { name: "Conversation messages" }),
    ).toHaveTextContent("First follow-up");
    expect(
      c.getByRole("log", { name: "Conversation messages" }),
    ).not.toHaveTextContent("Second follow-up");
    expect(
      commands(c).filter((v: any) => v.action.action === "prompt"),
    ).toHaveLength(2);
    const cancel = commands(c).find((v: any) => v.action.action === "cancel");
    expect(cancel.action.target).toBe(commands(c)[1].request_id);
  }
  async function review(canvasElement: HTMLElement) {
    const c = within(canvasElement);
    await c.findByText("Ready to help.");
    await userEvent.click(
      c.getByRole("button", { name: "Add review comment" }),
    );
    await c.findByRole("group", { name: "Pending comments" });
    await userEvent.click(c.getByRole("button", { name: "Queue message" }));
    const pending = await c.findByRole("region", { name: "Queued messages" });
    expect(pending).toHaveTextContent("Review this output next");
    expect(c.queryByRole("group", { name: "Pending comments" })).toBeNull();
    expect(commands(c)[0].action.text).toBe("");
    expect(commands(c)[0].action.comments.comments[0].body).toBe(
      "Review this output next",
    );
  }
</script>

<Story
  name="Document queued message preview"
  args={{ view: "document" }}
  play={async ({ canvasElement }) => preview(canvasElement)}
/>
<Story
  name="Classic queued message preview"
  args={{ view: "classic" }}
  play={async ({ canvasElement }) => preview(canvasElement)}
/>
<Story
  name="Document queues cancels and advances messages"
  play={async ({ canvasElement }) => queue(canvasElement)}
/>
<Story
  name="Classic queues cancels and advances messages"
  args={{ view: "classic" }}
  play={async ({ canvasElement }) => queue(canvasElement)}
/>
<Story
  name="Document queues comments without message text"
  play={async ({ canvasElement }) => review(canvasElement)}
/>
<Story
  name="Classic queues comments without message text"
  args={{ view: "classic" }}
  play={async ({ canvasElement }) => review(canvasElement)}
/>
<Story
  name="Delayed acknowledgement releases the input without replay"
  args={{ delayed: true }}
  play={async ({ canvasElement }) => {
    const c = within(canvasElement);
    await c.findByText("Ready to help.");
    await userEvent.click(input(c));
    await userEvent.keyboard("Wait for acknowledgement");
    await userEvent.click(c.getByRole("button", { name: "Queue message" }));
    await c.findByRole("button", { name: "Check retained request" });
    expect(c.getByRole("button", { name: "Stop agent" })).toBeEnabled();
    expect(c.getByRole("button", { name: "Queue message" })).toBeDisabled();
    expect(input(c)).toHaveTextContent("Wait for acknowledgement");
    await userEvent.click(c.getByRole("button", { name: "Acknowledge queue" }));
    await waitFor(() => expect(input(c)).toHaveTextContent(""));
    await userEvent.click(input(c));
    await userEvent.keyboard("Next draft is editable");
    expect(commands(c)).toHaveLength(1);
  }}
/>
<Story
  name="Refusal retains an editable draft"
  play={async ({ canvasElement }) => {
    const c = within(canvasElement);
    await c.findByText("Ready to help.");
    await userEvent.click(c.getByRole("button", { name: "Reject next send" }));
    await userEvent.click(input(c));
    await userEvent.keyboard("Keep the refused draft");
    await userEvent.click(c.getByRole("button", { name: "Queue message" }));
    await waitFor(() =>
      expect(input(c)).toHaveAttribute("aria-disabled", "false"),
    );
    expect(input(c)).toHaveTextContent("Keep the refused draft");
    expect(c.queryByRole("region", { name: "Queued messages" })).toBeNull();
  }}
/>

<Story
  name="Document long queued prompt preview"
  args={{ view: "document" }}
  play={async ({ canvasElement }) => longPreview(canvasElement)}
/>
<Story
  name="Classic long queued prompt preview"
  args={{ view: "classic" }}
  play={async ({ canvasElement }) => longPreview(canvasElement)}
/>

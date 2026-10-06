<script module lang="ts">
  import { defineMeta } from "@storybook/addon-svelte-csf";
  import { expect, userEvent, within, waitFor } from "storybook/test";
  import { agentMessageTimestamp } from "../../../web/src/lib/collaboration/presentation.js";
  import Fixture from "../../fixtures/AgentExecutionFixture.svelte";
  const { Story } = defineMeta({
    title: "JJ Store/Agent chat feedback",
    component: Fixture,
    tags: ["visual-pending"],
    parameters: { layout: "fullscreen" },
  });
</script>

<Story
  name="Turn completion uses the shared system message"
  args={{ chatFeedback: "completed" }}
  play={async ({ canvasElement }) => {
    const c = within(canvasElement);
    await userEvent.click(
      c.getByRole("button", { name: "Open fixture conversation" }),
    );
    const completed = await c.findByText("Turn completed");
    await expect(
      completed.closest('[data-ui-component="ai-chat-system-message"]'),
    ).not.toBeNull();
    await expect(completed.closest('[role="log"]')).not.toBeNull();
    await expect(
      c.queryByText("Working", { exact: true }),
    ).not.toBeInTheDocument();
  }}
/>

<Story
  name="Running session mirrors Community working in the body and composer activity strip"
  args={{ status: "Working", chatFeedback: "working" }}
  play={async ({ canvasElement }) => {
    const c = within(canvasElement);
    await userEvent.click(
      c.getByRole("button", { name: "Open fixture conversation" }),
    );
    const working = await c.findAllByText("Working", { exact: true });
    const body = working.find((element) => element.closest('[role="log"]'));
    await expect(body).toBeDefined();
    await expect(
      body?.closest('[data-ui-component="ai-chat-reasoning"]'),
    ).toHaveAttribute("data-streaming", "true");
    const strip = canvasElement.querySelector<HTMLElement>(
      '[data-ui-component="community-agent-composer-activity"]',
    )!;
    await expect(strip).toBeVisible();
    await expect(
      strip.querySelector(".community-member-avatar"),
    ).not.toBeNull();
    await expect(within(strip).getByRole("status")).toHaveTextContent(
      "Working",
    );
    await userEvent.click(
      c.getByRole("button", { name: "Manage fixture agents" }),
    );
    await expect(
      canvasElement.querySelector(
        '[data-ui-component="community-agent-composer-activity"]',
      ),
    ).toBeNull();
  }}
/>

<Story
  name="Uncertain request uses the Community activity strip and retains exact recovery"
  args={{ fail: true }}
  play={async ({ canvasElement }) => {
    const c = within(canvasElement);
    await userEvent.click(
      c.getByRole("button", { name: "Open fixture conversation" }),
    );
    await userEvent.type(
      await c.findByRole("textbox", { name: "Markdown editor" }),
      "Retain uncertain message",
    );
    await userEvent.click(c.getByRole("button", { name: "Send" }));
    const strip = await waitFor(() => {
      const node = canvasElement.querySelector<HTMLElement>(
        '[data-ui-component="community-agent-composer-activity"]',
      );
      expect(node).not.toBeNull();
      return node!;
    });
    await expect(within(strip).getByRole("status")).toHaveTextContent(
      "Outcome unknown",
    );
    await expect(
      c.getByRole("button", {
        name: "Keep unresolved request and review workspace",
      }),
    ).toBeVisible();
    await expect(
      c.getByRole("textbox", { name: "Markdown editor" }),
    ).toHaveTextContent("Retain uncertain message");
    await expect(
      c.queryByText(/^Request .*Draft and request identity retained\./),
    ).not.toBeInTheDocument();
  }}
/>

<Story
  name="Messages use shared timestamp metadata and retain unknown legacy times"
  args={{ chatFeedback: "completed" }}
  play={async ({ canvasElement }) => {
    const c = within(canvasElement);
    await userEvent.click(
      c.getByRole("button", { name: "Open fixture conversation" }),
    );
    const user = await c.findByRole("article", { name: "Message from You" });
    const assistants = c.getAllByRole("article", {
      name: "Message from Repository helper",
    });
    const assistant = assistants.find((row) =>
      row.textContent?.includes("Ready to help."),
    )!;
    for (const [message, millis] of [
      [user, 1791324000000],
      [assistant, 1791324001000],
    ] as const) {
      const time = message.querySelector("time");
      await expect(time).not.toBeNull();
      await expect(time).toHaveAttribute(
        "datetime",
        new Date(millis).toISOString(),
      );
      await expect(
        time?.closest('[data-ui-component="ai-chat-message-metadata"]'),
      ).not.toBeNull();
      await expect(time?.getAttribute("title")).toBeTruthy();
      await expect(time).toHaveTextContent(
        agentMessageTimestamp(millis).timestamp!,
      );
    }
    const legacy = assistants.find((row) =>
      row.textContent?.includes("Legacy message without timing."),
    )!;
    await expect(legacy.querySelector("time")).toBeNull();
    await expect(
      within(legacy).getByText("Time unknown", { exact: true }),
    ).toBeVisible();
  }}
/>
